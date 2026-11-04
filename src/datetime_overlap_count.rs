//! Output-sensitive-free counting of overlapping inclusive interval pairs.

use chrono::NaiveDateTime;

#[derive(Clone, Copy)]
struct Event {
    point: NaiveDateTime,
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
pub fn count_overlapping_datetime_interval_pairs(
    left: &[(NaiveDateTime, NaiveDateTime)],
    right: &[(NaiveDateTime, NaiveDateTime)],
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

    fn tick(n: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, n)
            .unwrap()
    }

    #[test]
    fn counts_inclusive_nanosecond_touches_and_duplicates() {
        let left = [(tick(1), tick(3)), (tick(5), tick(7))];
        let right = [(tick(3), tick(5)), (tick(7), tick(8))];
        assert_eq!(count_overlapping_datetime_interval_pairs(&left, &right), Some(3));
        assert_eq!(
            count_overlapping_datetime_interval_pairs(
                &[(tick(2), tick(3)), (tick(2), tick(3))],
                &[(tick(3), tick(3)), (tick(3), tick(3))]
            ),
            Some(4)
        );
        assert_eq!(
            count_overlapping_datetime_interval_pairs(
                &[(NaiveDateTime::MIN, NaiveDateTime::MAX)],
                &[(NaiveDateTime::MAX, NaiveDateTime::MAX)]
            ),
            Some(1)
        );
        assert_eq!(
            count_overlapping_datetime_interval_pairs(&[(tick(5), tick(4))], &[]),
            None
        );
    }

    #[test]
    fn matches_materialized_join_for_small_collections() {
        let choices = [(tick(0), tick(2)), (tick(2), tick(4)), (tick(5), tick(6))];
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
                let expected = crate::join_overlapping_datetime_intervals(&left, &right)
                    .unwrap()
                    .len();
                assert_eq!(
                    count_overlapping_datetime_interval_pairs(&left, &right),
                    Some(expected)
                );
            }
        }
    }

}
