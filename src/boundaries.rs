//! Calendar month and year boundaries.

use chrono::{Datelike, NaiveDate};

/// Return whether a proleptic Gregorian calendar year is a leap year.
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Return the first day of the date's month.
pub fn start_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
        .expect("first day of a valid month must exist")
}

/// Return the last day of the date's month.
pub fn end_of_month(date: NaiveDate) -> NaiveDate {
    let last_day = match date.month() {
        2 if is_leap_year(date.year()) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    date.with_day(last_day)
        .expect("last day of a valid month must exist")
}

/// Return January 1 of the date's year.
pub fn start_of_year(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), 1, 1)
        .expect("first day of a valid year must exist")
}

/// Return December 31 of the date's year.
pub fn end_of_year(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), 12, 31)
        .expect("last day of a valid year must exist")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn knows_gregorian_leap_year_rules() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(0));
    }

    #[test]
    fn finds_month_boundaries() {
        assert_eq!(start_of_month(date(2024, 2, 29)), date(2024, 2, 1));
        assert_eq!(end_of_month(date(2024, 2, 1)), date(2024, 2, 29));
        assert_eq!(end_of_month(date(2023, 2, 1)), date(2023, 2, 28));
        assert_eq!(end_of_month(date(2024, 4, 30)), date(2024, 4, 30));
    }

    #[test]
    fn finds_year_boundaries_even_at_supported_limits() {
        assert_eq!(start_of_year(date(2024, 12, 31)), date(2024, 1, 1));
        assert_eq!(end_of_year(date(2024, 1, 1)), date(2024, 12, 31));
        assert_eq!(start_of_month(NaiveDate::MIN), NaiveDate::MIN);
        assert_eq!(end_of_month(NaiveDate::MAX), NaiveDate::MAX);
        assert_eq!(start_of_year(NaiveDate::MIN), NaiveDate::MIN);
        assert_eq!(end_of_year(NaiveDate::MAX), NaiveDate::MAX);
    }
}
