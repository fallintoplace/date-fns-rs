//! Calendar date metadata with Gregorian leap-year rules.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::{end_of_month, is_leap_year};

/// Return the number of the day within the calendar year, starting at 1.
pub fn get_day_of_year(date: NaiveDate) -> u32 {
    date.ordinal()
}

/// Return the number of days in the date's month.
pub fn get_days_in_month(date: NaiveDate) -> u32 {
    end_of_month(date).day()
}

/// Return 365 or 366 for the date's calendar year.
pub fn get_days_in_year(date: NaiveDate) -> u32 {
    if is_leap_year(date.year()) {
        366
    } else {
        365
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn ordinal_counts_leap_days_and_new_year() {
        assert_eq!(get_day_of_year(date(2024, 2, 29)), 60);
        assert_eq!(get_day_of_year(date(2024, 12, 31)), 366);
        assert_eq!(get_day_of_year(date(2023, 12, 31)), 365);
        assert_eq!(get_day_of_year(date(2025, 1, 1)), 1);
    }

    #[test]
    fn month_lengths_follow_gregorian_rules() {
        assert_eq!(get_days_in_month(date(2024, 2, 1)), 29);
        assert_eq!(get_days_in_month(date(1900, 2, 1)), 28);
        assert_eq!(get_days_in_month(date(2000, 2, 1)), 29);
        assert_eq!(get_days_in_month(date(2024, 4, 30)), 30);
        assert_eq!(get_days_in_month(date(2024, 12, 1)), 31);
    }

    #[test]
    fn year_length_is_safe_at_supported_date_limits() {
        assert_eq!(get_days_in_year(date(2024, 1, 1)), 366);
        assert_eq!(get_days_in_year(date(2023, 1, 1)), 365);
        assert!((1..=366).contains(&get_day_of_year(NaiveDate::MIN)));
        assert!((1..=366).contains(&get_day_of_year(NaiveDate::MAX)));
    }
}
