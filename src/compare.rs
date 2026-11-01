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

/// Return the earliest date in a slice, or None for an empty slice.
pub fn min_date(dates: &[NaiveDate]) -> Option<NaiveDate> {
    dates.iter().copied().min()
}

/// Return the latest date in a slice, or None for an empty slice.
pub fn max_date(dates: &[NaiveDate]) -> Option<NaiveDate> {
    dates.iter().copied().max()
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
    #[test]
    fn finds_extrema_without_modifying_candidates() {
        let input = [date(2024, 2, 29), date(2023, 12, 31), date(2025, 1, 1)];
        assert_eq!(min_date(&input), Some(date(2023, 12, 31)));
        assert_eq!(max_date(&input), Some(date(2025, 1, 1)));
        assert_eq!(input[0], date(2024, 2, 29));
    }

    #[test]
    fn handles_empty_singleton_and_extreme_dates() {
        assert_eq!(min_date(&[]), None);
        assert_eq!(max_date(&[]), None);
        assert_eq!(min_date(&[NaiveDate::MAX]), Some(NaiveDate::MAX));
        assert_eq!(
            max_date(&[NaiveDate::MAX, NaiveDate::MIN]),
            Some(NaiveDate::MAX)
        );
    }
}
