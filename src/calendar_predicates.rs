//! Additional calendar-date predicates without time-zone assumptions.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::end_of_month;
use crate::quarters::get_quarter;

/// Return whether the date is the first day of its calendar month.
pub fn is_first_day_of_month(date: NaiveDate) -> bool {
    date.day() == 1
}

/// Return whether the date is the last day of its calendar month.
pub fn is_last_day_of_month(date: NaiveDate) -> bool {
    date == end_of_month(date)
}

/// Return whether the date is February 29.
pub fn is_leap_day(date: NaiveDate) -> bool {
    date.month() == 2 && date.day() == 29
}

/// Return whether two dates belong to the same quarter and calendar year.
pub fn is_same_quarter(left: NaiveDate, right: NaiveDate) -> bool {
    left.year() == right.year() && get_quarter(left) == get_quarter(right)
}

/// Return whether the date is January 1.
pub fn is_first_day_of_year(date: NaiveDate) -> bool {
    date.month() == 1 && date.day() == 1
}

/// Return whether the date is December 31.
pub fn is_last_day_of_year(date: NaiveDate) -> bool {
    date.month() == 12 && date.day() == 31
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn detects_first_and_last_days_of_month() {
        assert!(is_first_day_of_month(date(2024, 2, 1)));
        assert!(!is_first_day_of_month(date(2024, 2, 2)));
        assert!(is_last_day_of_month(date(2024, 2, 29)));
        assert!(is_last_day_of_month(date(2023, 2, 28)));
        assert!(!is_last_day_of_month(date(2024, 2, 28)));
        assert!(is_last_day_of_month(NaiveDate::MAX));
        assert!(is_first_day_of_month(NaiveDate::MIN));
    }

    #[test]
    fn identifies_leap_days_only() {
        assert!(is_leap_day(date(2024, 2, 29)));
        assert!(!is_leap_day(date(2023, 2, 28)));
        assert!(!is_leap_day(date(2024, 3, 1)));
    }

    #[test]
    fn quarter_equality_requires_matching_years() {
        assert!(is_same_quarter(date(2024, 1, 1), date(2024, 3, 31)));
        assert!(!is_same_quarter(date(2024, 3, 31), date(2024, 4, 1)));
        assert!(!is_same_quarter(date(2024, 1, 1), date(2025, 1, 1)));
    }
    #[test]
    fn identifies_first_and_last_days_of_year() {
        assert!(is_first_day_of_year(date(2024, 1, 1)));
        assert!(!is_first_day_of_year(date(2024, 1, 2)));
        assert!(is_last_day_of_year(date(2024, 12, 31)));
        assert!(!is_last_day_of_year(date(2024, 12, 30)));
        assert!(!is_last_day_of_year(date(2024, 2, 29)));
    }

    #[test]
    fn works_at_supported_date_limits() {
        assert!(is_first_day_of_year(NaiveDate::MIN));
        assert!(is_last_day_of_year(NaiveDate::MAX));
    }
}
