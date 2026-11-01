//! Complete elapsed days and weeks between naive date-times.

use chrono::NaiveDateTime;

/// Number of complete 24-hour periods from right to left.
/// Incomplete periods truncate toward zero, including negative durations.
pub fn difference_in_days_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    left.signed_duration_since(right).num_days()
}

/// Number of complete seven-day periods from right to left.
/// This measures elapsed time, not calendar week boundaries.
pub fn difference_in_weeks_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    left.signed_duration_since(right).num_weeks()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    #[test]
    fn ignores_incomplete_days_and_weeks() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 28)
            .unwrap()
            .and_hms_opt(23, 30, 0)
            .unwrap();
        let short = start.checked_add_signed(TimeDelta::hours(23)).unwrap();
        let day = start.checked_add_signed(TimeDelta::days(1)).unwrap();
        let short_week = start.checked_add_signed(TimeDelta::days(6)).unwrap();
        let week = start.checked_add_signed(TimeDelta::weeks(1)).unwrap();

        assert_eq!(difference_in_days_datetime(short, start), 0);
        assert_eq!(difference_in_days_datetime(day, start), 1);
        assert_eq!(difference_in_days_datetime(start, short), 0);
        assert_eq!(difference_in_days_datetime(short_week, start), 6);
        assert_eq!(difference_in_weeks_datetime(short_week, start), 0);
        assert_eq!(difference_in_weeks_datetime(week, start), 1);
        assert_eq!(difference_in_weeks_datetime(start, week), -1);
    }

    #[test]
    fn handles_supported_date_limits() {
        let days = difference_in_days_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(days > 0);
        assert_eq!(
            difference_in_days_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -days
        );
        let weeks = difference_in_weeks_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(weeks > 0);
        assert_eq!(
            difference_in_weeks_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -weeks
        );
    }
}
