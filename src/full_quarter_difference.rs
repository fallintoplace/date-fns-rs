//! Signed complete-quarter differences between calendar dates.

use chrono::NaiveDate;

use crate::full_month_difference::difference_in_months;

/// Count complete three-month periods between dates.
///
/// Unlike a difference in calendar-quarter labels, this requires a full
/// three-month offset with the same month-end clamping as add_quarters.
pub fn difference_in_quarters(left: NaiveDate, right: NaiveDate) -> i64 {
    difference_in_months(left, right) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn requires_complete_three_month_periods() {
        assert_eq!(
            difference_in_quarters(date(2024, 4, 30), date(2024, 1, 31)),
            1
        );
        assert_eq!(
            difference_in_quarters(date(2024, 4, 29), date(2024, 1, 31)),
            0
        );
        assert_eq!(
            difference_in_quarters(date(2024, 7, 1), date(2024, 3, 31)),
            1
        );
        assert_eq!(
            difference_in_quarters(date(2024, 1, 31), date(2024, 4, 30)),
            -1
        );
    }

    #[test]
    fn is_signed_and_safe_at_date_limits() {
        let span = difference_in_quarters(NaiveDate::MAX, NaiveDate::MIN);
        assert!(span > 0);
        assert_eq!(
            difference_in_quarters(NaiveDate::MIN, NaiveDate::MAX),
            -span
        );
        assert_eq!(difference_in_quarters(NaiveDate::MAX, NaiveDate::MAX), 0);
    }
}
