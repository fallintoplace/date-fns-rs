//! Sweep-line concurrency depth for inclusive nanosecond datetime ranges.

use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};

const BILLION: i128 = 1_000_000_000;

fn as_nanos(value: NaiveDateTime) -> Option<i128> {
    let utc = value.and_utc();
    let nanos = utc.timestamp_subsec_nanos();
    if nanos >= 1_000_000_000 {
        return None;
    }
    Some(i128::from(utc.timestamp()) * BILLION + i128::from(nanos))
}

fn from_nanos(value: i128) -> Option<NaiveDateTime> {
    let seconds = i64::try_from(value.div_euclid(BILLION)).ok()?;
    let nanos = u32::try_from(value.rem_euclid(BILLION)).ok()?;
    DateTime::<Utc>::from_timestamp(seconds, nanos).map(|datetime| datetime.naive_utc())
}

/// Return maximal datetime segments with their positive overlap depth.
///
/// Each input interval contributes one, including duplicates and shared
/// endpoints. Reversed ranges and Chrono leap-second encodings are rejected.
/// Input sorting takes O(n log n), and sweeping takes O(n).
pub fn datetime_overlap_depth(
    intervals: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime, usize)>> {
    let mut events = Vec::with_capacity(intervals.len().checked_mul(2)?);
    for &(start, end) in intervals {
        if start > end {
            return None;
        }
        events.push((as_nanos(start)?, 1_i64));
        // i128 allows the sentinel after the maximum representable datetime.
        events.push((as_nanos(end)? + 1, -1_i64));
    }
    events.sort_unstable_by_key(|&(point, _)| point);

    let mut result: Vec<(NaiveDateTime, NaiveDateTime, usize)> = Vec::new();
    let mut active = 0_i64;
    let mut previous = None;
    let mut i = 0;

    while i < events.len() {
        let point = events[i].0;
        if let Some(start) = previous {
            if active > 0 && start < point {
                let first = from_nanos(start)?;
                let last = from_nanos(point - 1)?;
                let depth = usize::try_from(active).ok()?;
                if let Some(prior) = result.last_mut() {
                    if prior.2 == depth
                        && prior.1.checked_add_signed(TimeDelta::nanoseconds(1)) == Some(first)
                    {
                        prior.1 = last;
                    } else {
                        result.push((first, last, depth));
                    }
                } else {
                    result.push((first, last, depth));
                }
            }
        }
        let mut delta = 0_i64;
        while i < events.len() && events[i].0 == point {
            delta = delta.checked_add(events[i].1)?;
            i += 1;
        }
        active = active.checked_add(delta)?;
        if active < 0 {
            return None;
        }
        previous = Some(point);
    }
    (active == 0).then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(12, 0, second)
            .unwrap()
    }

    #[test]
    fn preserves_single_nanosecond_boundaries_and_depths() {
        let nano = TimeDelta::nanoseconds(1);
        assert_eq!(
            datetime_overlap_depth(&[(at(0), at(2)), (at(1), at(3)), (at(3), at(3))]),
            Some(vec![
                (at(0), at(1).checked_sub_signed(nano).unwrap(), 1),
                (at(1), at(2), 2),
                (
                    at(2).checked_add_signed(nano).unwrap(),
                    at(3).checked_sub_signed(nano).unwrap(),
                    1
                ),
                (at(3), at(3), 2)
            ])
        );
    }

    #[test]
    fn matches_brute_force_on_short_nanosecond_domain() {
        let base = at(0);
        let date = |nano: i64| base.checked_add_signed(TimeDelta::nanoseconds(nano)).unwrap();
        let candidates = [(0, 3), (2, 6), (4, 4), (7, 9)];
        for mask in 0..16_u32 {
            let ranges: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, (a, b))| (date(*a), date(*b)))
                .collect();
            let result = datetime_overlap_depth(&ranges).unwrap();
            for n in 0..=10 {
                let expected = ranges
                    .iter()
                    .filter(|(a, b)| *a <= date(n) && date(n) <= *b)
                    .count();
                let actual = result
                    .iter()
                    .find(|(a, b, _)| *a <= date(n) && date(n) <= *b)
                    .map_or(0, |(_, _, depth)| *depth);
                assert_eq!(actual, expected, "{mask}, {n}");
            }
        }
    }

    #[test]
    fn rejects_invalid_ranges_and_preserves_supported_limits() {
        assert_eq!(datetime_overlap_depth(&[]), Some(vec![]));
        assert_eq!(datetime_overlap_depth(&[(at(2), at(1))]), None);
        assert_eq!(
            datetime_overlap_depth(&[(NaiveDateTime::MAX, NaiveDateTime::MAX); 2]),
            Some(vec![(NaiveDateTime::MAX, NaiveDateTime::MAX, 2)])
        );
        assert_eq!(
            datetime_overlap_depth(&[(NaiveDateTime::MIN, NaiveDateTime::MAX)]),
            Some(vec![(NaiveDateTime::MIN, NaiveDateTime::MAX, 1)])
        );
    }
}
