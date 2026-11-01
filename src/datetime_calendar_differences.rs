//! Signed calendar-unit boundary differences between naive date-times.

use chrono::{NaiveDateTime, Weekday};

/// Count calendar day boundaries from right to left, ignoring clock time.
pub fn difference_in_calendar_days_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    crate::difference_in_calendar_days(left.date(), right.date())
}

/// Count calendar week boundaries for an explicit first weekday.
pub fn difference_in_calendar_weeks_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
    week_starts_on: Weekday,
) -> i64 {
    crate::difference_in_calendar_weeks(left.date(), right.date(), week_starts_on)
}

/// Count calendar month boundaries, ignoring day and clock time.
pub fn difference_in_calendar_months_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    crate::difference_in_calendar_months(left.date(), right.date())
}

/// Count calendar quarter boundaries, ignoring day and clock time.
pub fn difference_in_calendar_quarters_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    crate::difference_in_calendar_quarters(left.date(), right.date())
}

/// Count calendar year boundaries, ignoring month, day and clock time.
pub fn difference_in_calendar_years_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    crate::difference_in_calendar_years(left.date(), right.date())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, min, 0)
            .unwrap()
    }

    #[test]
    fn counts_boundaries_even_for_short_elapsed_times() {
        let end = dt(2025, 1, 1, 0, 0);
        let start = dt(2024, 12, 31, 23, 59);
        assert_eq!(difference_in_calendar_days_datetime(end, start), 1);
        assert_eq!(difference_in_calendar_months_datetime(end, start), 1);
        assert_eq!(difference_in_calendar_quarters_datetime(end, start), 1);
        assert_eq!(difference_in_calendar_years_datetime(end, start), 1);
        assert_eq!(difference_in_calendar_weeks_datetime(end, start, Weekday::Mon), 0);
        let monday = dt(2024, 1, 8, 0, 0);
        let sunday = dt(2024, 1, 7, 23, 59);
        assert_eq!(
            difference_in_calendar_weeks_datetime(monday, sunday, Weekday::Mon),
            1
        );
    }

    #[test]
    fn is_signed_and_handles_chrono_limits() {
        let first = NaiveDateTime::MIN;
        let last = NaiveDateTime::MAX;
        let days = difference_in_calendar_days_datetime(last, first);
        assert!(days > 0);
        assert_eq!(difference_in_calendar_days_datetime(first, last), -days);
        assert_eq!(difference_in_calendar_days_datetime(first, first), 0);
        let weeks = difference_in_calendar_weeks_datetime(last, first, Weekday::Sun);
        assert_eq!(
            difference_in_calendar_weeks_datetime(first, last, Weekday::Sun),
            -weeks
        );
    }
}
