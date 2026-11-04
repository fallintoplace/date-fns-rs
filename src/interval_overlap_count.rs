//! Output-sensitive-free counting of overlapping inclusive interval pairs.

use chrono::NaiveDate;

#[derive(Clone, Copy)]
struct Event {
    point: NaiveDate,
    opening: bool,
    left: bool,
}

/// Count overlapping pairs between two interval collections.
///
/// Endpoint touching counts as overlap, and duplicates count independently.
/// Returns None for a reversed input range or count overflow.
///
/// Runs in O((n + m) log(n + m)) time and O(n + m) memory, independent
/// of the number of overlapping pairs.
pub fn count_overlapping_date_interval_pairs(
    left: &[(NaiveDate, NaiveDate)],
    right: &[(NaiveDate, NaiveDate)],
) -> Option<usize> {
    let capacity = left.len().checked_add(right.len())?.checked_mul(2)?;
    let mut events = Vec::with_capacity(capacity);
    for (is_left, intervals) in [(true, left), (false, right)] {
        for &(start, end) in intervals {
            if start > end {
                return None;
            }
            events.push(Event {
                point: start,
                opening: true,
                left: is_left,
            });
            events.push(Event {
                point: end,
                opening: false,
                left: is_left,
            });
        }
    }

    // Opening events must precede closing events at the same instant.
    events.sort_unstable_by_key(|event| (event.point, !event.opening));

    let mut active_left = 0_usize;
    let mut active_right = 0_usize;
    let mut matches = 0_usize;
    for event in events {
        match (event.left, event.opening) {
            (true, true) => {
                matches = matches.checked_add(active_right)?;
                active_left = active_left.checked_add(1)?;
            }
            (false, true) => {
                matches = matches.checked_add(active_left)?;
                active_right = active_right.checked_add(1)?;
            }
            (true, false) => active_left -= 1,
            (false, false) => active_right -= 1,
        }
    }
    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn counts_inclusive_endpoint_touches_and_duplicate_ranges() {
        let left = [(day(3), day(5)), (day(1), day(2))];
        let right = [(day(5), day(6)), (day(2), day(3)), (day(7), day(8))];
        assert_eq!(count_overlapping_date_interval_pairs(&left, &right), Some(3));
        assert_eq!(
            count_overlapping_date_interval_pairs(
                &[(day(2), day(3)), (day(2), day(3))],
                &[(day(3), day(3)), (day(3), day(3))]
            ),
            Some(4)
        );
        assert_eq!(count_overlapping_date_interval_pairs(&[], &right), Some(0));
        assert_eq!(
            count_overlapping_date_interval_pairs(&[(day(5), day(4))], &[]),
            None
        );
    }

    #[test]
    fn agrees_with_materialized_join_on_small_sets() {
        let choices = [(day(1), day(3)), (day(3), day(5)), (day(6), day(7))];
        for a in 0..8_usize {
            for b in 0..8_usize {
                let choose = |mask: usize| -> Vec<_> {
                    choices
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, pair)| *pair)
                        .collect()
                };
                let left = choose(a);
                let right = choose(b);
                let expected = crate::join_overlapping_date_intervals(&left, &right)
                    .unwrap()
                    .len();
                assert_eq!(
                    count_overlapping_date_interval_pairs(&left, &right),
                    Some(expected)
                );
            }
        }
    }

}
