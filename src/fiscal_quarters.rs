//! Fiscal quarter boundaries with a configurable starting month.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::end_of_month;

/// Return the 1-based fiscal quarter for a financial year starting in `first_month`.
///
/// `first_month` must be 1 through 12.
pub fn get_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<u32> {
    if !(1..=12).contains(&first_month) {
        return None;
    }
    let offset = (date.month() + 12 - first_month) % 12;
    Some(offset / 3 + 1)
}

/// Return the first calendar day of the date's fiscal quarter.
///
/// Returns None for an invalid `first_month` or an out-of-range boundary.
pub fn start_of_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<NaiveDate> {
    if !(1..=12).contains(&first_month) {
        return None;
    }

    let offset = (date.month() + 12 - first_month) % 12;
    let start_offset = offset / 3 * 3;
    let month = (first_month - 1 + start_offset) % 12 + 1;
    let year = if month > date.month() {
        date.year().checked_sub(1)?
    } else {
        date.year()
    };
    NaiveDate::from_ymd_opt(year, month, 1)
}

/// Return the last calendar day of the date's fiscal quarter.
///
/// Returns None for an invalid `first_month` or an out-of-range boundary.
pub fn end_of_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<NaiveDate> {
    let start = start_of_fiscal_quarter(date, first_month)?;
    let last_month = (start.month() + 1) % 12 + 1;
    let year = if last_month < start.month() {
        start.year().checked_add(1)?
    } else {
        start.year()
    };
    let last_month_start = NaiveDate::from_ymd_opt(year, last_month, 1)?;
    Some(end_of_month(last_month_start))
}

/// Count fiscal-quarter boundaries from right to left.
///
/// The fiscal year starts in `first_month`, which must be 1 through 12.
pub fn difference_in_calendar_fiscal_quarters(
    left: NaiveDate,
    right: NaiveDate,
    first_month: u32,
) -> Option<i64> {
    if !(1..=12).contains(&first_month) {
        return None;
    }

    let index = |date: NaiveDate| {
        (i64::from(date.year()) * 12 + i64::from(date.month()) - i64::from(first_month))
            .div_euclid(3)
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
    fn supports_fiscal_years_starting_in_february() {
        let january = date(2024, 1, 15);
        assert_eq!(get_fiscal_quarter(january, 2), Some(4));
        assert_eq!(start_of_fiscal_quarter(january, 2), Some(date(2023, 11, 1)));
        assert_eq!(end_of_fiscal_quarter(january, 2), Some(date(2024, 1, 31)));

        let february = date(2024, 2, 29);
        assert_eq!(get_fiscal_quarter(february, 2), Some(1));
        assert_eq!(start_of_fiscal_quarter(february, 2), Some(date(2024, 2, 1)));
        assert_eq!(end_of_fiscal_quarter(february, 2), Some(date(2024, 4, 30)));
    }

    #[test]
    fn supports_standard_and_april_fiscal_years() {
        let december = date(2024, 12, 31);
        assert_eq!(get_fiscal_quarter(december, 1), Some(4));
        assert_eq!(get_fiscal_quarter(december, 4), Some(3));
        assert_eq!(
            start_of_fiscal_quarter(december, 4),
            Some(date(2024, 10, 1))
        );
        assert_eq!(end_of_fiscal_quarter(december, 4), Some(date(2024, 12, 31)));
    }

    #[test]
    fn rejects_invalid_months_and_handles_limits() {
        let day = date(2024, 3, 15);
        assert_eq!(get_fiscal_quarter(day, 0), None);
        assert_eq!(start_of_fiscal_quarter(day, 13), None);
        assert_eq!(end_of_fiscal_quarter(day, u32::MAX), None);
        assert_eq!(
            start_of_fiscal_quarter(NaiveDate::MIN, 1),
            Some(NaiveDate::MIN)
        );
        assert_eq!(
            end_of_fiscal_quarter(NaiveDate::MAX, 1),
            Some(NaiveDate::MAX)
        );
    }
    #[test]
    fn respects_configured_fiscal_quarter_boundaries() {
        let a = date(2024, 4, 1);
        let b = date(2024, 3, 31);
        assert_eq!(difference_in_calendar_fiscal_quarters(a, b, 4), Some(1));
        assert_eq!(difference_in_calendar_fiscal_quarters(a, b, 2), Some(0));
        assert_eq!(
            difference_in_calendar_fiscal_quarters(date(2024, 2, 1), date(2024, 1, 31), 2),
            Some(1)
        );
    }

    #[test]
    fn validates_months_and_signed_order() {
        let a = date(2024, 6, 15);
        let b = date(2023, 12, 31);
        let count = difference_in_calendar_fiscal_quarters(a, b, 4).unwrap();
        assert_eq!(difference_in_calendar_fiscal_quarters(b, a, 4), Some(-count));
        assert_eq!(difference_in_calendar_fiscal_quarters(a, b, 0), None);
        assert_eq!(difference_in_calendar_fiscal_quarters(a, b, 13), None);
    }
}
