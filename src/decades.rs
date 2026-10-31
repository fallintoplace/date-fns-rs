//! Gregorian decade boundaries, including years before the common era.

use chrono::{Datelike, NaiveDate};

/// Return January 1 of the first year in a ten-year decade.
///
/// Years 2020 through 2029 form one decade, while -10 through -1 form
/// another. None indicates an unrepresentable boundary.
pub fn start_of_decade(date: NaiveDate) -> Option<NaiveDate> {
    let first_year = date.year().div_euclid(10) * 10;
    NaiveDate::from_ymd_opt(first_year, 1, 1)
}

/// Return December 31 of the last year in the date's decade.
///
/// Returns None if the end of the decade exceeds Chrono's supported range.
pub fn end_of_decade(date: NaiveDate) -> Option<NaiveDate> {
    let first_year = date.year().div_euclid(10) * 10;
    NaiveDate::from_ymd_opt(first_year + 9, 12, 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_decade_boundaries_in_regular_years() {
        assert_eq!(start_of_decade(date(2024, 7, 15)), Some(date(2020, 1, 1)));
        assert_eq!(end_of_decade(date(2024, 7, 15)), Some(date(2029, 12, 31)));
        assert_eq!(start_of_decade(date(2000, 1, 1)), Some(date(2000, 1, 1)));
        assert_eq!(end_of_decade(date(1999, 12, 31)), Some(date(1999, 12, 31)));
    }

    #[test]
    fn uses_euclidean_decades_around_year_zero() {
        let bc_year = date(-1, 5, 1);
        assert_eq!(start_of_decade(bc_year), Some(date(-10, 1, 1)));
        assert_eq!(end_of_decade(bc_year), Some(date(-1, 12, 31)));
        assert_eq!(start_of_decade(date(0, 1, 1)), Some(date(0, 1, 1)));
        assert_eq!(end_of_decade(date(0, 1, 1)), Some(date(9, 12, 31)));
    }

    #[test]
    fn does_not_panic_at_supported_date_extremes() {
        let _ = start_of_decade(NaiveDate::MIN);
        let _ = end_of_decade(NaiveDate::MAX);
        assert!(start_of_decade(NaiveDate::MAX) <= Some(NaiveDate::MAX));
    }
}
