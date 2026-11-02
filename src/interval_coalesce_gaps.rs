//! Merge inclusive date intervals when their uncovered gaps are small enough.

use chrono::{Datelike, NaiveDate};

/// Coalesce ranges separated by at most max_gap_days uncovered calendar days.
///
/// Input may be unordered and contain overlaps. An empty input returns an
/// empty list. A zero threshold behaves like the regular interval merge.
/// Returns None for reversed input intervals.
pub fn coalesce_date_intervals_with_gap(
    intervals: &[(NaiveDate, NaiveDate)],
    max_gap_days: u64,
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    let normalized = crate::merge_intervals(intervals)?;
    let mut result: Vec<(NaiveDate, NaiveDate)> = Vec::with_capacity(normalized.len());
    for (first, last) in normalized {
        if let Some(previous) = result.last_mut() {
            let uncovered =
                i64::from(first.num_days_from_ce()) - i64::from(previous.1.num_days_from_ce()) - 1;
            if u64::try_from(uncovered).ok()? <= max_gap_days {
                previous.1 = last;
                continue;
            }
        }
        result.push((first, last));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn bridges_only_gaps_within_threshold() {
        let input = [
            (day(10), day(12)),
            (day(1), day(3)),
            (day(7), day(8)),
            (day(3), day(4)),
        ];
        assert_eq!(
            coalesce_date_intervals_with_gap(&input, 2),
            Some(vec![(day(1), day(12))])
        );
        assert_eq!(
            coalesce_date_intervals_with_gap(&input, 1),
            Some(vec![(day(1), day(4)), (day(7), day(12))])
        );
        assert_eq!(
            coalesce_date_intervals_with_gap(&input, 0),
            crate::merge_intervals(&input)
        );
    }

    #[test]
    fn validates_ranges_and_supports_calendar_extremes() {
        assert_eq!(coalesce_date_intervals_with_gap(&[], 3), Some(vec![]));
        assert_eq!(
            coalesce_date_intervals_with_gap(&[(day(3), day(2))], 9),
            None
        );
        assert_eq!(
            coalesce_date_intervals_with_gap(
                &[
                    (NaiveDate::MIN, NaiveDate::MIN),
                    (NaiveDate::MAX, NaiveDate::MAX)
                ],
                u64::MAX
            ),
            Some(vec![(NaiveDate::MIN, NaiveDate::MAX)])
        );
    }
}
