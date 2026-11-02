//! Indexed inclusive datetime intervals at nanosecond resolution.

use chrono::{NaiveDateTime, TimeDelta};

/// Normalized union of naive datetime intervals for repeated O(log n) queries.
///
/// Ranges are inclusive, sorted, and coalesced when they overlap or share
/// adjacent nanosecond ticks. No time zone is assumed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatetimeIntervalSet {
    intervals: Vec<(NaiveDateTime, NaiveDateTime)>,
}

impl DatetimeIntervalSet {
    /// Normalize input ranges, returning None if any input is reversed.
    pub fn new(intervals: &[(NaiveDateTime, NaiveDateTime)]) -> Option<Self> {
        Some(Self {
            intervals: crate::merge_datetime_intervals(intervals)?,
        })
    }

    /// Return the ordered, nonoverlapping inclusive intervals.
    pub fn intervals(&self) -> &[(NaiveDateTime, NaiveDateTime)] {
        &self.intervals
    }

    /// Locate an interval containing the value with a binary search.
    pub fn containing_interval(
        &self,
        value: NaiveDateTime,
    ) -> Option<(NaiveDateTime, NaiveDateTime)> {
        let index = self.intervals.partition_point(|(_, end)| *end < value);
        self.intervals
            .get(index)
            .copied()
            .filter(|(start, _)| *start <= value)
    }

    /// Test exact nanosecond-precision membership.
    pub fn contains(&self, value: NaiveDateTime) -> bool {
        self.containing_interval(value).is_some()
    }

    /// Test overlap with an inclusive query range in O(log n).
    ///
    /// Returns None if the query endpoints are reversed.
    pub fn overlaps(&self, start: NaiveDateTime, end: NaiveDateTime) -> Option<bool> {
        if start > end {
            return None;
        }
        let index = self.intervals.partition_point(|(_, last)| *last < start);
        Some(
            self.intervals
                .get(index)
                .is_some_and(|(first, _)| *first <= end),
        )
    }

    /// Return the first covered timestamp at or after the input.
    pub fn next_covered_datetime(&self, value: NaiveDateTime) -> Option<NaiveDateTime> {
        let index = self.intervals.partition_point(|(_, end)| *end < value);
        self.intervals
            .get(index)
            .map(|(start, _)| (*start).max(value))
    }

    /// Return the first uncovered timestamp at or after the input.
    ///
    /// The maximum representable timestamp has no successor.
    pub fn next_uncovered_datetime(&self, value: NaiveDateTime) -> Option<NaiveDateTime> {
        match self.containing_interval(value) {
            Some((_, end)) => end.checked_add_signed(TimeDelta::nanoseconds(1)),
            None => Some(value),
        }
    }

    /// Count the number of distinct nanosecond ticks in the union.
    ///
    /// i128 avoids overflow even for the full supported Chrono range.
    pub fn covered_nanoseconds(&self) -> i128 {
        self.intervals
            .iter()
            .map(|(start, end)| {
                let first = start.and_utc();
                let last = end.and_utc();
                i128::from(last.timestamp() - first.timestamp()) * 1_000_000_000
                    + i128::from(last.timestamp_subsec_nanos())
                    - i128::from(first.timestamp_subsec_nanos())
                    + 1
            })
            .sum()
    }

    /// Clip all covered intervals to the supplied inclusive range.
    ///
    /// Returns None for invalid query endpoints.
    pub fn clipped(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
        if start > end {
            return None;
        }
        let index = self.intervals.partition_point(|(_, last)| *last < start);
        Some(
            self.intervals[index..]
                .iter()
                .take_while(|(first, _)| *first <= end)
                .map(|(first, last)| ((*first).max(start), (*last).min(end)))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(nanos: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, nanos)
            .unwrap()
    }

    #[test]
    fn supports_exact_membership_and_navigation() {
        let set = DatetimeIntervalSet::new(&[
            (at(8), at(9)),
            (at(1), at(3)),
            (at(4), at(5)),
            (at(3), at(4)),
        ])
        .unwrap();
        assert_eq!(set.intervals(), &[(at(1), at(5)), (at(8), at(9))]);
        assert_eq!(set.covered_nanoseconds(), 7);
        assert_eq!(set.containing_interval(at(3)), Some((at(1), at(5))));
        assert_eq!(set.containing_interval(at(6)), None);
        assert_eq!(set.next_uncovered_datetime(at(3)), Some(at(6)));
        assert_eq!(set.next_uncovered_datetime(at(6)), Some(at(6)));
        assert_eq!(set.next_covered_datetime(at(7)), Some(at(8)));
        assert_eq!(set.next_covered_datetime(at(10)), None);
        assert_eq!(set.overlaps(at(5), at(8)), Some(true));
        assert_eq!(set.overlaps(at(6), at(7)), Some(false));
        assert_eq!(set.overlaps(at(7), at(6)), None);
        assert_eq!(
            set.clipped(at(4), at(8)),
            Some(vec![(at(4), at(5)), (at(8), at(8))])
        );
    }

    #[test]
    fn handles_empty_invalid_and_full_range() {
        let empty = DatetimeIntervalSet::new(&[]).unwrap();
        assert!(!empty.contains(NaiveDateTime::MIN));
        assert_eq!(empty.covered_nanoseconds(), 0);
        assert!(DatetimeIntervalSet::new(&[(at(5), at(4))]).is_none());
        let full = DatetimeIntervalSet::new(&[(NaiveDateTime::MIN, NaiveDateTime::MAX)]).unwrap();
        assert_eq!(full.next_uncovered_datetime(NaiveDateTime::MAX), None);
        assert_eq!(
            full.clipped(NaiveDateTime::MAX, NaiveDateTime::MAX),
            Some(vec![(NaiveDateTime::MAX, NaiveDateTime::MAX)])
        );
        assert!(full.covered_nanoseconds() > i128::from(i64::MAX));
    }

    #[test]
    fn exhaustive_small_domain_matches_reference_membership() {
        let candidates = [
            (at(0), at(2)),
            (at(2), at(4)),
            (at(6), at(7)),
            (at(8), at(9)),
        ];
        for mask in 0..16_u32 {
            let ranges: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, range)| *range)
                .collect();
            let set = DatetimeIntervalSet::new(&ranges).unwrap();
            let mut expected_count = 0_i128;
            for tick in 0..=10 {
                let point = at(tick);
                let expected = ranges.iter().any(|(a, b)| *a <= point && point <= *b);
                assert_eq!(set.contains(point), expected, "{mask}, {tick}");
                if expected {
                    expected_count += 1;
                }
            }
            assert_eq!(set.covered_nanoseconds(), expected_count);
        }
    }
}
