//! Checked calendar-day and calendar-week arithmetic.

use chrono::{Days, NaiveDate};

/// Add signed calendar days, returning `None` on date-range overflow.
pub fn add_days(date: NaiveDate, days: i64) -> Option<NaiveDate> {
    if days >= 0 {
        date.checked_add_days(Days::new(days as u64))
    } else {
        date.checked_sub_days(Days::new(days.unsigned_abs()))
    }
}

/// Subtract signed calendar days, returning `None` on date-range overflow.
pub fn sub_days(date: NaiveDate, days: i64) -> Option<NaiveDate> {
    if days >= 0 {
        date.checked_sub_days(Days::new(days as u64))
    } else {
        date.checked_add_days(Days::new(days.unsigned_abs()))
    }
}

/// Add signed calendar weeks, returning `None` on overflow.
pub fn add_weeks(date: NaiveDate, weeks: i64) -> Option<NaiveDate> {
    add_days(date, weeks.checked_mul(7)?)
}

/// Subtract signed calendar weeks, returning `None` on overflow.
pub fn sub_weeks(date: NaiveDate, weeks: i64) -> Option<NaiveDate> {
    sub_days(date, weeks.checked_mul(7)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn shifts_across_leap_days_and_years() {
        assert_eq!(add_days(date(2024, 2, 28), 1), Some(date(2024, 2, 29)));
        assert_eq!(sub_days(date(2024, 3, 1), 1), Some(date(2024, 2, 29)));
        assert_eq!(add_days(date(2024, 1, 1), -1), Some(date(2023, 12, 31)));
        assert_eq!(sub_days(date(2024, 1, 1), -1), Some(date(2024, 1, 2)));
    }

    #[test]
    fn shifts_calendar_weeks() {
        assert_eq!(add_weeks(date(2024, 2, 26), 1), Some(date(2024, 3, 4)));
        assert_eq!(sub_weeks(date(2024, 3, 4), 1), Some(date(2024, 2, 26)));
        assert_eq!(add_weeks(date(2024, 3, 4), -1), Some(date(2024, 2, 26)));
    }

    #[test]
    fn returns_none_instead_of_overflowing() {
        assert_eq!(add_days(NaiveDate::MAX, 1), None);
        assert_eq!(sub_days(NaiveDate::MIN, 1), None);
        assert_eq!(add_days(NaiveDate::MIN, i64::MIN), None);
        assert_eq!(sub_days(NaiveDate::MAX, i64::MIN), None);
        assert_eq!(add_weeks(date(2024, 1, 1), i64::MAX), None);
        assert_eq!(sub_weeks(date(2024, 1, 1), i64::MIN), None);
        assert_eq!(add_days(NaiveDate::MAX, 0), Some(NaiveDate::MAX));
    }
}
