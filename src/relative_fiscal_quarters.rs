//! Relative fiscal-quarter predicates with a caller-chosen first month.

use chrono::NaiveDate;

/// Return whether target falls in the fiscal quarter after reference.
///
/// first_month must be between 1 and 12.
pub fn is_next_fiscal_quarter(
    target: NaiveDate,
    reference: NaiveDate,
    first_month: u32,
) -> Option<bool> {
    Some(crate::difference_in_calendar_fiscal_quarters(target, reference, first_month)? == 1)
}

/// Return whether target falls in the fiscal quarter before reference.
pub fn is_previous_fiscal_quarter(
    target: NaiveDate,
    reference: NaiveDate,
    first_month: u32,
) -> Option<bool> {
    Some(crate::difference_in_calendar_fiscal_quarters(target, reference, first_month)? == -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn compares_april_fiscal_quarters_at_boundary() {
        let june = date(2024, 6, 30);
        let july = date(2024, 7, 1);
        assert_eq!(is_next_fiscal_quarter(july, june, 4), Some(true));
        assert_eq!(is_previous_fiscal_quarter(june, july, 4), Some(true));
        assert_eq!(is_next_fiscal_quarter(june, june, 4), Some(false));
        assert_eq!(is_next_fiscal_quarter(date(2025, 1, 1), june, 4), Some(false));
    }

    #[test]
    fn handles_custom_start_month_and_invalid_configuration() {
        let january = date(2024, 1, 31);
        let february = date(2024, 2, 1);
        assert_eq!(is_next_fiscal_quarter(february, january, 2), Some(true));
        assert_eq!(is_next_fiscal_quarter(february, january, 4), Some(false));
        assert_eq!(is_next_fiscal_quarter(february, january, 0), None);
        assert_eq!(is_previous_fiscal_quarter(february, january, 13), None);
    }

    #[test]
    fn includes_negative_years_and_chrono_limits() {
        assert_eq!(
            is_next_fiscal_quarter(date(0, 1, 1), date(-1, 12, 31), 1),
            Some(true)
        );
        assert_eq!(
            is_previous_fiscal_quarter(NaiveDate::MIN, NaiveDate::MAX, 7),
            Some(false)
        );
    }
}
