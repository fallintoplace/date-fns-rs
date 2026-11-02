//! Sweep-line overlap counts for inclusive calendar date intervals.

use chrono::{Datelike, NaiveDate};

/// Return maximal date segments annotated with their positive overlap depth.
///
/// Every input interval contributes one to the depth, including duplicates.
/// Input order is irrelevant. Returns None for a reversed interval.
/// Complexity: O(n log n) sorting and O(n) sweeping; no day-by-day iteration.
pub fn interval_overlap_depth(
    intervals: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(NaiveDate, NaiveDate, usize)>> {
    let mut events = Vec::with_capacity(intervals.len().checked_mul(2)?);
    for &(start, end) in intervals {
        if start > end {
            return None;
        }
        events.push((i64::from(start.num_days_from_ce()), 1_i64));
        // Use integer day ordinals so MAX + 1 is a valid sentinel.
        events.push((i64::from(end.num_days_from_ce()) + 1, -1_i64));
    }
    events.sort_unstable_by_key(|&(day, _)| day);

    let mut result: Vec<(NaiveDate, NaiveDate, usize)> = Vec::new();
    let mut active = 0_i64;
    let mut previous = None;
    let mut i = 0;

    while i < events.len() {
        let day = events[i].0;
        if let Some(start) = previous {
            if active > 0 && start < day {
                let first = NaiveDate::from_num_days_from_ce_opt(i32::try_from(start).ok()?)?;
                let last = NaiveDate::from_num_days_from_ce_opt(i32::try_from(day - 1).ok()?)?;
                let depth = usize::try_from(active).ok()?;
                if let Some(prior) = result.last_mut() {
                    if prior.2 == depth && prior.1.succ_opt() == Some(first) {
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
        while i < events.len() && events[i].0 == day {
            delta = delta.checked_add(events[i].1)?;
            i += 1;
        }
        active = active.checked_add(delta)?;
        if active < 0 {
            return None;
        }
        previous = Some(day);
    }
    (active == 0).then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn splits_overlaps_and_coalesces_equal_depth() {
        assert_eq!(
            interval_overlap_depth(&[(date(1), date(4)), (date(3), date(6)), (date(6), date(7))]),
            Some(vec![
                (date(1), date(2), 1),
                (date(3), date(4), 2),
                (date(5), date(5), 1),
                (date(6), date(6), 2),
                (date(7), date(7), 1)
            ])
        );
        assert_eq!(
            interval_overlap_depth(&[(date(1), date(2)), (date(3), date(4))]),
            Some(vec![(date(1), date(4), 1)])
        );
    }

    #[test]
    fn matches_brute_force_depth_for_all_small_masks() {
        let candidates = [
            (date(1), date(3)),
            (date(2), date(7)),
            (date(5), date(5)),
            (date(7), date(9)),
        ];
        for mask in 0..16_u32 {
            let ranges: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| (mask & (1 << i)) != 0)
                .map(|(_, item)| *item)
                .collect();
            let segments = interval_overlap_depth(&ranges).unwrap();
            for day in 1..=10 {
                let expected = ranges
                    .iter()
                    .filter(|(a, b)| *a <= date(day) && date(day) <= *b)
                    .count();
                let actual = segments
                    .iter()
                    .find(|(a, b, _)| *a <= date(day) && date(day) <= *b)
                    .map_or(0, |(_, _, depth)| *depth);
                assert_eq!(actual, expected, "{mask} at {day}");
            }
        }
    }

    #[test]
    fn handles_empty_invalid_duplicate_and_boundary_dates() {
        assert_eq!(interval_overlap_depth(&[]), Some(vec![]));
        assert_eq!(interval_overlap_depth(&[(date(2), date(1))]), None);
        assert_eq!(
            interval_overlap_depth(&[(NaiveDate::MAX, NaiveDate::MAX); 2]),
            Some(vec![(NaiveDate::MAX, NaiveDate::MAX, 2)])
        );
        assert_eq!(
            interval_overlap_depth(&[(NaiveDate::MIN, NaiveDate::MAX)]),
            Some(vec![(NaiveDate::MIN, NaiveDate::MAX, 1)])
        );
    }
}
