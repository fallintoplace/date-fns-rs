//! Gregorian quarter and fiscal half-year boundary predicates.

use chrono::NaiveDate;

/// Return whether a date begins its calendar quarter.
pub fn is_first_day_of_quarter(date: NaiveDate) -> bool {
    date == crate::start_of_quarter(date)
}

/// Return whether a date ends its calendar quarter.
pub fn is_last_day_of_quarter(date: NaiveDate) -> bool {
    date == crate::end_of_quarter(date)
}

/// Return whether date starts a fiscal half-year with the configured first month.
///
/// Returns None for invalid months or unrepresentable fiscal boundaries.
pub fn is_first_day_of_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<bool> {
    Some(date == crate::start_of_fiscal_half_year(date, first_month)?)
}

/// Return whether date ends a fiscal half-year with the configured first month.
pub fn is_last_day_of_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<bool> {
    Some(date == crate::end_of_fiscal_half_year(date, first_month)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn distinguishes_calendar_quarter_boundaries() {
        assert!(is_first_day_of_quarter(date(2024, 4, 1)));
        assert!(is_last_day_of_quarter(date(2024, 3, 31)));
        assert!(is_last_day_of_quarter(date(2024, 6, 30)));
        assert!(!is_last_day_of_quarter(date(2024, 2, 29)));
        assert!(!is_first_day_of_quarter(date(2024, 2, 1)));
    }

    #[test]
    fn supports_april_fiscal_half_years() {
        assert_eq!(
            is_first_day_of_fiscal_half_year(date(2024, 4, 1), 4),
            Some(true)
        );
        assert_eq!(
            is_last_day_of_fiscal_half_year(date(2024, 9, 30), 4),
            Some(true)
        );
        assert_eq!(
            is_first_day_of_fiscal_half_year(date(2024, 10, 1), 4),
            Some(true)
        );
        assert_eq!(
            is_last_day_of_fiscal_half_year(date(2025, 3, 31), 4),
            Some(true)
        );
        assert_eq!(
            is_first_day_of_fiscal_half_year(date(2024, 1, 1), 4),
            Some(false)
        );
        assert_eq!(is_first_day_of_fiscal_half_year(date(2024, 1, 1), 0), None);
        assert_eq!(is_last_day_of_fiscal_half_year(date(2024, 1, 1), 13), None);
    }

    #[test]
    fn handles_supported_calendar_limits() {
        assert!(is_first_day_of_quarter(NaiveDate::MIN));
        assert!(is_last_day_of_quarter(NaiveDate::MAX));
    }
}
