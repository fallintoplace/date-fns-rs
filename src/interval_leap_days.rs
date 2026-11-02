//! Constant-time leap-day counts across Gregorian calendar intervals.

use chrono::{Datelike, NaiveDate};

fn leap_years_through(year: i64) -> i64 {
    year.div_euclid(4) - year.div_euclid(100) + year.div_euclid(400)
}

fn leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Count February 29 dates within an inclusive calendar interval.
///
/// Reversed intervals return None. Uses the proleptic Gregorian calendar,
/// including year zero and negative years, without iterating over days.
pub fn count_leap_days_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    let start_year = i64::from(start.year());
    let end_year = i64::from(end.year());
    let mut count =
        leap_years_through(end_year) - leap_years_through(start_year - 1);
    if leap_year(start_year) && start.ordinal() > 60 {
        count -= 1;
    }
    if leap_year(end_year) && end.ordinal() < 60 {
        count -= 1;
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn counts_inclusive_leap_days_and_century_exceptions() {
        assert_eq!(count_leap_days_in_interval(date(2024, 2, 29), date(2024, 2, 29)), Some(1));
        assert_eq!(count_leap_days_in_interval(date(2024, 3, 1), date(2024, 12, 31)), Some(0));
        assert_eq!(count_leap_days_in_interval(date(1896, 1, 1), date(2000, 12, 31)), Some(26));
        assert_eq!(count_leap_days_in_interval(date(1900, 1, 1), date(1900, 12, 31)), Some(0));
        assert_eq!(count_leap_days_in_interval(date(-1, 1, 1), date(0, 12, 31)), Some(1));
    }

    #[test]
    fn handles_extreme_dates_and_reversed_intervals() {
        assert_eq!(count_leap_days_in_interval(NaiveDate::MAX, NaiveDate::MIN), None);
        assert!(count_leap_days_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap() > 0);
        assert_eq!(count_leap_days_in_interval(date(2023, 1, 1), date(2023, 12, 31)), Some(0));
    }
}
