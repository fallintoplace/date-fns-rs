//! Find a free inclusive date window among arbitrary blocked intervals.

use chrono::{Days, NaiveDate};

/// Find the earliest available window of `required_days` consecutive dates.
///
/// The search is inside `[start, end]` and excludes all blocked ranges.
/// An outer None means invalid bounds, a zero window size, or an invalid
/// blocked interval. Some(None) means that no sufficiently long gap exists.
/// Complexity is O(n log n) for normalization and O(n) for gap scanning.
pub fn first_available_date_window(
    start: NaiveDate,
    end: NaiveDate,
    blocked: &[(NaiveDate, NaiveDate)],
    required_days: u64,
) -> Option<Option<(NaiveDate, NaiveDate)>> {
    if start > end || required_days == 0 {
        return None;
    }
    let exclusions = crate::merge_intervals(blocked)?;
    let offset = required_days - 1;
    let mut cursor = start;

    for (blocked_start, blocked_end) in exclusions {
        if blocked_end < cursor {
            continue;
        }
        if blocked_start > end {
            break;
        }
        if blocked_start > cursor {
            if let Some(last) = cursor.checked_add_days(Days::new(offset)) {
                if last < blocked_start && last <= end {
                    return Some(Some((cursor, last)));
                }
            }
        }
        if blocked_end >= end {
            return Some(None);
        }
        cursor = blocked_end.succ_opt()?;
    }

    Some(
        cursor
            .checked_add_days(Days::new(offset))
            .filter(|&last| last <= end)
            .map(|last| (cursor, last)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn finds_earliest_gap_and_requires_contiguous_days() {
        let blocks = [(day(8), day(9)), (day(3), day(5)), (day(4), day(4))];
        assert_eq!(
            first_available_date_window(day(1), day(15), &blocks, 2),
            Some(Some((day(1), day(2))))
        );
        assert_eq!(
            first_available_date_window(day(1), day(15), &blocks, 3),
            Some(Some((day(10), day(12))))
        );
        assert_eq!(
            first_available_date_window(day(1), day(15), &blocks, 7),
            Some(None)
        );
    }

    #[test]
    fn exhaustive_small_domain_matches_day_by_day_search() {
        let candidates = [(day(2), day(4)), (day(5), day(8)), (day(9), day(9))];
        for mask in 0..8_u32 {
            let blocked: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, interval)| *interval)
                .collect();
            for needed in 1..=11_u64 {
                let expected = (1..=10_u32).find_map(|first| {
                    let last = first.checked_add(u32::try_from(needed - 1).ok()?)?;
                    if last > 10 {
                        return None;
                    }
                    let open = (first..=last).all(|n| {
                        !blocked
                            .iter()
                            .any(|(lo, hi)| *lo <= day(n) && day(n) <= *hi)
                    });
                    open.then_some((day(first), day(last)))
                });
                assert_eq!(
                    first_available_date_window(day(1), day(10), &blocked, needed),
                    Some(expected),
                    "{mask}, {needed}"
                );
            }
        }
    }

    #[test]
    fn rejects_invalid_inputs_and_handles_calendar_extremes() {
        assert_eq!(first_available_date_window(day(2), day(1), &[], 1), None);
        assert_eq!(first_available_date_window(day(1), day(2), &[], 0), None);
        assert_eq!(
            first_available_date_window(day(1), day(2), &[(day(3), day(2))], 1),
            None
        );
        assert_eq!(
            first_available_date_window(NaiveDate::MAX, NaiveDate::MAX, &[], 1),
            Some(Some((NaiveDate::MAX, NaiveDate::MAX)))
        );
        assert_eq!(
            first_available_date_window(NaiveDate::MIN, NaiveDate::MAX, &[], u64::MAX),
            Some(None)
        );
    }
}
