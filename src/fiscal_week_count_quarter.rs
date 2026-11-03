//! Count calendar weeks in fiscal quarters, including partial weeks.

use chrono::{NaiveDate, Weekday};

/// Return the number of calendar weeks touched by the containing fiscal quarter.
///
/// Partial weeks at the beginning and end count as weeks. `week_starts_on`
/// controls where each week begins. Returns None for invalid fiscal
/// months or fiscal boundaries outside Chrono's supported range.
pub fn get_weeks_in_fiscal_quarter(
    date: NaiveDate,
    first_month: u32,
    week_starts_on: Weekday,
) -> Option<u32> {
    let last = crate::end_of_fiscal_quarter(date, first_month)?;
    crate::get_week_of_fiscal_quarter(last, first_month, week_starts_on)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn counts_partial_weeks_with_configurable_week_starts() {
        let date = day(2023, 2, 1);
        assert_eq!(get_weeks_in_fiscal_quarter(date, 1, Weekday::Mon), Some(14));
        assert_eq!(get_weeks_in_fiscal_quarter(date, 1, Weekday::Sun), Some(13));
        let last = crate::end_of_fiscal_quarter(date, 1).unwrap();
        assert_eq!(
            get_weeks_in_fiscal_quarter(date, 1, Weekday::Mon),
            crate::get_week_of_fiscal_quarter(last, 1, Weekday::Mon)
        );
    }

    #[test]
    fn rejects_invalid_months_and_out_of_range_boundaries() {
        let date = day(2023, 2, 1);
        assert_eq!(get_weeks_in_fiscal_quarter(date, 0, Weekday::Mon), None);
        assert_eq!(get_weeks_in_fiscal_quarter(date, 13, Weekday::Sun), None);
        assert_eq!(
            get_weeks_in_fiscal_quarter(NaiveDate::MAX, 2, Weekday::Mon),
            None
        );
        assert!(get_weeks_in_fiscal_quarter(NaiveDate::MAX, 1, Weekday::Mon).is_some());
    }
}
