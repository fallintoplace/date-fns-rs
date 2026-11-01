//! Fiscal half-year labels and boundaries with a chosen year start.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::end_of_month;

/// Return the 1-based fiscal half-year for a fiscal year starting in first_month.
pub fn get_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    if !(1..=12).contains(&first_month) {
        return None;
    }
    Some(((date.month() + 12 - first_month) % 12) / 6 + 1)
}

/// Return the first date of the fiscal half containing date.
///
/// Returns None for invalid months or out-of-range fiscal boundaries.
pub fn start_of_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<NaiveDate> {
    if !(1..=12).contains(&first_month) {
        return None;
    }
    let offset = (date.month() + 12 - first_month) % 12;
    let first = (first_month - 1 + (offset / 6) * 6) % 12 + 1;
    let year = if first > date.month() {
        date.year().checked_sub(1)?
    } else {
        date.year()
    };
    NaiveDate::from_ymd_opt(year, first, 1)
}

/// Return the inclusive last day of the fiscal half containing date.
pub fn end_of_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<NaiveDate> {
    let start = start_of_fiscal_half_year(date, first_month)?;
    let last_month = (start.month() + 4) % 12 + 1;
    let year = if last_month < start.month() {
        start.year().checked_add(1)?
    } else {
        start.year()
    };
    Some(end_of_month(NaiveDate::from_ymd_opt(year, last_month, 1)?))
}

/// Count fiscal half-year boundaries from right to left.
///
/// The fiscal year starts in first_month, which must be between 1 and 12.
/// Uses integer month indices, including across negative calendar years.
pub fn difference_in_calendar_fiscal_half_years(
    left: NaiveDate,
    right: NaiveDate,
    first_month: u32,
) -> Option<i64> {
    if !(1..=12).contains(&first_month) {
        return None;
    }
    let index = |date: NaiveDate| {
        (i64::from(date.year()) * 12 + i64::from(date.month()) - i64::from(first_month))
            .div_euclid(6)
    };
    Some(index(left) - index(right))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn supports_fiscal_halves_crossing_new_year() {
        let january = date(2024, 1, 15);
        assert_eq!(get_fiscal_half_year(january, 4), Some(2));
        assert_eq!(
            start_of_fiscal_half_year(january, 4),
            Some(date(2023, 10, 1))
        );
        assert_eq!(end_of_fiscal_half_year(january, 4), Some(date(2024, 3, 31)));
        assert_eq!(get_fiscal_half_year(date(2024, 4, 1), 4), Some(1));
        assert_eq!(
            end_of_fiscal_half_year(date(2024, 4, 1), 4),
            Some(date(2024, 9, 30))
        );
    }

    #[test]
    fn covers_leap_days_and_validates_start_month() {
        assert_eq!(
            end_of_fiscal_half_year(date(2024, 2, 29), 9),
            Some(date(2024, 2, 29))
        );
        assert_eq!(get_fiscal_half_year(date(2024, 1, 1), 0), None);
        assert_eq!(start_of_fiscal_half_year(date(2024, 1, 1), 13), None);
        assert_eq!(end_of_fiscal_half_year(date(2024, 1, 1), 0), None);
        assert_eq!(
            start_of_fiscal_half_year(NaiveDate::MIN, 1),
            Some(NaiveDate::MIN)
        );
    }
    #[test]
    fn fiscal_half_year_differences_follow_april_and_october() {
        let march = date(2024, 3, 31);
        let april = date(2024, 4, 1);
        let september = date(2024, 9, 30);
        let october = date(2024, 10, 1);
        assert_eq!(
            difference_in_calendar_fiscal_half_years(april, march, 4),
            Some(1)
        );
        assert_eq!(
            difference_in_calendar_fiscal_half_years(october, september, 4),
            Some(1)
        );
        assert_eq!(
            difference_in_calendar_fiscal_half_years(date(2025, 3, 31), october, 4),
            Some(0)
        );
        assert_eq!(
            difference_in_calendar_fiscal_half_years(march, october, 4),
            Some(-2)
        );
    }

    #[test]
    fn fiscal_half_year_differences_validate_start_and_limits() {
        let date = date(2024, 1, 1);
        assert_eq!(
            difference_in_calendar_fiscal_half_years(date, date, 0),
            None
        );
        assert_eq!(
            difference_in_calendar_fiscal_half_years(date, date, 13),
            None
        );
        let wide =
            difference_in_calendar_fiscal_half_years(NaiveDate::MAX, NaiveDate::MIN, 7).unwrap();
        assert_eq!(
            difference_in_calendar_fiscal_half_years(NaiveDate::MIN, NaiveDate::MAX, 7),
            Some(-wide)
        );
    }
}
