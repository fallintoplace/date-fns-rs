//! Signed whole-week differences between calendar dates.

use chrono::NaiveDate;

use crate::difference::difference_in_calendar_days;

/// Count complete seven-day periods from right to left.
///
/// Partial weeks truncate toward zero; this is not the number of calendar
/// week boundaries crossed. No time-zone or daylight-saving rules apply.
pub fn difference_in_weeks(left: NaiveDate, right: NaiveDate) -> i64 {
    difference_in_calendar_days(left, right) / 7
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn truncates_incomplete_weeks_toward_zero() {
        assert_eq!(difference_in_weeks(date(2024, 3, 8), date(2024, 3, 1)), 1);
        assert_eq!(difference_in_weeks(date(2024, 3, 7), date(2024, 3, 1)), 0);
        assert_eq!(difference_in_weeks(date(2024, 3, 1), date(2024, 3, 8)), -1);
        assert_eq!(difference_in_weeks(date(2024, 3, 1), date(2024, 3, 7)), 0);
    }

    #[test]
    fn handles_leap_days_and_extreme_dates() {
        assert_eq!(difference_in_weeks(date(2024, 3, 6), date(2024, 2, 28)), 1);
        let all = difference_in_weeks(NaiveDate::MAX, NaiveDate::MIN);
        assert!(all > 0);
        assert_eq!(difference_in_weeks(NaiveDate::MIN, NaiveDate::MAX), -all);
        assert_eq!(difference_in_weeks(NaiveDate::MAX, NaiveDate::MAX), 0);
    }
}
