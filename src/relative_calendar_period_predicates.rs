//! Relative calendar-month, quarter, and year comparisons.

use chrono::{Datelike, NaiveDate};

fn month_index(date: NaiveDate) -> i64 {
    i64::from(date.year()) * 12 + i64::from(date.month0())
}

fn quarter_index(date: NaiveDate) -> i64 {
    i64::from(date.year()) * 4 + i64::from(date.month0() / 3)
}

/// Return whether the target belongs to the next calendar month.
pub fn is_next_calendar_month(target: NaiveDate, reference: NaiveDate) -> bool {
    month_index(target) - month_index(reference) == 1
}

/// Return whether the target belongs to the previous calendar month.
pub fn is_previous_calendar_month(target: NaiveDate, reference: NaiveDate) -> bool {
    month_index(target) - month_index(reference) == -1
}

/// Return whether the target belongs to the next calendar quarter.
pub fn is_next_calendar_quarter(target: NaiveDate, reference: NaiveDate) -> bool {
    quarter_index(target) - quarter_index(reference) == 1
}

/// Return whether the target belongs to the previous calendar quarter.
pub fn is_previous_calendar_quarter(target: NaiveDate, reference: NaiveDate) -> bool {
    quarter_index(target) - quarter_index(reference) == -1
}

/// Return whether the target belongs to the next calendar year.
pub fn is_next_calendar_year(target: NaiveDate, reference: NaiveDate) -> bool {
    i64::from(target.year()) - i64::from(reference.year()) == 1
}

/// Return whether the target belongs to the previous calendar year.
pub fn is_previous_calendar_year(target: NaiveDate, reference: NaiveDate) -> bool {
    i64::from(target.year()) - i64::from(reference.year()) == -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn recognizes_month_quarter_and_year_boundaries() {
        let end = date(2024, 12, 31);
        let next = date(2025, 1, 1);
        assert!(is_next_calendar_month(next, end));
        assert!(is_previous_calendar_month(end, next));
        assert!(is_next_calendar_quarter(next, end));
        assert!(is_previous_calendar_quarter(end, next));
        assert!(is_next_calendar_year(next, end));
        assert!(is_previous_calendar_year(end, next));
        assert!(!is_next_calendar_month(date(2025, 2, 1), end));
        assert!(!is_next_calendar_quarter(date(2025, 4, 1), end));
        assert!(!is_next_calendar_year(date(2026, 1, 1), end));
    }

    #[test]
    fn handles_negative_years_and_chrono_extremes() {
        let zero = date(0, 1, 1);
        let previous = date(-1, 12, 31);
        assert!(is_next_calendar_month(zero, previous));
        assert!(is_next_calendar_year(zero, previous));
        assert!(!is_next_calendar_year(NaiveDate::MIN, NaiveDate::MAX));
        assert!(!is_previous_calendar_month(NaiveDate::MAX, NaiveDate::MIN));
    }
}
