//! Checked calendar-quarter arithmetic with month-end clamping.

use chrono::NaiveDate;

use crate::months::{add_months, sub_months};

/// Add signed calendar quarters, clamping the day of month if necessary.
///
/// Returns None if the quarter-to-month conversion or date range overflows.
pub fn add_quarters(date: NaiveDate, quarters: i32) -> Option<NaiveDate> {
    add_months(date, quarters.checked_mul(3)?)
}

/// Subtract signed calendar quarters, clamping the day of month if necessary.
pub fn sub_quarters(date: NaiveDate, quarters: i32) -> Option<NaiveDate> {
    sub_months(date, quarters.checked_mul(3)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn quarter_offsets_cross_years_and_clamp_month_ends() {
        assert_eq!(add_quarters(date(2024, 1, 31), 1), Some(date(2024, 4, 30)));
        assert_eq!(sub_quarters(date(2024, 5, 31), 1), Some(date(2024, 2, 29)));
        assert_eq!(add_quarters(date(2024, 11, 30), 1), Some(date(2025, 2, 28)));
        assert_eq!(add_quarters(date(2024, 2, 29), 4), Some(date(2025, 2, 28)));
    }

    #[test]
    fn negative_and_zero_offsets_are_supported() {
        let start = date(2024, 5, 15);
        assert_eq!(add_quarters(start, -1), Some(date(2024, 2, 15)));
        assert_eq!(sub_quarters(start, -1), Some(date(2024, 8, 15)));
        assert_eq!(sub_quarters(start, 0), Some(start));
    }

    #[test]
    fn excessive_quarters_never_wrap() {
        assert_eq!(add_quarters(date(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_quarters(date(2024, 1, 1), i32::MIN), None);
        assert_eq!(add_quarters(NaiveDate::MAX, 1), None);
        assert_eq!(sub_quarters(NaiveDate::MIN, 1), None);
    }
}
