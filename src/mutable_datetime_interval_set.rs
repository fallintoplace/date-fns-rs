//! Mutable nanosecond-precise interval union based on a B-tree.

use std::collections::BTreeMap;

use chrono::{NaiveDateTime, TimeDelta};

/// A normalized, incrementally updated union of inclusive datetime ranges.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MutableDatetimeIntervalSet {
    ranges: BTreeMap<NaiveDateTime, NaiveDateTime>,
}

impl MutableDatetimeIntervalSet {
    /// Create an initially empty datetime interval set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Test membership in O(log n) time.
    pub fn contains(&self, value: NaiveDateTime) -> bool {
        self.ranges
            .range(..=value)
            .next_back()
            .is_some_and(|(_, end)| *end >= value)
    }

    /// Iterate normalized inclusive ranges in increasing order.
    pub fn intervals(&self) -> impl Iterator<Item = (NaiveDateTime, NaiveDateTime)> + '_ {
        self.ranges.iter().map(|(&start, &end)| (start, end))
    }

    /// Insert an inclusive interval and merge overlapping or adjacent ones.
    ///
    /// A one-nanosecond boundary counts as adjacency. Reversed intervals
    /// return None without modifying the set.
    pub fn insert(&mut self, start: NaiveDateTime, end: NaiveDateTime) -> Option<()> {
        if start > end {
            return None;
        }
        let tick = TimeDelta::nanoseconds(1);
        let mut left = start;
        let mut right = end;

        if let Some((&previous_start, &previous_end)) = self.ranges.range(..=start).next_back() {
            if previous_end >= start || previous_end.checked_add_signed(tick) == Some(start) {
                left = previous_start;
                right = right.max(previous_end);
                self.ranges.remove(&previous_start);
            }
        }

        while let Some((&next_start, &next_end)) = self.ranges.range(left..).next() {
            if next_start > right && right.checked_add_signed(tick) != Some(next_start) {
                break;
            }
            right = right.max(next_end);
            self.ranges.remove(&next_start);
        }
        self.ranges.insert(left, right);
        Some(())
    }

    /// Remove an inclusive interval, preserving fragments at nanosecond edges.
    ///
    /// Only overlapping B-tree entries are visited. Reversed endpoints do not
    /// change the set.
    pub fn remove(&mut self, start: NaiveDateTime, end: NaiveDateTime) -> Option<()> {
        if start > end {
            return None;
        }
        let tick = TimeDelta::nanoseconds(1);
        let mut affected = Vec::new();
        if let Some((&first, &last)) = self.ranges.range(..=start).next_back() {
            if first < start && last >= start {
                affected.push(first);
            }
        }
        affected.extend(self.ranges.range(start..=end).map(|(&first, _)| first));

        for first in affected {
            let last = self.ranges.remove(&first)?;
            if first < start {
                self.ranges.insert(first, start.checked_sub_signed(tick)?);
            }
            if last > end {
                self.ranges.insert(end.checked_add_signed(tick)?, last);
            }
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn tick(n: i64) -> NaiveDateTime {
        let base = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        base.checked_add_signed(TimeDelta::nanoseconds(n)).unwrap()
    }

    #[test]
    fn merges_and_splits_exact_nanosecond_neighbors() {
        let mut index = MutableDatetimeIntervalSet::new();
        index.insert(tick(1), tick(3)).unwrap();
        index.insert(tick(5), tick(7)).unwrap();
        index.insert(tick(4), tick(4)).unwrap();
        assert_eq!(
            index.intervals().collect::<Vec<_>>(),
            vec![(tick(1), tick(7))]
        );
        index.remove(tick(3), tick(5)).unwrap();
        assert_eq!(
            index.intervals().collect::<Vec<_>>(),
            vec![(tick(1), tick(2)), (tick(6), tick(7))]
        );
        assert!(index.contains(tick(2)));
        assert!(!index.contains(tick(3)));
    }

    #[test]
    fn handles_extremes_and_invalid_intervals_without_mutation() {
        let mut index = MutableDatetimeIntervalSet::new();
        index
            .insert(NaiveDateTime::MIN, NaiveDateTime::MAX)
            .unwrap();
        index
            .remove(
                NaiveDateTime::MIN
                    .checked_add_signed(TimeDelta::nanoseconds(1))
                    .unwrap(),
                NaiveDateTime::MAX
                    .checked_sub_signed(TimeDelta::nanoseconds(1))
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            index.intervals().collect::<Vec<_>>(),
            vec![
                (NaiveDateTime::MIN, NaiveDateTime::MIN),
                (NaiveDateTime::MAX, NaiveDateTime::MAX),
            ]
        );
        let original = index.clone();
        assert_eq!(index.insert(tick(2), tick(1)), None);
        assert_eq!(index.remove(tick(2), tick(1)), None);
        assert_eq!(original, index);
    }

    #[test]
    fn agrees_with_small_domain_reference_updates() {
        let mut index = MutableDatetimeIntervalSet::new();
        let mut reference = [false; 8];
        for first in 0..8 {
            for last in first..8 {
                index.insert(tick(first), tick(last)).unwrap();
                for i in first..=last {
                    reference[i as usize] = true;
                }
                for i in 0..8 {
                    assert_eq!(index.contains(tick(i)), reference[i as usize]);
                }
            }
        }
        for first in 0..8 {
            for last in first..8 {
                index.remove(tick(first), tick(last)).unwrap();
                for i in first..=last {
                    reference[i as usize] = false;
                }
                for i in 0..8 {
                    assert_eq!(index.contains(tick(i)), reference[i as usize]);
                }
            }
        }
    }
}
