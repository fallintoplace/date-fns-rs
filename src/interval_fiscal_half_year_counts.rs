//! Count calendar periods touched by inclusive date intervals.

use chrono::NaiveDate;

/// Count distinct fiscal half-years touched by an inclusive interval.
///
/// Includes both endpoints and rejects reversed intervals.
/// Fiscal years begin in month 1 through 12.
pub fn count_fiscal_half_years_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    first_month: u32,
) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_fiscal_half_years(end, start, first_month)? + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_boundaries_using_the_configured_calendar() {
        assert_eq!(
            count_fiscal_half_years_in_interval(day(2024, 3, 31), day(2024, 4, 1), 4),
            Some(2)
        );
        assert_eq!(
            count_fiscal_half_years_in_interval(day(2024, 9, 30), day(2024, 10, 1), 4),
            Some(2)
        );
        assert_eq!(
            count_fiscal_half_years_in_interval(day(2024, 10, 1), day(2025, 3, 31), 4),
            Some(1)
        );
    }

    #[test]
    fn validates_rules_and_supported_date_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(count_fiscal_half_years_in_interval(last, first, 4), None);
        assert_eq!(count_fiscal_half_years_in_interval(first, last, 0), None);
        assert_eq!(count_fiscal_half_years_in_interval(first, last, 13), None);
        let count = count_fiscal_half_years_in_interval(NaiveDate::MIN, NaiveDate::MAX, 4).unwrap();
        assert!(count > 1);
        assert_eq!(
            count_fiscal_half_years_in_interval(NaiveDate::MAX, NaiveDate::MAX, 4),
            Some(1)
        );
    }
}
