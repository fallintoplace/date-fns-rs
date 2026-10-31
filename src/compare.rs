//! Date-only comparison predicates.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return whether `left` is strictly before `right`.
pub fn is_before(left: NaiveDate, right: NaiveDate) -> bool {
    left < right
}

/// Return whether `left` is strictly after `right`.
pub fn is_after(left: NaiveDate, right: NaiveDate) -> bool {
    left > right
}

/// Return whether two dates represent the same calendar day.
pub fn is_same_day(left: NaiveDate, right: NaiveDate) -> bool {
    left == right
}

/// Return whether two dates have the same calendar month and year.
pub fn is_same_month(left: NaiveDate, right: NaiveDate) -> bool {
    left.year() == right.year() && left.month() == right.month()
}

/// Return whether two dates have the same calendar year.
pub fn is_same_year(left: NaiveDate, right: NaiveDate) -> bool {
    left.year() == right.year()
}

/// Return whether a date is Saturday or Sunday.
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn strict_comparisons_exclude_equal_dates() {
        let first = date(2024, 1, 1);
        let next = date(2024, 1, 2);
        assert!(is_before(first, next));
        assert!(is_after(next, first));
        assert!(!is_before(first, first));
        assert!(!is_after(first, first));
        assert!(is_same_day(first, first));
        assert!(!is_same_day(first, next));
    }

    #[test]
    fn month_and_year_comparisons_include_years() {
        let first = date(2024, 1, 1);
        assert!(is_same_month(first, date(2024, 1, 31)));
        assert!(!is_same_month(first, date(2025, 1, 1)));
        assert!(!is_same_month(first, date(2024, 2, 1)));
        assert!(is_same_year(first, date(2024, 12, 31)));
        assert!(!is_same_year(first, date(2025, 1, 1)));
    }

    #[test]
    fn identifies_saturdays_and_sundays() {
        assert!(is_weekend(date(2024, 6, 1)));
        assert!(is_weekend(date(2024, 6, 2)));
        assert!(!is_weekend(date(2024, 6, 3)));
    }
}
