//! Greedy packing of fixed-length slots into free inclusive datetime ranges.

use chrono::{NaiveDateTime, TimeDelta};

fn fill_gap(
    start: NaiveDateTime,
    end: NaiveDateTime,
    length_nanos: i64,
    gap_nanos: i64,
    limit: usize,
    result: &mut Vec<(NaiveDateTime, NaiveDateTime)>,
) {
    let length_minus_one = TimeDelta::nanoseconds(length_nanos - 1);
    let gap = TimeDelta::nanoseconds(gap_nanos);
    let tick = TimeDelta::nanoseconds(1);
    let mut next = Some(start);

    while result.len() < limit {
        let Some(first) = next else {
            break;
        };
        let Some(last) = first.checked_add_signed(length_minus_one) else {
            break;
        };
        if last > end {
            break;
        }
        result.push((first, last));
        next = last
            .checked_add_signed(tick)
            .and_then(|following| following.checked_add_signed(gap));
    }
}

/// Pack the earliest available, non-overlapping slots into `[start, end]`.
///
/// Slot length counts nanosecond ticks, so a one-tick slot has identical start
/// and end. `gap_nanos` counts free ticks required between consecutive slots.
/// A zero output limit returns no slots. Invalid durations, reversed windows,
/// or reversed exclusions return None. Time is O(n log n + output count).
pub fn pack_datetime_slots(
    start: NaiveDateTime,
    end: NaiveDateTime,
    exclusions: &[(NaiveDateTime, NaiveDateTime)],
    slot_nanos: i64,
    gap_nanos: i64,
    limit: usize,
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if start > end || slot_nanos <= 0 || gap_nanos < 0 {
        return None;
    }
    let occupied = crate::merge_datetime_intervals(exclusions)?;
    let tick = TimeDelta::nanoseconds(1);
    let mut result = Vec::new();
    if limit == 0 {
        return Some(result);
    }

    let mut cursor = Some(start);
    for (first, last) in occupied {
        let Some(current) = cursor else {
            break;
        };
        if first > end {
            break;
        }
        if last < current {
            continue;
        }
        if first > current {
            fill_gap(
                current,
                first.checked_sub_signed(tick)?,
                slot_nanos,
                gap_nanos,
                limit,
                &mut result,
            );
        }
        if result.len() >= limit {
            return Some(result);
        }
        cursor = last
            .checked_add_signed(tick)
            .filter(|following| *following <= end);
    }

    if let Some(current) = cursor {
        fill_gap(current, end, slot_nanos, gap_nanos, limit, &mut result);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn tick(n: i64) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::nanoseconds(n))
            .unwrap()
    }

    #[test]
    fn packs_with_separation_and_skips_excluded_instants() {
        let slots =
            pack_datetime_slots(tick(0), tick(20), &[(tick(7), tick(10))], 3, 1, 10).unwrap();
        assert_eq!(
            slots,
            vec![
                (tick(0), tick(2)),
                (tick(4), tick(6)),
                (tick(11), tick(13)),
                (tick(15), tick(17)),
            ]
        );
        assert_eq!(
            pack_datetime_slots(tick(0), tick(20), &[], 3, 0, 2)
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn handles_single_nanoseconds_and_extreme_boundaries() {
        let slots = pack_datetime_slots(NaiveDateTime::MAX, NaiveDateTime::MAX, &[], 1, 0, 3);
        assert_eq!(slots, Some(vec![(NaiveDateTime::MAX, NaiveDateTime::MAX)]));
        assert_eq!(
            pack_datetime_slots(tick(0), tick(0), &[], 1, 0, 5),
            Some(vec![(tick(0), tick(0))])
        );
        assert_eq!(
            pack_datetime_slots(tick(0), tick(20), &[], 1, 0, 0),
            Some(vec![])
        );
    }

    #[test]
    fn rejects_invalid_windows_exclusions_and_durations() {
        assert!(pack_datetime_slots(tick(2), tick(1), &[], 1, 0, 1).is_none());
        assert!(pack_datetime_slots(tick(0), tick(1), &[], 0, 0, 1).is_none());
        assert!(pack_datetime_slots(tick(0), tick(1), &[], 1, -1, 1).is_none());
        assert!(pack_datetime_slots(tick(0), tick(5), &[(tick(4), tick(3))], 1, 0, 1,).is_none());
        assert_eq!(
            pack_datetime_slots(tick(0), tick(1), &[], 3, 0, 4),
            Some(vec![])
        );
    }

    #[test]
    fn packs_without_allocating_nanoseconds_of_dense_ranges() {
        let result =
            pack_datetime_slots(NaiveDateTime::MIN, NaiveDateTime::MAX, &[], 1, 0, 3).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, NaiveDateTime::MIN);
    }
}
