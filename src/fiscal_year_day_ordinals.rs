//! One-based day positions and lengths of configurable fiscal years.

use chrono::NaiveDate;

/// Return the one-based day position within a fiscal year.
///
/// The first month must be 1 through 12. Returns None when the fiscal
/// start cannot be represented by Chrono.
pub fn get_day_of_fiscal_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    let start = crate::start_of_fiscal_year(date, first_month)?;
    u32::try_from(date.signed_duration_since(start).num_days() + 1).ok()
}

/// Return the full fiscal-year length, including a leap day when present.
///
/// Returns None for invalid first months or unrepresentable boundaries.
pub fn get_days_in_fiscal_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    let start = crate::start_of_fiscal_year(date, first_month)?;
    let end = crate::end_of_fiscal_year(date, first_month)?;
    u32::try_from(end.signed_duration_since(start).num_days() + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_april_to_march_years_across_leap_day() {
        assert_eq!(get_day_of_fiscal_year(day(2024, 2, 29), 4), Some(335));
        assert_eq!(get_days_in_fiscal_year(day(2024, 2, 29), 4), Some(366));
        assert_eq!(get_day_of_fiscal_year(day(2024, 3, 31), 4), Some(366));
        assert_eq!(get_day_of_fiscal_year(day(2024, 4, 1), 4), Some(1));
        assert_eq!(get_days_in_fiscal_year(day(2024, 4, 1), 4), Some(365));
        assert_eq!(get_day_of_fiscal_year(day(2024, 2, 29), 1), Some(60));
    }

    #[test]
    fn rejects_invalid_months_and_unrepresentable_boundaries() {
        let date = day(2024, 5, 6);
        assert_eq!(get_day_of_fiscal_year(date, 0), None);
        assert_eq!(get_days_in_fiscal_year(date, 13), None);
        assert_eq!(get_day_of_fiscal_year(NaiveDate::MIN, 2), None);
        assert!(get_day_of_fiscal_year(NaiveDate::MAX, 1).is_some());
    }
}
