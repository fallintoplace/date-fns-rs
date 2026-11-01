//! Predicates for calendar half-year comparisons and boundaries.

use chrono::{Datelike, NaiveDate};

/// Whether two dates belong to the same calendar half-year.
pub fn is_same_half_year(left: NaiveDate, right: NaiveDate) -> bool {
    left.year() == right.year() && (left.month() - 1) / 6 == (right.month() - 1) / 6
}

/// Whether date is January 1 or July 1.
pub fn is_first_day_of_half_year(date: NaiveDate) -> bool {
    matches!(date.month(), 1 | 7) && date.day() == 1
}

/// Whether date is June 30 or December 31.
pub fn is_last_day_of_half_year(date: NaiveDate) -> bool {
    (date.month() == 6 && date.day() == 30) || (date.month() == 12 && date.day() == 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn checks_half_year_equality_and_boundaries() {
        assert!(is_same_half_year(date(2024, 1, 1), date(2024, 6, 30)));
        assert!(!is_same_half_year(date(2024, 6, 30), date(2024, 7, 1)));
        assert!(!is_same_half_year(date(2024, 7, 1), date(2025, 7, 1)));
        assert!(is_first_day_of_half_year(date(2024, 1, 1)));
        assert!(is_first_day_of_half_year(date(2024, 7, 1)));
        assert!(!is_first_day_of_half_year(date(2024, 7, 2)));
        assert!(is_last_day_of_half_year(date(2024, 6, 30)));
        assert!(is_last_day_of_half_year(date(2024, 12, 31)));
        assert!(!is_last_day_of_half_year(date(2024, 6, 29)));
    }

    #[test]
    fn works_on_supported_date_limits() {
        assert!(is_first_day_of_half_year(NaiveDate::MIN));
        assert!(is_last_day_of_half_year(NaiveDate::MAX));
    }
}
