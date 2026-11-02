//! Indexed unions of inclusive calendar-date intervals.

use chrono::NaiveDate;

/// Immutable normalized date interval set with logarithmic point lookups.
///
/// Construction sorts and merges unsorted, overlapping, and adjacent ranges.
/// Reversed ranges are rejected. Repeated queries never rescan the input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateIntervalSet {
    intervals: Vec<(NaiveDate, NaiveDate)>,
}

impl DateIntervalSet {
    /// Build an indexed set from arbitrary inclusive ranges.
    pub fn new(intervals: &[(NaiveDate, NaiveDate)]) -> Option<Self> {
        Some(Self {
            intervals: crate::merge_intervals(intervals)?,
        })
    }

    /// Return sorted, disjoint, non-adjacent inclusive ranges.
    pub fn intervals(&self) -> &[(NaiveDate, NaiveDate)] {
        &self.intervals
    }

    /// Find the normalized interval containing a date in O(log n).
    pub fn containing_interval(&self, date: NaiveDate) -> Option<(NaiveDate, NaiveDate)> {
        let index = self.intervals.partition_point(|(_, end)| *end < date);
        self.intervals
            .get(index)
            .copied()
            .filter(|(start, _)| *start <= date)
    }

    /// Check if at least one source range contains the date.
    pub fn contains(&self, date: NaiveDate) -> bool {
        self.containing_interval(date).is_some()
    }

    /// Test overlap against a query range in O(log n).
    ///
    /// Returns None for a reversed query.
    pub fn overlaps(&self, start: NaiveDate, end: NaiveDate) -> Option<bool> {
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

    /// Find the first covered date at or after the input.
    pub fn next_covered_date(&self, date: NaiveDate) -> Option<NaiveDate> {
        let index = self.intervals.partition_point(|(_, end)| *end < date);
        self.intervals
            .get(index)
            .map(|(start, _)| (*start).max(date))
    }

    /// Find the first uncovered date at or after the input.
    ///
    /// Returns None when the containing range ends at NaiveDate::MAX.
    pub fn next_uncovered_date(&self, date: NaiveDate) -> Option<NaiveDate> {
        match self.containing_interval(date) {
            Some((_, end)) => end.succ_opt(),
            None => Some(date),
        }
    }

    /// Return the total number of distinct covered calendar dates.
    pub fn covered_days(&self) -> i64 {
        self.intervals
            .iter()
            .map(|(start, end)| end.signed_duration_since(*start).num_days() + 1)
            .sum()
    }

    /// Clip the union to an inclusive query without enumerating dates.
    ///
    /// Returns None for a reversed query. The output is already normalized.
    pub fn clipped(&self, start: NaiveDate, end: NaiveDate) -> Option<Vec<(NaiveDate, NaiveDate)>> {
        if start > end {
            return None;
        }
        let first = self.intervals.partition_point(|(_, last)| *last < start);
        Some(
            self.intervals[first..]
                .iter()
                .take_while(|(begin, _)| *begin <= end)
                .map(|(begin, last)| ((*begin).max(start), (*last).min(end)))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn builds_sorted_merged_set_and_queries_binary_boundaries() {
        let set = DateIntervalSet::new(&[
            (date(6), date(7)),
            (date(1), date(3)),
            (date(3), date(5)),
            (date(10), date(12)),
        ])
        .unwrap();
        assert_eq!(set.intervals(), &[(date(1), date(7)), (date(10), date(12))]);
        assert_eq!(set.covered_days(), 10);
        assert_eq!(set.containing_interval(date(4)), Some((date(1), date(7))));
        assert_eq!(set.containing_interval(date(8)), None);
        assert_eq!(set.next_covered_date(date(8)), Some(date(10)));
        assert_eq!(set.next_covered_date(date(12)), Some(date(12)));
        assert_eq!(set.next_covered_date(date(13)), None);
        assert_eq!(set.next_uncovered_date(date(4)), Some(date(8)));
        assert_eq!(set.next_uncovered_date(date(8)), Some(date(8)));
        assert_eq!(set.overlaps(date(7), date(10)), Some(true));
        assert_eq!(set.overlaps(date(8), date(9)), Some(false));
        assert_eq!(set.overlaps(date(9), date(8)), None);
        assert_eq!(
            set.clipped(date(5), date(11)),
            Some(vec![(date(5), date(7)), (date(10), date(11))])
        );
    }

    #[test]
    fn handles_empty_invalid_and_chrono_limit_ranges() {
        let empty = DateIntervalSet::new(&[]).unwrap();
        assert!(!empty.contains(NaiveDate::MIN));
        assert_eq!(empty.covered_days(), 0);
        assert_eq!(empty.next_covered_date(NaiveDate::MAX), None);
        assert_eq!(
            empty.next_uncovered_date(NaiveDate::MAX),
            Some(NaiveDate::MAX)
        );
        assert_eq!(empty.clipped(NaiveDate::MIN, NaiveDate::MAX), Some(vec![]));
        assert!(DateIntervalSet::new(&[(date(3), date(1))]).is_none());
        let full = DateIntervalSet::new(&[(NaiveDate::MIN, NaiveDate::MAX)]).unwrap();
        assert_eq!(full.next_uncovered_date(NaiveDate::MIN), None);
        assert_eq!(full.next_covered_date(NaiveDate::MAX), Some(NaiveDate::MAX));
        assert_eq!(
            full.clipped(NaiveDate::MAX, NaiveDate::MAX),
            Some(vec![(NaiveDate::MAX, NaiveDate::MAX)])
        );
        assert!(full.covered_days() > 0);
    }

    #[test]
    fn exhaustive_small_domain_matches_direct_membership() {
        let candidates = [
            (date(1), date(2)),
            (date(2), date(4)),
            (date(6), date(7)),
            (date(7), date(9)),
        ];
        for mask in 0..16_u32 {
            let ranges: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, range)| *range)
                .collect();
            let set = DateIntervalSet::new(&ranges).unwrap();
            let mut expected_count = 0;
            for day in 1..=10 {
                let point = date(day);
                let expected = ranges.iter().any(|(a, b)| *a <= point && point <= *b);
                assert_eq!(set.contains(point), expected, "{mask}, {day}");
                expected_count += i64::from(expected);
            }
            assert_eq!(set.covered_days(), expected_count);
        }
    }
}
