//! Output-sensitive sweep joins for inclusive datetime intervals.

use std::collections::BTreeSet;

use chrono::NaiveDateTime;

#[derive(Clone, Copy)]
struct Event {
    time: NaiveDateTime,
    opening: bool,
    side: u8,
    index: usize,
}

/// Return original left/right index pairs whose inclusive datetimes overlap.
///
/// Inputs may be unordered or contain duplicates. An equal endpoint counts
/// as overlapping. Results are sorted by left index then right index.
/// Returns None when an input interval is reversed.
pub fn join_overlapping_datetime_intervals(
    left: &[(NaiveDateTime, NaiveDateTime)],
    right: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(usize, usize)>> {
    let capacity = left.len().checked_add(right.len())?.checked_mul(2)?;
    let mut events = Vec::with_capacity(capacity);
    for (side, ranges) in [(0_u8, left), (1_u8, right)] {
        for (index, &(start, end)) in ranges.iter().enumerate() {
            if start > end {
                return None;
            }
            events.push(Event {
                time: start,
                opening: true,
                side,
                index,
            });
            events.push(Event {
                time: end,
                opening: false,
                side,
                index,
            });
        }
    }
    events.sort_unstable_by_key(|event| (event.time, !event.opening, event.side, event.index));
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
    use chrono::{NaiveDate, TimeDelta};

    fn tick(n: i64) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::nanoseconds(n))
            .unwrap()
    }

    #[test]
    fn joins_touching_nested_and_duplicate_intervals() {
        let left = [(tick(3), tick(7)), (tick(0), tick(2)), (tick(3), tick(7))];
        let right = [(tick(7), tick(7)), (tick(1), tick(3)), (tick(8), tick(9))];
        assert_eq!(
            join_overlapping_datetime_intervals(&left, &right),
            Some(vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)])
        );
    }

    #[test]
    fn agrees_with_quadratic_reference_for_small_inputs() {
        let candidates = [(tick(0), tick(1)), (tick(1), tick(2)), (tick(4), tick(5))];
        for a_mask in 0..8_u32 {
            for b_mask in 0..8_u32 {
                let select = |mask: u32| -> Vec<_> {
                    candidates
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, interval)| *interval)
                        .collect()
                };
                let a = select(a_mask);
                let b = select(b_mask);
                let expected: Vec<_> = a
                    .iter()
                    .enumerate()
                    .flat_map(|(i, &(start, end))| {
                        b.iter().enumerate().filter_map(move |(j, &(first, last))| {
                            (start <= last && first <= end).then_some((i, j))
                        })
                    })
                    .collect();
                assert_eq!(join_overlapping_datetime_intervals(&a, &b), Some(expected));
            }
        }
    }

    #[test]
    fn rejects_invalid_ranges_and_preserves_extreme_instants() {
        assert_eq!(
            join_overlapping_datetime_intervals(&[(tick(2), tick(1))], &[]),
            None
        );
        assert_eq!(
            join_overlapping_datetime_intervals(
                &[(NaiveDateTime::MIN, NaiveDateTime::MAX)],
                &[(NaiveDateTime::MAX, NaiveDateTime::MAX)]
            ),
            Some(vec![(0, 0)])
        );
    }
}
