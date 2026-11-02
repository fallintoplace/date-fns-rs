//! Rank and select nanosecond ticks in a union of datetime intervals.

use chrono::{DateTime, NaiveDateTime, Utc};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

fn to_nanos(value: NaiveDateTime) -> Option<i128> {
    let utc = value.and_utc();
    let nanos = utc.timestamp_subsec_nanos();
    if nanos >= 1_000_000_000 {
        return None;
    }
    Some(i128::from(utc.timestamp()) * NANOS_PER_SECOND + i128::from(nanos))
}

fn from_nanos(value: i128) -> Option<NaiveDateTime> {
    let seconds = i64::try_from(value.div_euclid(NANOS_PER_SECOND)).ok()?;
    let nanos = u32::try_from(value.rem_euclid(NANOS_PER_SECOND)).ok()?;
    DateTime::<Utc>::from_timestamp(seconds, nanos).map(|datetime| datetime.naive_utc())
}

/// Index sorted, normalized inclusive datetime ranges without enumerating ticks.
///
/// Construction takes O(n log n). Rank, select, and count-before take O(log n).
/// All counts use i128 to cover the full supported Chrono datetime range.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatetimeCoverageIndex {
    intervals: Vec<(NaiveDateTime, NaiveDateTime)>,
    cumulative: Vec<i128>,
}

impl DatetimeCoverageIndex {
    /// Build a rank/select index, rejecting reversed or leap-second intervals.
    pub fn new(input: &[(NaiveDateTime, NaiveDateTime)]) -> Option<Self> {
        let intervals = crate::merge_datetime_intervals(input)?;
        let mut cumulative = Vec::with_capacity(intervals.len());
        let mut total = 0_i128;
        for &(start, end) in &intervals {
            let length = to_nanos(end)?
                .checked_sub(to_nanos(start)?)?
                .checked_add(1)?;
            total = total.checked_add(length)?;
            cumulative.push(total);
        }
        Some(Self {
            intervals,
            cumulative,
        })
    }

    /// Return the canonical, disjoint inclusive datetime ranges.
    pub fn intervals(&self) -> &[(NaiveDateTime, NaiveDateTime)] {
        &self.intervals
    }

    /// Number of covered nanosecond ticks.
    pub fn len(&self) -> i128 {
        self.cumulative.last().copied().unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Return the zero-based nth covered timestamp in O(log n).
    pub fn select(&self, index: i128) -> Option<NaiveDateTime> {
        if index < 0 {
            return None;
        }
        let pos = self.cumulative.partition_point(|&end| end <= index);
        let &(start, _) = self.intervals.get(pos)?;
        let before = if pos == 0 {
            0
        } else {
            self.cumulative[pos - 1]
        };
        from_nanos(to_nanos(start)?.checked_add(index - before)?)
    }

    /// Return the zero-based ordinal of a covered timestamp, or None in a gap.
    pub fn rank(&self, value: NaiveDateTime) -> Option<i128> {
        let pos = self.intervals.partition_point(|&(start, _)| start <= value);
        let index = pos.checked_sub(1)?;
        let (start, end) = self.intervals[index];
        if value > end {
            return None;
        }
        let before = if index == 0 {
            0
        } else {
            self.cumulative[index - 1]
        };
        before.checked_add(to_nanos(value)?.checked_sub(to_nanos(start)?)?)
    }

    /// Count covered nanosecond ticks strictly before the query timestamp.
    pub fn count_before(&self, value: NaiveDateTime) -> Option<i128> {
        let point = to_nanos(value)?;
        let completed = self.intervals.partition_point(|&(_, end)| end < value);
        let before = if completed == 0 {
            0
        } else {
            self.cumulative[completed - 1]
        };
        match self.intervals.get(completed) {
            Some(&(start, _)) if start < value => {
                before.checked_add(point.checked_sub(to_nanos(start)?)?)
            }
            _ => Some(before),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(nanos: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, nanos)
            .unwrap()
    }

    #[test]
    fn supports_rank_select_and_gaps_without_enumerating_ticks() {
        let index =
            DatetimeCoverageIndex::new(&[(at(8), at(10)), (at(1), at(2)), (at(2), at(4))]).unwrap();
        assert_eq!(index.len(), 7);
        assert_eq!(index.select(-1), None);
        assert_eq!(index.select(0), Some(at(1)));
        assert_eq!(index.select(3), Some(at(4)));
        assert_eq!(index.select(4), Some(at(8)));
        assert_eq!(index.select(6), Some(at(10)));
        assert_eq!(index.select(7), None);
        assert_eq!(index.rank(at(8)), Some(4));
        assert_eq!(index.rank(at(5)), None);
        assert_eq!(index.count_before(at(3)), Some(2));
        assert_eq!(index.count_before(at(7)), Some(4));
        assert_eq!(index.count_before(at(11)), Some(7));
    }

    #[test]
    fn agrees_with_brute_force_for_small_interval_sets() {
        let candidates = [(at(0), at(2)), (at(3), at(5)), (at(5), at(7))];
        for mask in 0..8_u32 {
            let intervals: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, interval)| *interval)
                .collect();
            let index = DatetimeCoverageIndex::new(&intervals).unwrap();
            let expected: Vec<_> = (0..=9)
                .map(at)
                .filter(|value| intervals.iter().any(|(a, b)| a <= value && value <= b))
                .collect();
            assert_eq!(index.len(), expected.len() as i128);
            for (i, &value) in expected.iter().enumerate() {
                assert_eq!(index.select(i as i128), Some(value));
                assert_eq!(index.rank(value), Some(i as i128));
            }
            for n in 0..=9 {
                assert_eq!(
                    index.count_before(at(n)),
                    Some(expected.iter().filter(|&&value| value < at(n)).count() as i128)
                );
            }
        }
    }

    #[test]
    fn handles_empty_invalid_and_maximum_datetime_ranges() {
        let empty = DatetimeCoverageIndex::new(&[]).unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.select(0), None);
        assert_eq!(empty.count_before(NaiveDateTime::MIN), Some(0));
        assert!(DatetimeCoverageIndex::new(&[(at(5), at(4))]).is_none());
        let full = DatetimeCoverageIndex::new(&[(NaiveDateTime::MIN, NaiveDateTime::MAX)]).unwrap();
        assert!(full.len() > i128::from(i64::MAX));
        assert_eq!(full.select(0), Some(NaiveDateTime::MIN));
        assert_eq!(full.select(full.len() - 1), Some(NaiveDateTime::MAX));
        assert_eq!(full.rank(NaiveDateTime::MAX), Some(full.len() - 1));
    }
}
