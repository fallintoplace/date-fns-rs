//! One-based day positions and lengths of configurable fiscal half-years.

use chrono::NaiveDate;

/// Return the one-based day within the fiscal half-year containing date.
///
/// Returns None for invalid first months or an out-of-range start boundary.
pub fn get_day_of_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    let start = crate::start_of_fiscal_half_year(date, first_month)?;
    u32::try_from(date.signed_duration_since(start).num_days() + 1).ok()
}

/// Return the complete length of the fiscal half-year in calendar days.
///
/// Leap years and fiscal halves that cross the Gregorian year are supported.
pub fn get_days_in_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    let start = crate::start_of_fiscal_half_year(date, first_month)?;
    let end = crate::end_of_fiscal_half_year(date, first_month)?;
    u32::try_from(end.signed_duration_since(start).num_days() + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_october_to_march_halves_including_leap_day() {
        assert_eq!(get_day_of_fiscal_half_year(day(2024, 2, 29), 4), Some(152));
        assert_eq!(get_days_in_fiscal_half_year(day(2024, 2, 29), 4), Some(183));
        assert_eq!(get_day_of_fiscal_half_year(day(2024, 3, 31), 4), Some(183));
        assert_eq!(get_day_of_fiscal_half_year(day(2024, 4, 1), 4), Some(1));
        assert_eq!(get_days_in_fiscal_half_year(day(2024, 4, 1), 4), Some(183));
        assert_eq!(get_days_in_fiscal_half_year(day(2024, 2, 29), 9), Some(182));
    }

    #[test]
    fn validates_months_and_handles_date_limits() {
        let date = day(2024, 6, 1);
        assert_eq!(get_day_of_fiscal_half_year(date, 0), None);
        assert_eq!(get_days_in_fiscal_half_year(date, 13), None);
        assert_eq!(get_day_of_fiscal_half_year(NaiveDate::MIN, 2), None);
        assert!(get_days_in_fiscal_half_year(NaiveDate::MAX, 1).is_some());
    }
}
