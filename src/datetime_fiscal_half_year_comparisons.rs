//! Compare fiscal half-years using naive date-times.

use chrono::NaiveDateTime;

/// Check whether two naive date-times belong to the same fiscal half-year.
///
/// Clock time does not affect membership. `first_month` is 1 through 12;
/// invalid months return None.
pub fn is_same_fiscal_half_year_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
    first_month: u32,
) -> Option<bool> {
    crate::is_same_fiscal_half_year(left.date(), right.date(), first_month)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn timestamp(year: i32, month: u32, day: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, 0, 0)
            .unwrap()
    }

    #[test]
    fn compares_fiscal_periods_across_boundaries() {
        let first = timestamp(2023, 10, 1, 0);
        let last = timestamp(2024, 3, 31, 23);
        let following = timestamp(2024, 4, 1, 0);
        assert_eq!(
            is_same_fiscal_half_year_datetime(first, last, 4),
            Some(true)
        );
        assert_eq!(
            is_same_fiscal_half_year_datetime(last, following, 4),
            Some(false)
        );
        assert_eq!(
            is_same_fiscal_half_year_datetime(first, first, 4),
            Some(true)
        );
    }

    #[test]
    fn validates_months_and_extreme_timestamps() {
        let instant = timestamp(2023, 10, 1, 0);
        assert_eq!(is_same_fiscal_half_year_datetime(instant, instant, 0), None);
        assert_eq!(
            is_same_fiscal_half_year_datetime(instant, instant, 13),
            None
        );
        assert_eq!(
            is_same_fiscal_half_year_datetime(NaiveDateTime::MIN, NaiveDateTime::MIN, 1),
            Some(true)
        );
        assert_eq!(
            is_same_fiscal_half_year_datetime(NaiveDateTime::MAX, NaiveDateTime::MAX, 1),
            Some(true)
        );
    }
}
