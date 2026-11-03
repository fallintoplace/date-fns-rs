//! Week numbers within fiscal years with configurable start months.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return the one-based week number within a fiscal year.
///
/// Partial weeks at the start of the fiscal year count as week one.
/// Invalid start months or out-of-range fiscal boundaries return None.
pub fn get_week_of_fiscal_year(
    date: NaiveDate,
    first_month: u32,
    week_starts_on: Weekday,
) -> Option<u32> {
    let start = crate::start_of_fiscal_year(date, first_month)?;
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
    fn follows_april_to_march_fiscal_years() {
        assert_eq!(
            get_week_of_fiscal_year(day(2024, 4, 1), 4, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_year(day(2024, 4, 7), 4, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            get_week_of_fiscal_year(day(2024, 4, 8), 4, Weekday::Mon),
            Some(2)
        );
        assert_eq!(
            get_week_of_fiscal_year(day(2024, 4, 7), 4, Weekday::Sun),
            Some(2)
        );
        assert_eq!(
            get_week_of_fiscal_year(day(2025, 3, 31), 4, Weekday::Mon),
            Some(53)
        );
        assert_eq!(
            get_week_of_fiscal_year(day(2025, 4, 1), 4, Weekday::Mon),
            Some(1)
        );
    }

    #[test]
    fn rejects_bad_months_and_unrepresentable_year_starts() {
        let date = day(2024, 3, 1);
        assert_eq!(get_week_of_fiscal_year(date, 0, Weekday::Mon), None);
        assert_eq!(get_week_of_fiscal_year(date, 13, Weekday::Mon), None);
        assert_eq!(
            get_week_of_fiscal_year(NaiveDate::MIN, 2, Weekday::Mon),
            None
        );
        assert!(get_week_of_fiscal_year(NaiveDate::MAX, 1, Weekday::Sun).is_some());
    }
}
