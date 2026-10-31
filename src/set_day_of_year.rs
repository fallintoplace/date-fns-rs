//! Checked ordinal-day replacement within a calendar year.

use chrono::{Datelike, NaiveDate};

/// Replace a date's 1-based day of year while keeping the same year.
///
/// Returns None for zero, day 366 in non-leap years, or any ordinal beyond
/// the year's length. The Gregorian year is never silently changed.
pub fn set_day_of_year(date: NaiveDate, day_of_year: u32) -> Option<NaiveDate> {
    NaiveDate::from_yo_opt(date.year(), day_of_year)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn sets_ordinal_days_in_leap_years() {
        let input = date(2024, 7, 15);
        assert_eq!(set_day_of_year(input, 1), Some(date(2024, 1, 1)));
        assert_eq!(set_day_of_year(input, 60), Some(date(2024, 2, 29)));
        assert_eq!(set_day_of_year(input, 366), Some(date(2024, 12, 31)));
    }

    #[test]
    fn validates_ordinals_in_common_years() {
        let input = date(2023, 2, 15);
        assert_eq!(set_day_of_year(input, 59), Some(date(2023, 2, 28)));
        assert_eq!(set_day_of_year(input, 60), Some(date(2023, 3, 1)));
        assert_eq!(set_day_of_year(input, 365), Some(date(2023, 12, 31)));
        assert_eq!(set_day_of_year(input, 0), None);
        assert_eq!(set_day_of_year(input, 366), None);
        assert_eq!(set_day_of_year(input, u32::MAX), None);
    }

    #[test]
    fn preserves_extreme_dates_when_ordinal_is_unchanged() {
        let first = NaiveDate::MIN;
        let last = NaiveDate::MAX;
        assert_eq!(set_day_of_year(first, first.ordinal()), Some(first));
        assert_eq!(set_day_of_year(last, last.ordinal()), Some(last));
    }
}
