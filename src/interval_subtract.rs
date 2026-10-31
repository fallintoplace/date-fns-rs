//! Difference of inclusive calendar intervals.

use chrono::NaiveDate;

/// Subtract an inclusive interval from another inclusive interval.
///
/// Returns None if either input is reversed. Otherwise, returns the remaining
/// zero, one, or two inclusive intervals in chronological order. Endpoints
/// shared with the excluded interval are removed.
pub fn subtract_intervals(
    start: NaiveDate,
    end: NaiveDate,
    exclude_start: NaiveDate,
    exclude_end: NaiveDate,
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if start > end || exclude_start > exclude_end {
        return None;
    }
    if exclude_end < start || exclude_start > end {
        return Some(vec![(start, end)]);
    }

    let mut remaining = Vec::with_capacity(2);
    if exclude_start > start {
        remaining.push((start, exclude_start.pred_opt()?));
    }
    if exclude_end < end {
        remaining.push((exclude_end.succ_opt()?, end));
    }
    Some(remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn splits_an_interval_into_two_inclusive_parts() {
        assert_eq!(
            subtract_intervals(date(1), date(10), date(4), date(6)),
            Some(vec![(date(1), date(3)), (date(7), date(10))])
        );
    }

    #[test]
    fn handles_partial_overlap_total_removal_and_no_overlap() {
        assert_eq!(
            subtract_intervals(date(1), date(10), date(1), date(3)),
            Some(vec![(date(4), date(10))])
        );
        assert_eq!(
            subtract_intervals(date(1), date(10), date(9), date(20)),
            Some(vec![(date(1), date(8))])
        );
        assert_eq!(
            subtract_intervals(date(1), date(10), date(1), date(10)),
            Some(vec![])
        );
        assert_eq!(
            subtract_intervals(date(1), date(10), date(11), date(12)),
            Some(vec![(date(1), date(10))])
        );
    }

    #[test]
    fn rejects_invalid_ranges_and_handles_date_extremes() {
        assert_eq!(subtract_intervals(date(2), date(1), date(3), date(4)), None);
        assert_eq!(subtract_intervals(date(1), date(2), date(4), date(3)), None);
        assert_eq!(
            subtract_intervals(
                NaiveDate::MIN,
                NaiveDate::MAX,
                NaiveDate::MIN,
                NaiveDate::MIN
            ),
            Some(vec![(NaiveDate::MIN.succ_opt().unwrap(), NaiveDate::MAX)])
        );
        assert_eq!(
            subtract_intervals(
                NaiveDate::MAX,
                NaiveDate::MAX,
                NaiveDate::MAX,
                NaiveDate::MAX
            ),
            Some(vec![])
        );
    }
}
