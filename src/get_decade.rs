//! Gregorian decades aligned to years divisible by ten.
use chrono::{Datelike, NaiveDate};

/// Return the first year of the decade containing `date`.
///
/// For example, 1942 belongs to the 1940s; year -1 belongs to -10 through -1.
pub fn get_decade(date: NaiveDate) -> i32 {
    date.year().div_euclid(10) * 10
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 6, 15).unwrap()
    }

    #[test]
    fn returns_matching_gregorian_decade_labels() {
        assert_eq!(get_decade(day(1942)), 1940);
        assert_eq!(get_decade(day(2029)), 2020);
        assert_eq!(get_decade(day(2000)), 2000);
        assert_eq!(get_decade(day(0)), 0);
        assert_eq!(get_decade(day(-1)), -10);
        assert_eq!(get_decade(day(-10)), -10);
    }

    #[test]
    fn remains_consistent_with_start_of_decade_at_date_limits() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            if let Some(start) = crate::start_of_decade(date) {
                assert_eq!(get_decade(date), start.year());
            }
        }
    }
}
