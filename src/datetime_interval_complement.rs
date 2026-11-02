//! Compute the complement of inclusive naive datetime intervals.

use chrono::{NaiveDateTime, TimeDelta};

/// Return all uncovered inclusive nanosecond-precision ranges in `[start, end]`.
///
/// Exclusions can be unordered, overlapping, or outside the window. Invalid
/// ranges return None. Runs in O(n log n) time, independent of interval length.
pub fn complement_datetime_intervals(
    start: NaiveDateTime,
    end: NaiveDateTime,
    exclusions: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if start > end || exclusions.iter().any(|(first, last)| first > last) {
        return None;
    }

    let mut relevant: Vec<_> = exclusions
        .iter()
        .copied()
        .filter(|(first, last)| *first <= end && *last >= start)
        .collect();
    relevant.sort_unstable_by_key(|(first, _)| *first);

    let tick = TimeDelta::nanoseconds(1);
    let mut result = Vec::new();
    let mut next = Some(start);

    for (first, last) in relevant {
        let Some(cursor) = next else {
            break;
        };
        if last < cursor {
            continue;
        }
        if first > cursor {
            result.push((cursor, first.checked_sub_signed(tick)?));
        }
        next = last
            .checked_add_signed(tick)
            .filter(|following| *following <= end);
    }

    if let Some(cursor) = next {
        result.push((cursor, end));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn instant(nanos: i64) -> NaiveDateTime {
        let base = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        base.checked_add_signed(TimeDelta::nanoseconds(nanos))
            .unwrap()
    }

    #[test]
    fn retains_subsecond_gaps_between_nested_exclusions() {
        let exclusions = [
            (instant(7), instant(20)),
            (instant(2), instant(4)),
            (instant(4), instant(5)),
            (instant(8), instant(9)),
        ];
        assert_eq!(
            complement_datetime_intervals(instant(1), instant(10), &exclusions),
            Some(vec![
                (instant(1), instant(1)),
                (instant(6), instant(6)),
            ])
        );
    }

    #[test]
    fn handles_invalid_empty_and_entirely_covered_ranges() {
        assert_eq!(
            complement_datetime_intervals(instant(1), instant(3), &[]),
            Some(vec![(instant(1), instant(3))])
        );
        assert_eq!(
            complement_datetime_intervals(
                instant(1),
                instant(3),
                &[(instant(0), instant(4))]
            ),
            Some(vec![])
        );
        assert_eq!(
            complement_datetime_intervals(instant(2), instant(1), &[]),
            None
        );
        assert_eq!(
            complement_datetime_intervals(
                instant(1),
                instant(3),
                &[(instant(3), instant(2))]
            ),
            None
        );
    }

    #[test]
    fn handles_supported_extremes_without_overflow() {
        let tick = TimeDelta::nanoseconds(1);
        let second = NaiveDateTime::MIN.checked_add_signed(tick).unwrap();
        let penultimate = NaiveDateTime::MAX.checked_sub_signed(tick).unwrap();
        assert_eq!(
            complement_datetime_intervals(
                NaiveDateTime::MIN,
                NaiveDateTime::MAX,
                &[(second, penultimate)]
            ),
            Some(vec![
                (NaiveDateTime::MIN, NaiveDateTime::MIN),
                (NaiveDateTime::MAX, NaiveDateTime::MAX),
            ])
        );
    }

    #[test]
    fn matches_all_eight_tick_exclusion_bitmasks() {
        for mask in 0_u32..256 {
            let exclusions: Vec<_> = (0..8)
                .filter(|bit| mask & (1 << bit) != 0)
                .map(|bit| (instant(bit), instant(bit)))
                .collect();
            let free =
                complement_datetime_intervals(instant(0), instant(7), &exclusions).unwrap();
            for bit in 0..8 {
                let present = free
                    .iter()
                    .any(|(a, b)| *a <= instant(bit) && instant(bit) <= *b);
                assert_eq!(present, mask & (1 << bit) == 0);
            }
        }
    }
}
