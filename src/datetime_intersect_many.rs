//! Find the intersection of several datetime interval unions.

use chrono::{NaiveDateTime, TimeDelta};

/// Intersect each union of inclusive datetime intervals at nanosecond precision.
///
/// Intervals may be unordered and overlapping within each calendar. The empty
/// collection, or any empty calendar, has an empty intersection. Returns None
/// for any reversed range. Work is bounded by input intervals, not time ticks.
pub fn intersect_many_datetime_interval_sets(
    sets: &[&[(NaiveDateTime, NaiveDateTime)]],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if sets.is_empty() {
        return Some(Vec::new());
    }
    let normalized: Vec<_> = sets
        .iter()
        .map(|ranges| crate::merge_datetime_intervals(ranges))
        .collect::<Option<_>>()?;
    if normalized.iter().any(Vec::is_empty) {
        return Some(Vec::new());
    }
    let mut positions = vec![0_usize; normalized.len()];
    let mut result: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::new();
    loop {
        let mut latest_start = NaiveDateTime::MIN;
        let mut earliest_end = NaiveDateTime::MAX;
        for (ranges, &index) in normalized.iter().zip(positions.iter()) {
            let Some(&(start, end)) = ranges.get(index) else {
                return Some(result);
            };
            latest_start = latest_start.max(start);
            earliest_end = earliest_end.min(end);
        }
        if latest_start <= earliest_end {
            if let Some(last) = result.last_mut() {
                if latest_start <= last.1
                    || last.1.checked_add_signed(TimeDelta::nanoseconds(1)) == Some(latest_start)
                {
                    last.1 = last.1.max(earliest_end);
                } else {
                    result.push((latest_start, earliest_end));
                }
            } else {
                result.push((latest_start, earliest_end));
            }
        }
        for (ranges, index) in normalized.iter().zip(positions.iter_mut()) {
            if ranges[*index].1 == earliest_end {
                *index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn tick(n: i64) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::nanoseconds(n))
            .unwrap()
    }

    #[test]
    fn intersects_three_unordered_interval_unions() {
        let first = [(tick(8), tick(12)), (tick(0), tick(6))];
        let second = [(tick(3), tick(9))];
        let third = [(tick(1), tick(5)), (tick(9), tick(11))];
        assert_eq!(
            intersect_many_datetime_interval_sets(&[&first, &second, &third]),
            Some(vec![(tick(3), tick(5)), (tick(9), tick(9))])
        );
    }

    #[test]
    fn matches_brute_force_for_small_calendar_sets() {
        for a in 0..16_u32 {
            for b in 0..16_u32 {
                let make = |mask: u32| -> Vec<_> {
                    (0..4)
                        .filter(|i| mask & (1 << i) != 0)
                        .map(|i| (tick(i), tick(i)))
                        .collect()
                };
                let left = make(a);
                let right = make(b);
                let result = intersect_many_datetime_interval_sets(&[&left, &right]).unwrap();
                for n in 0..4 {
                    let found = result
                        .iter()
                        .any(|(first, last)| *first <= tick(n) && tick(n) <= *last);
                    assert_eq!(found, a & (1 << n) != 0 && b & (1 << n) != 0);
                }
            }
        }
    }

    #[test]
    fn rejects_invalid_and_handles_empty_or_extreme_ranges() {
        let invalid = [(tick(1), tick(0))];
        assert_eq!(intersect_many_datetime_interval_sets(&[&invalid]), None);
        assert_eq!(intersect_many_datetime_interval_sets(&[]), Some(vec![]));
        assert_eq!(intersect_many_datetime_interval_sets(&[&[]]), Some(vec![]));
        let full = [(NaiveDateTime::MIN, NaiveDateTime::MAX)];
        let last = [(NaiveDateTime::MAX, NaiveDateTime::MAX)];
        assert_eq!(
            intersect_many_datetime_interval_sets(&[&full, &last]),
            Some(last.to_vec())
        );
    }
}
