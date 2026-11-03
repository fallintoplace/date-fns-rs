//! Fiscal quarter boundary predicates for date-only values.

use chrono::NaiveDate;

/// Check whether date is the first day of its fiscal quarter.
///
/// The first fiscal month must be 1 through 12. Invalid or
/// unrepresentable period boundaries return None.
pub fn is_first_day_of_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<bool> {
    Some(date == crate::start_of_fiscal_quarter(date, first_month)?)
}

/// Check whether date is the last day of its fiscal quarter.
///
/// Returns None when the fiscal quarter end is out of range.
pub fn is_last_day_of_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<bool> {
    Some(date == crate::end_of_fiscal_quarter(date, first_month)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn detects_fiscal_boundaries_across_calendar_years() {
        assert_eq!(
            is_first_day_of_fiscal_quarter(day(2023, 11, 1), 2),
            Some(true)
        );
        assert_eq!(
            is_last_day_of_fiscal_quarter(day(2024, 1, 31), 2),
            Some(true)
        );
        assert_eq!(
            is_first_day_of_fiscal_quarter(day(2023, 10, 31), 2),
            Some(false)
        );
        assert_eq!(
            is_last_day_of_fiscal_quarter(day(2024, 2, 1), 2),
            Some(false)
        );
        assert_eq!(
            is_first_day_of_fiscal_quarter(day(2024, 1, 31), 2),
            Some(false)
        );
        assert_eq!(
            is_last_day_of_fiscal_quarter(day(2023, 11, 1), 2),
            Some(false)
        );
    }

    #[test]
    fn rejects_invalid_months_and_observes_date_limits() {
        let date = day(2023, 11, 1);
        assert_eq!(is_first_day_of_fiscal_quarter(date, 0), None);
        assert_eq!(is_last_day_of_fiscal_quarter(date, 13), None);
        assert_eq!(
            is_first_day_of_fiscal_quarter(NaiveDate::MIN, 1),
            Some(true)
        );
        assert_eq!(is_last_day_of_fiscal_quarter(NaiveDate::MAX, 1), Some(true));
        assert_eq!(is_last_day_of_fiscal_quarter(NaiveDate::MAX, 2), None);
    }
}
