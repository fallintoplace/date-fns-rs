//! Constant-time inclusive calendar-day counts for date intervals.

use chrono::NaiveDate;

/// Count all calendar dates in an inclusive interval.
///
/// Unlike a signed date difference, this includes both endpoints.
/// Returns None for reversed intervals. The complete Chrono date range
/// fits in i64 without arithmetic overflow.
pub fn count_calendar_days_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(end.signed_duration_since(start).num_days() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_both_endpoints_and_leap_days() {
        assert_eq!(
            count_calendar_days_in_interval(day(2024, 2, 28), day(2024, 3, 1)),
            Some(3)
        );
        assert_eq!(
            count_calendar_days_in_interval(day(2024, 2, 29), day(2024, 2, 29)),
            Some(1)
        );
        assert_eq!(
            count_calendar_days_in_interval(day(2024, 3, 1), day(2024, 2, 28)),
            None
        );
    }

    #[test]
    fn supports_full_chrono_range_and_partitions_by_weekday() {
        let start = NaiveDate::MIN;
        let end = NaiveDate::MAX;
        let days = count_calendar_days_in_interval(start, end).unwrap();
        let business = crate::count_business_days_in_interval(start, end).unwrap();
        let weekend = crate::count_weekend_days_in_interval(start, end).unwrap();
        assert_eq!(days, business + weekend);
        assert_eq!(count_calendar_days_in_interval(end, end), Some(1));
    }
}
