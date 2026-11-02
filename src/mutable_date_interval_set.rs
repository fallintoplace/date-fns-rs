//! Incrementally update normalized inclusive date ranges with a B-tree.

use std::collections::BTreeMap;

use chrono::NaiveDate;

/// A mutable union of sorted, disjoint, nonadjacent inclusive date ranges.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MutableDateIntervalSet {
    ranges: BTreeMap<NaiveDate, NaiveDate>,
}

impl MutableDateIntervalSet {
    /// Construct an empty set of calendar dates.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return whether the given calendar date belongs to the set.
    pub fn contains(&self, date: NaiveDate) -> bool {
        self.ranges
            .range(..=date)
            .next_back()
            .is_some_and(|(_, end)| *end >= date)
    }

    /// Return stored normalized inclusive intervals in chronological order.
    pub fn intervals(&self) -> impl Iterator<Item = (NaiveDate, NaiveDate)> + '_ {
        self.ranges.iter().map(|(&start, &end)| (start, end))
    }

    /// Insert an inclusive range and merge all touching or overlapping ranges.
    ///
    /// Returns None for reversed endpoints. Changes only intersecting B-tree
    /// entries instead of reconstructing the entire set.
    pub fn insert(&mut self, start: NaiveDate, end: NaiveDate) -> Option<()> {
        if start > end {
            return None;
        }
        let mut left = start;
        let mut right = end;

        if let Some((&previous_start, &previous_end)) = self.ranges.range(..=start).next_back() {
            if previous_end >= start || previous_end.succ_opt() == Some(start) {
                left = previous_start;
                right = right.max(previous_end);
                self.ranges.remove(&previous_start);
            }
        }

        while let Some((&next_start, &next_end)) = self.ranges.range(left..).next() {
            if next_start > right && right.succ_opt() != Some(next_start) {
                break;
            }
            right = right.max(next_end);
            self.ranges.remove(&next_start);
        }
        self.ranges.insert(left, right);
        Some(())
    }

    /// Remove an inclusive range, splitting any partially covered intervals.
    ///
    /// Returns None for reversed endpoints. The original set is unchanged on
    /// invalid input.
    pub fn remove(&mut self, start: NaiveDate, end: NaiveDate) -> Option<()> {
        if start > end {
            return None;
        }
        let mut affected = Vec::new();
        if let Some((&previous_start, &previous_end)) = self.ranges.range(..=start).next_back() {
            if previous_start < start && previous_end >= start {
                affected.push(previous_start);
            }
        }
        affected.extend(self.ranges.range(start..=end).map(|(&first, _)| first));

        for first in affected {
            let last = self.ranges.remove(&first)?;
            if first < start {
                self.ranges.insert(first, start.pred_opt()?);
            }
            if last > end {
                self.ranges.insert(end.succ_opt()?, last);
            }
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn merges_adjacent_ranges_and_splits_removals() {
        let mut set = MutableDateIntervalSet::new();
        set.insert(date(3), date(5)).unwrap();
        set.insert(date(7), date(9)).unwrap();
        set.insert(date(6), date(6)).unwrap();
        assert_eq!(set.intervals().collect::<Vec<_>>(), vec![(date(3), date(9))]);
        set.remove(date(5), date(7)).unwrap();
        assert_eq!(
            set.intervals().collect::<Vec<_>>(),
            vec![(date(3), date(4)), (date(8), date(9))]
        );
        assert!(set.contains(date(3)));
        assert!(!set.contains(date(5)));
    }

    #[test]
    fn handles_chrono_limits_and_rejects_reversed_ranges() {
        let mut set = MutableDateIntervalSet::new();
        set.insert(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        set.remove(
            NaiveDate::MIN.succ_opt().unwrap(),
            NaiveDate::MAX.pred_opt().unwrap(),
        )
        .unwrap();
        assert_eq!(
            set.intervals().collect::<Vec<_>>(),
            vec![
                (NaiveDate::MIN, NaiveDate::MIN),
                (NaiveDate::MAX, NaiveDate::MAX),
            ]
        );
        let snapshot = set.clone();
        assert_eq!(set.insert(date(5), date(4)), None);
        assert_eq!(set.remove(date(5), date(4)), None);
        assert_eq!(set, snapshot);
    }

    #[test]
    fn matches_boolean_model_for_all_range_updates() {
        let mut set = MutableDateIntervalSet::new();
        let mut expected = [false; 10];
        for start in 1..=10 {
            for end in start..=10 {
                set.insert(date(start), date(end)).unwrap();
                for day in start..=end {
                    expected[(day - 1) as usize] = true;
                }
                for day in 1..=10 {
                    assert_eq!(set.contains(date(day)), expected[(day - 1) as usize]);
                }
            }
        }
        for start in 1..=10 {
            for end in start..=10 {
                set.remove(date(start), date(end)).unwrap();
                for day in start..=end {
                    expected[(day - 1) as usize] = false;
                }
                for day in 1..=10 {
                    assert_eq!(set.contains(date(day)), expected[(day - 1) as usize]);
                }
            }
        }
    }
}
