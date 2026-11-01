//! Normalize overlapping and nanosecond-adjacent naive datetime intervals.

use chrono::{NaiveDateTime, TimeDelta};

/// Merge unordered, inclusive datetime intervals into sorted disjoint ranges.
///
/// Adjacent intervals merge when exactly one nanosecond separates the previous
/// inclusive end from the next inclusive start. Invalid input returns None.
pub fn merge_datetime_intervals(
    intervals: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if intervals.iter().any(|(start, end)| start > end) {
        return None;
    }
    let mut sorted = intervals.to_vec();
    sorted.sort_unstable_by_key(|(start, _)| *start);

    let mut merged: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::with_capacity(sorted.len());
    for (start, end) in sorted {
        if let Some(previous) = merged.last_mut() {
            let adjacent =
                previous.1.checked_add_signed(TimeDelta::nanoseconds(1)) == Some(start);
            if start <= previous.1 || adjacent {
                previous.1 = previous.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    Some(merged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn stamp(second: u32, nanos: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 0, second, nanos)
            .unwrap()
    }

    #[test]
    fn merges_unordered_nested_overlapping_and_touching_ranges() {
        let input = [
            (stamp(3, 0), stamp(4, 0)),
            (stamp(1, 1), stamp(2, 0)),
            (stamp(0, 0), stamp(1, 0)),
            (stamp(2, 1), stamp(3, 0)),
            (stamp(0, 0), stamp(0, 1)),
        ];
        assert_eq!(
            merge_datetime_intervals(&input),
            Some(vec![(stamp(0, 0), stamp(4, 0))])
        );
    }

    #[test]
    fn preserves_gaps_and_checks_invalid_ranges() {
        assert_eq!(merge_datetime_intervals(&[]), Some(vec![]));
        assert_eq!(
            merge_datetime_intervals(&[
                (stamp(2, 0), stamp(2, 0)),
                (stamp(0, 0), stamp(0, 0))
            ]),
            Some(vec![
                (stamp(0, 0), stamp(0, 0)),
                (stamp(2, 0), stamp(2, 0))
            ])
        );
        assert_eq!(
            merge_datetime_intervals(&[(stamp(2, 0), stamp(1, 0))]),
            None
        );
    }

    #[test]
    fn handles_max_datetime_without_overflow() {
        let previous = NaiveDateTime::MAX
            .checked_sub_signed(TimeDelta::nanoseconds(1))
            .unwrap();
        assert_eq!(
            merge_datetime_intervals(&[
                (NaiveDateTime::MAX, NaiveDateTime::MAX),
                (previous, previous)
            ]),
            Some(vec![(previous, NaiveDateTime::MAX)])
        );
    }
}
