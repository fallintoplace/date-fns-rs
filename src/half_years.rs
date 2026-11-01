//! Calendar half-year labels and boundaries.

use chrono::{Datelike, NaiveDate};

/// Return 1 for January through June, or 2 for July through December.
pub fn get_half_year(date: NaiveDate) -> u32 {
    (date.month() - 1) / 6 + 1
}

/// Return the first day of the half-year containing the date.
pub fn start_of_half_year(date: NaiveDate) -> NaiveDate {
    let month = if date.month() <= 6 { 1 } else { 7 };
    NaiveDate::from_ymd_opt(date.year(), month, 1)
        .expect("first day of a valid half-year must exist")
}

/// Return the last day of the half-year containing the date.
pub fn end_of_half_year(date: NaiveDate) -> NaiveDate {
    let (month, day) = if date.month() <= 6 { (6, 30) } else { (12, 31) };
    NaiveDate::from_ymd_opt(date.year(), month, day)
        .expect("last day of a valid half-year must exist")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn halves_have_inclusive_boundaries() {
        let first = date(2024, 6, 30);
        let second = date(2024, 7, 1);
        assert_eq!(get_half_year(first), 1);
        assert_eq!(get_half_year(second), 2);
        assert_eq!(start_of_half_year(first), date(2024, 1, 1));
        assert_eq!(end_of_half_year(first), first);
        assert_eq!(start_of_half_year(second), second);
        assert_eq!(end_of_half_year(second), date(2024, 12, 31));
    }

    #[test]
    fn handles_leap_years_and_supported_limits() {
        assert_eq!(end_of_half_year(date(2024, 2, 29)), date(2024, 6, 30));
        assert_eq!(start_of_half_year(NaiveDate::MIN), NaiveDate::MIN);
        assert_eq!(end_of_half_year(NaiveDate::MAX), NaiveDate::MAX);
    }
}
