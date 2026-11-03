//! Constant-time counts of calendar periods touched by inclusive date intervals.

use chrono::NaiveDate;

/// Count fiscal years overlapping an inclusive date interval.
///
/// Both endpoints count. Reversed intervals return None.
/// Fiscal years start in `first_month`, which must be 1 through 12.
pub fn count_fiscal_years_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    first_month: u32,
) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_fiscal_years(end, start, first_month)? + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_periods_across_boundaries() {
        assert_eq!(
            count_fiscal_years_in_interval(day(2024, 3, 31), day(2024, 4, 1), 4),
            Some(2)
        );
        assert_eq!(
            count_fiscal_years_in_interval(day(2024, 12, 31), day(2025, 1, 1), 4),
            Some(1)
        );
        assert_eq!(
            count_fiscal_years_in_interval(day(2023, 4, 1), day(2025, 3, 31), 4),
            Some(2)
        );
    }

    #[test]
    fn validates_ranges_and_date_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(count_fiscal_years_in_interval(last, first, 4), None);
        assert_eq!(count_fiscal_years_in_interval(first, last, 0), None);
        assert_eq!(count_fiscal_years_in_interval(first, last, 13), None);
        let count = count_fiscal_years_in_interval(NaiveDate::MIN, NaiveDate::MAX, 4).unwrap();
        assert!(count > 1);
        assert_eq!(
            count_fiscal_years_in_interval(NaiveDate::MAX, NaiveDate::MAX, 4),
            Some(1)
        );
    }
}
