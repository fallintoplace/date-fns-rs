//! Week numbers within configurable fiscal quarters.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return the one-based week number of the current fiscal quarter.
///
/// An incomplete first week counts as week one. Returns None if the
/// fiscal start month is invalid or the quarter start is unrepresentable.
pub fn get_week_of_fiscal_quarter(
    date: NaiveDate,
    first_month: u32,
    week_starts_on: Weekday,
) -> Option<u32> {
    let start = crate::start_of_fiscal_quarter(date, first_month)?;
    let elapsed = date.signed_duration_since(start).num_days();
    let offset = i64::from(
        (start.weekday().num_days_from_monday() + 7 - week_starts_on.num_days_from_monday()) % 7,
    );
    u32::try_from((elapsed + offset) / 7 + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_partial_weeks_and_february_first_fiscal_quarters() {
        assert_eq!(
            get_week_of_fiscal_quarter(day(2024, 2, 1), 2, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_quarter(day(2024, 2, 4), 2, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_quarter(day(2024, 2, 5), 2, Weekday::Mon),
            Some(2)
        );
        assert_eq!(
            get_week_of_fiscal_quarter(day(2024, 2, 4), 2, Weekday::Sun),
            Some(2)
        );
        assert_eq!(
            get_week_of_fiscal_quarter(day(2024, 4, 30), 2, Weekday::Mon),
            Some(14)
        );
        assert_eq!(
            get_week_of_fiscal_quarter(day(2024, 5, 1), 2, Weekday::Mon),
            Some(1)
        );
    }

    #[test]
    fn validates_first_month_and_date_limits() {
        let date = day(2024, 6, 15);
        assert_eq!(get_week_of_fiscal_quarter(date, 0, Weekday::Mon), None);
        assert_eq!(get_week_of_fiscal_quarter(date, 13, Weekday::Mon), None);
        assert_eq!(
            get_week_of_fiscal_quarter(NaiveDate::MIN, 2, Weekday::Mon),
            None
        );
        assert!(get_week_of_fiscal_quarter(NaiveDate::MAX, 1, Weekday::Sun).is_some());
    }
}
