//! One-based Gregorian century boundaries with checked date construction.

use chrono::{Datelike, NaiveDate};

/// Get January 1 of the century containing a date.
///
/// Years 2001 through 2100 belong to the 21st century. Astronomical
/// year zero belongs to the century spanning years -99 through 0.
pub fn start_of_century(date: NaiveDate) -> Option<NaiveDate> {
    let first_year = (date.year() - 1).div_euclid(100) * 100 + 1;
    NaiveDate::from_ymd_opt(first_year, 1, 1)
}

/// Get December 31 of the final year of the date's century.
///
/// Returns None if the century's final date is out of Chrono's range.
pub fn end_of_century(date: NaiveDate) -> Option<NaiveDate> {
    let first_year = (date.year() - 1).div_euclid(100) * 100 + 1;
    NaiveDate::from_ymd_opt(first_year + 99, 12, 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn uses_one_based_centuries() {
        assert_eq!(start_of_century(date(2024, 5, 6)), Some(date(2001, 1, 1)));
        assert_eq!(end_of_century(date(2024, 5, 6)), Some(date(2100, 12, 31)));
        assert_eq!(start_of_century(date(2000, 1, 1)), Some(date(1901, 1, 1)));
        assert_eq!(end_of_century(date(2000, 1, 1)), Some(date(2000, 12, 31)));
        assert_eq!(start_of_century(date(1, 1, 1)), Some(date(1, 1, 1)));
    }

    #[test]
    fn handles_year_zero_and_bce_without_wrapping() {
        assert_eq!(start_of_century(date(0, 1, 1)), Some(date(-99, 1, 1)));
        assert_eq!(end_of_century(date(-1, 12, 31)), Some(date(0, 12, 31)));
        assert_eq!(start_of_century(date(-100, 1, 1)), Some(date(-199, 1, 1)));
        assert_eq!(end_of_century(date(-100, 1, 1)), Some(date(-100, 12, 31)));
        let _ = start_of_century(NaiveDate::MIN);
        let _ = end_of_century(NaiveDate::MAX);
    }
}
