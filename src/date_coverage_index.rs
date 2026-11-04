//! Rank and select dates from a normalized union of inclusive date intervals.

use chrono::{Days, NaiveDate};

/// A sorted, disjoint interval index with logarithmic rank and select.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateCoverageIndex {
    intervals: Vec<(NaiveDate, NaiveDate)>,
    cumulative_days: Vec<u64>,
}

impl DateCoverageIndex {
    /// Build an index from unsorted or overlapping inclusive date intervals.
    ///
    /// Returns None for any reversed interval. Adjacent ranges are merged.
    /// Construction takes O(n log n); queries take O(log n).
    pub fn new(input: &[(NaiveDate, NaiveDate)]) -> Option<Self> {
        let intervals = crate::merge_intervals(input)?;
        let mut cumulative_days = Vec::with_capacity(intervals.len());
        let mut total = 0_u64;
        for &(start, end) in &intervals {
            let length = u64::try_from(end.signed_duration_since(start).num_days())
                .ok()?
                .checked_add(1)?;
            total = total.checked_add(length)?;
            cumulative_days.push(total);
        }
        Some(Self {
            intervals,
            cumulative_days,
        })
    }

    /// Return the canonical, non-adjacent inclusive ranges.
    pub fn intervals(&self) -> &[(NaiveDate, NaiveDate)] {
        &self.intervals
    }

    /// Return the number of distinct covered calendar dates.
    pub fn len(&self) -> u64 {
        self.cumulative_days.last().copied().unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Select the zero-based nth covered date without enumerating earlier days.
    pub fn select(&self, index: u64) -> Option<NaiveDate> {
        let pos = self.cumulative_days.partition_point(|&end| end <= index);
        let &(start, _) = self.intervals.get(pos)?;
        let before = if pos == 0 {
            0
        } else {
            self.cumulative_days[pos - 1]
        };
        start.checked_add_days(Days::new(index - before))
    }

    /// Return the zero-based index of a covered date, or None for a gap.
    pub fn rank(&self, date: NaiveDate) -> Option<u64> {
        let pos = self.intervals.partition_point(|&(start, _)| start <= date);
        let index = pos.checked_sub(1)?;
        let (start, end) = self.intervals[index];
        if date > end {
            return None;
        }
        let before = if index == 0 {
            0
        } else {
            self.cumulative_days[index - 1]
        };
        Some(before + u64::try_from(date.signed_duration_since(start).num_days()).ok()?)
    }

    /// Find the earliest covered date on or after the query in O(log n).
    ///
    /// Returns None when no covered date remains.
    pub fn first_on_or_after(&self, date: NaiveDate) -> Option<NaiveDate> {
        let pos = self.intervals.partition_point(|&(_, end)| end < date);
        let &(start, _) = self.intervals.get(pos)?;
        Some(start.max(date))
    }

    /// Find the latest covered date on or before the query in O(log n).
    ///
    /// Returns None when all covered dates are later than the query.
    pub fn last_on_or_before(&self, date: NaiveDate) -> Option<NaiveDate> {
        let pos = self.intervals.partition_point(|&(start, _)| start <= date);
        let &(_, end) = self.intervals.get(pos.checked_sub(1)?)?;
        Some(end.min(date))
    }

    /// Count covered dates strictly before a given date.
    ///
    /// This works for gaps as well as dates inside covered intervals.
    pub fn count_before(&self, date: NaiveDate) -> u64 {
        let completed = self.intervals.partition_point(|&(_, end)| end < date);
        let mut count = if completed == 0 {
            0
        } else {
            self.cumulative_days[completed - 1]
        };
        if let Some(&(start, _)) = self.intervals.get(completed) {
            if start < date {
                count += date.signed_duration_since(start).num_days() as u64;
            }
        }
        count
    }
    /// Count covered calendar dates in an inclusive query range in O(log n).
    ///
    /// Invalid query bounds return None.
    pub fn count_inclusive(&self, start: NaiveDate, end: NaiveDate) -> Option<u64> {
        if start > end {
            return None;
        }
        let before = self.count_before(start);
        let through = self
            .count_before(end)
            .checked_add(u64::from(self.rank(end).is_some()))?;
        through.checked_sub(before)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn merges_input_and_supports_rank_select_and_gaps() {
        let index = DateCoverageIndex::new(&[
            (day(7), day(9)),
            (day(1), day(2)),
            (day(2), day(3)),
            (day(8), day(8)),
        ])
        .unwrap();
        assert_eq!(index.intervals(), &[(day(1), day(3)), (day(7), day(9))]);
        assert_eq!(index.len(), 6);
        assert_eq!(index.select(0), Some(day(1)));
        assert_eq!(index.select(3), Some(day(7)));
        assert_eq!(index.select(5), Some(day(9)));
        assert_eq!(index.select(6), None);
        assert_eq!(index.rank(day(7)), Some(3));
        assert_eq!(index.rank(day(5)), None);
        assert_eq!(index.count_before(day(1)), 0);
        assert_eq!(index.count_before(day(5)), 3);
        assert_eq!(index.count_before(day(8)), 4);
        assert_eq!(index.count_before(day(31)), 6);
    }

    #[test]
    fn agrees_with_brute_force_for_many_small_input_sets() {
        for a in 1..=9 {
            for b in a..=9 {
                for c in 1..=9 {
                    for d in c..=9 {
                        let input = [(day(a), day(b)), (day(c), day(d))];
                        let index = DateCoverageIndex::new(&input).unwrap();
                        let expected: Vec<_> = (1..=10)
                            .map(day)
                            .filter(|date| input.iter().any(|(lo, hi)| lo <= date && date <= hi))
                            .collect();
                        assert_eq!(index.len(), expected.len() as u64);
                        for (i, date) in expected.iter().enumerate() {
                            assert_eq!(index.select(i as u64), Some(*date));
                            assert_eq!(index.rank(*date), Some(i as u64));
                        }
                        for n in 1..=10 {
                            assert_eq!(
                                index.count_before(day(n)),
                                expected.iter().filter(|&&date| date < day(n)).count() as u64
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn seeks_nearest_covered_dates_across_gaps() {
        let index = DateCoverageIndex::new(&[(day(1), day(3)), (day(7), day(9))]).unwrap();
        assert_eq!(index.first_on_or_after(day(2)), Some(day(2)));
        assert_eq!(index.first_on_or_after(day(5)), Some(day(7)));
        assert_eq!(index.first_on_or_after(day(10)), None);
        assert_eq!(index.last_on_or_before(day(2)), Some(day(2)));
        assert_eq!(index.last_on_or_before(day(5)), Some(day(3)));
        let empty = DateCoverageIndex::new(&[]).unwrap();
        assert_eq!(empty.first_on_or_after(day(1)), None);
        assert_eq!(empty.last_on_or_before(day(1)), None);
        let full = DateCoverageIndex::new(&[(NaiveDate::MIN, NaiveDate::MAX)]).unwrap();
        assert_eq!(full.first_on_or_after(NaiveDate::MAX), Some(NaiveDate::MAX));
        assert_eq!(full.last_on_or_before(NaiveDate::MIN), Some(NaiveDate::MIN));
    }

    #[test]
    fn handles_empty_invalid_and_chrono_boundary_intervals() {
        let empty = DateCoverageIndex::new(&[]).unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert_eq!(empty.select(0), None);
        assert_eq!(empty.rank(day(1)), None);
        assert_eq!(DateCoverageIndex::new(&[(day(3), day(2))]), None);
        let boundaries = DateCoverageIndex::new(&[
            (NaiveDate::MAX, NaiveDate::MAX),
            (NaiveDate::MIN, NaiveDate::MIN),
        ])
        .unwrap();
        assert_eq!(boundaries.len(), 2);
        assert_eq!(boundaries.select(0), Some(NaiveDate::MIN));
        assert_eq!(boundaries.select(1), Some(NaiveDate::MAX));
        assert_eq!(boundaries.rank(NaiveDate::MAX), Some(1));
    }
    #[test]
    fn counts_covered_days_in_inclusive_queries() {
        let day = |m, d| NaiveDate::from_ymd_opt(2024, m, d).unwrap();
        let index = DateCoverageIndex::new(&[
            (day(2, 28), day(3, 1)),
            (day(3, 5), day(3, 6)),
        ])
        .unwrap();
        assert_eq!(index.count_inclusive(day(2, 29), day(3, 5)), Some(3));
        assert_eq!(index.count_inclusive(day(3, 2), day(3, 4)), Some(0));
        assert_eq!(index.count_inclusive(day(2, 28), day(3, 6)), Some(5));
        assert_eq!(index.count_inclusive(day(3, 6), day(3, 5)), None);
        let full = DateCoverageIndex::new(&[(NaiveDate::MIN, NaiveDate::MAX)]).unwrap();
        assert_eq!(
            full.count_inclusive(NaiveDate::MIN, NaiveDate::MAX),
            Some(full.len())
        );
    }


}
