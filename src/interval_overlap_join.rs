//! Output-sensitive overlap joins across two date interval collections.

use std::collections::BTreeSet;

use chrono::NaiveDate;

#[derive(Clone, Copy)]
struct Event {
    day: NaiveDate,
    opening: bool,
    side: u8,
    index: usize,
}

/// Return original index pairs whose inclusive date intervals overlap.
///
/// Inputs can be unordered, nested, or duplicated. Each pair appears exactly
/// once, sorted by the left index and then the right index. Starts are
/// processed before ends on the same day, so endpoint touching is an overlap.
/// Returns None if any interval is reversed.
///
/// Complexity is O((n + m) log(n + m) + k log k), where k is the number
/// of reported overlapping pairs. No calendar dates are enumerated.
pub fn join_overlapping_date_intervals(
    left: &[(NaiveDate, NaiveDate)],
    right: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(usize, usize)>> {
    let capacity = left.len().checked_add(right.len())?.checked_mul(2)?;
    let mut events = Vec::with_capacity(capacity);
    for (side, intervals) in [(0_u8, left), (1_u8, right)] {
        for (index, &(start, end)) in intervals.iter().enumerate() {
            if start > end {
                return None;
            }
            events.push(Event {
                day: start,
                opening: true,
                side,
                index,
            });
            events.push(Event {
                day: end,
                opening: false,
                side,
                index,
            });
        }
    }
    events.sort_unstable_by_key(|event| {
        (event.day, !event.opening, event.side, event.index)
    });

    let mut active_left = BTreeSet::new();
    let mut active_right = BTreeSet::new();
    let mut matches = Vec::new();
    for event in events {
        if event.opening {
            if event.side == 0 {
                matches.extend(active_right.iter().map(|&index| (event.index, index)));
                active_left.insert(event.index);
            } else {
                matches.extend(active_left.iter().map(|&index| (index, event.index)));
                active_right.insert(event.index);
            }
        } else if event.side == 0 {
            active_left.remove(&event.index);
        } else {
            active_right.remove(&event.index);
        }
    }
    matches.sort_unstable();
    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn joins_nested_and_touching_intervals_once() {
        let left = [(day(4), day(7)), (day(1), day(3))];
        let right = [(day(8), day(10)), (day(3), day(5)), (day(7), day(7))];
        assert_eq!(
            join_overlapping_date_intervals(&left, &right),
            Some(vec![(0, 1), (0, 2), (1, 1)])
        );
        assert_eq!(
            join_overlapping_date_intervals(&[(day(5), day(5))], &[(day(5), day(5))]),
            Some(vec![(0, 0)])
        );
    }

    #[test]
    fn preserves_distinct_duplicate_records_and_original_indices() {
        let left = [(day(1), day(5)), (day(1), day(5))];
        let right = [(day(3), day(4))];
        assert_eq!(
            join_overlapping_date_intervals(&left, &right),
            Some(vec![(0, 0), (1, 0)])
        );
        assert_eq!(join_overlapping_date_intervals(&[], &right), Some(vec![]));
    }

    #[test]
    fn exhaustive_small_collections_match_quadratic_reference() {
        let candidates = [(day(1), day(3)), (day(3), day(4)), (day(6), day(7))];
        for left_mask in 0..8_u32 {
            for right_mask in 0..8_u32 {
                let select = |mask: u32| -> Vec<_> {
                    candidates
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, range)| *range)
                        .collect()
                };
                let left = select(left_mask);
                let right = select(right_mask);
                let mut expected = Vec::new();
                for (i, &(a, b)) in left.iter().enumerate() {
                    for (j, &(c, d)) in right.iter().enumerate() {
                        if a <= d && c <= b {
                            expected.push((i, j));
                        }
                    }
                }
                assert_eq!(
                    join_overlapping_date_intervals(&left, &right),
                    Some(expected)
                );
            }
        }
    }

    #[test]
    fn validates_all_ranges_and_handles_chrono_extremes() {
        assert_eq!(
            join_overlapping_date_intervals(&[(day(3), day(1))], &[]),
            None
        );
        assert_eq!(
            join_overlapping_date_intervals(&[], &[(day(3), day(1))]),
            None
        );
        assert_eq!(
            join_overlapping_date_intervals(
                &[(NaiveDate::MIN, NaiveDate::MAX)],
                &[(NaiveDate::MIN, NaiveDate::MIN), (NaiveDate::MAX, NaiveDate::MAX)]
            ),
            Some(vec![(0, 0), (0, 1)])
        );
    }
}
