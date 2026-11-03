//! Week numbers in fiscal half-years, with explicit partial weeks.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return the one-based week of the current fiscal half-year.
///
/// The fiscal year starts in first_month (1 through 12). A partial
/// first week counts as week one. Returns None if its start is outside
/// the supported date range or first_month is invalid.
pub fn get_week_of_fiscal_half_year(
    date: NaiveDate,
    first_month: u32,
    week_starts_on: Weekday,
) -> Option<u32> {
    let start = crate::start_of_fiscal_half_year(date, first_month)?;
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
    fn numbers_partial_weeks_from_april_and_october() {
        assert_eq!(
            get_week_of_fiscal_half_year(day(2024, 4, 1), 4, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_half_year(day(2024, 4, 7), 4, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_half_year(day(2024, 4, 8), 4, Weekday::Mon),
            Some(2)
        );
        assert_eq!(
            get_week_of_fiscal_half_year(day(2024, 4, 7), 4, Weekday::Sun),
            Some(2)
        );
        assert_eq!(
            get_week_of_fiscal_half_year(day(2024, 10, 1), 4, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_half_year(day(2024, 10, 7), 4, Weekday::Mon),
            Some(2)
        );
    }

    #[test]
    fn rejects_invalid_months_and_handles_chrono_limits() {
        let date = day(2024, 6, 1);
        assert_eq!(get_week_of_fiscal_half_year(date, 0, Weekday::Mon), None);
        assert_eq!(get_week_of_fiscal_half_year(date, 13, Weekday::Mon), None);
        assert_eq!(
            get_week_of_fiscal_half_year(NaiveDate::MIN, 2, Weekday::Mon),
            None
        );
        assert!(get_week_of_fiscal_half_year(NaiveDate::MAX, 1, Weekday::Mon).is_some());
    }
}
