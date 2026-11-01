//! Checked date setters and Gregorian date existence checks.

use chrono::{Datelike, NaiveDate, Weekday};

/// Set a day of the current month without overflowing into the next month.
/// Returns None for day zero or a day beyond the month's length.
pub fn set_day_of_month(date: NaiveDate, day: u32) -> Option<NaiveDate> {
    date.with_day(day)
}

/// Set a weekday index from 0 through 6 within the week containing date.
/// Index zero means week_starts_on, not necessarily Sunday.
pub fn set_day_of_week(
    date: NaiveDate,
    index: u32,
    week_starts_on: Weekday,
) -> Option<NaiveDate> {
    if index > 6 {
        return None;
    }
    let from_monday = date.weekday().num_days_from_monday() as i64;
    let start_from_monday = week_starts_on.num_days_from_monday() as i64;
    let days_since_start = (from_monday - start_from_monday).rem_euclid(7);
    let first = crate::add_days(date, -days_since_start)?;
    crate::add_days(first, i64::from(index))
}

/// Check whether the given year-month-day represents a valid Gregorian date.
pub fn is_existing_calendar_date(year: i32, month: u32, day: u32) -> bool {
    NaiveDate::from_ymd_opt(year, month, day).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_leap_month_limits() {
        let february = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(set_day_of_month(february, 29), NaiveDate::from_ymd_opt(2024, 2, 29));
        assert_eq!(set_day_of_month(february, 30), None);
        assert_eq!(set_day_of_month(february, 0), None);
        assert!(is_existing_calendar_date(2024, 2, 29));
        assert!(!is_existing_calendar_date(2023, 2, 29));
        assert!(!is_existing_calendar_date(2024, 13, 1));
    }

    #[test]
    fn changes_weekday_relative_to_custom_week_start() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(set_day_of_week(monday, 0, Weekday::Mon), Some(monday));
        assert_eq!(set_day_of_week(monday, 0, Weekday::Sun), NaiveDate::from_ymd_opt(2023, 12, 31));
        assert_eq!(set_day_of_week(monday, 6, Weekday::Sun), NaiveDate::from_ymd_opt(2024, 1, 6));
        assert_eq!(set_day_of_week(monday, 7, Weekday::Mon), None);
        assert_eq!(set_day_of_week(NaiveDate::MIN, 0, Weekday::Mon).is_some(), NaiveDate::MIN.weekday() == Weekday::Mon);
    }
}