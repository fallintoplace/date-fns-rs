//! Signed complete-year differences for calendar dates.

use chrono::NaiveDate;

use crate::full_month_difference::difference_in_months;

/// Count complete calendar-year offsets from right to left.
///
/// February 29 is clamped to February 28 in non-leap target years,
/// matching add_years. Incomplete years truncate toward zero.
pub fn difference_in_years(left: NaiveDate, right: NaiveDate) -> i64 {
    difference_in_months(left, right) / 12
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn leap_day_anniversaries_are_clamped() {
        assert_eq!(
            difference_in_years(date(2021, 2, 28), date(2020, 2, 29)),
            1
        );
        assert_eq!(
            difference_in_years(date(2021, 2, 27), date(2020, 2, 29)),
            0
        );
        assert_eq!(
            difference_in_years(date(2024, 2, 29), date(2020, 2, 29)),
            4
        );
    }

    #[test]
    fn ignores_partial_years_and_keeps_signs() {
        assert_eq!(
            difference_in_years(date(2025, 1, 1), date(2024, 12, 31)),
            0
        );
        assert_eq!(
            difference_in_years(date(2020, 2, 29), date(2021, 2, 28)),
            -1
        );
    }

    #[test]
    fn extreme_dates_remain_safe() {
        let years = difference_in_years(NaiveDate::MAX, NaiveDate::MIN);
        assert!(years > 0);
        assert_eq!(difference_in_years(NaiveDate::MIN, NaiveDate::MAX), -years);
    }
}
