//! Relative fiscal half-year comparisons with a configurable first month.

use chrono::NaiveDate;

/// Return whether target is in the next fiscal half-year.
///
/// first_month must be between 1 and 12.
pub fn is_next_fiscal_half_year(
    target: NaiveDate,
    reference: NaiveDate,
    first_month: u32,
) -> Option<bool> {
    Some(crate::difference_in_calendar_fiscal_half_years(target, reference, first_month)? == 1)
}

/// Return whether target is in the previous fiscal half-year.
pub fn is_previous_fiscal_half_year(
    target: NaiveDate,
    reference: NaiveDate,
    first_month: u32,
) -> Option<bool> {
    Some(crate::difference_in_calendar_fiscal_half_years(target, reference, first_month)? == -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn follows_april_and_october_fiscal_boundaries() {
        let march = date(2024, 3, 31);
        let april = date(2024, 4, 1);
        let september = date(2024, 9, 30);
        let october = date(2024, 10, 1);
        assert_eq!(is_next_fiscal_half_year(april, march, 4), Some(true));
        assert_eq!(is_next_fiscal_half_year(october, september, 4), Some(true));
        assert_eq!(is_previous_fiscal_half_year(march, april, 4), Some(true));
        assert_eq!(is_previous_fiscal_half_year(september, october, 4), Some(true));
        assert_eq!(is_next_fiscal_half_year(october, march, 4), Some(false));
    }

    #[test]
    fn handles_calendar_new_years_and_invalid_months() {
        let december = date(2024, 12, 31);
        let january = date(2025, 1, 1);
        assert_eq!(is_next_fiscal_half_year(january, december, 1), Some(true));
        assert_eq!(is_next_fiscal_half_year(january, december, 4), Some(false));
        assert_eq!(is_next_fiscal_half_year(january, december, 0), None);
        assert_eq!(is_previous_fiscal_half_year(december, january, 13), None);
        assert_eq!(
            is_next_fiscal_half_year(NaiveDate::MAX, NaiveDate::MIN, 4),
            Some(false)
        );
    }
}
