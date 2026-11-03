//! Boundary predicates for Gregorian decades and one-based centuries.

use chrono::NaiveDate;

/// Return whether date opens its Gregorian decade.
///
/// Decades are aligned to years divisible by ten, including year zero.
/// Returns None if the decade start cannot be represented.
pub fn is_first_day_of_decade(date: NaiveDate) -> Option<bool> {
    Some(date == crate::start_of_decade(date)?)
}

/// Return whether date ends its Gregorian decade.
pub fn is_last_day_of_decade(date: NaiveDate) -> Option<bool> {
    Some(date == crate::end_of_decade(date)?)
}

/// Return whether date opens its one-based Gregorian century.
///
/// The 21st century begins on January 1, 2001.
pub fn is_first_day_of_century(date: NaiveDate) -> Option<bool> {
    Some(date == crate::start_of_century(date)?)
}

/// Return whether date ends its one-based Gregorian century.
pub fn is_last_day_of_century(date: NaiveDate) -> Option<bool> {
    Some(date == crate::end_of_century(date)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn recognizes_decade_boundaries_across_year_zero() {
        assert_eq!(is_first_day_of_decade(day(2020, 1, 1)), Some(true));
        assert_eq!(is_last_day_of_decade(day(2029, 12, 31)), Some(true));
        assert_eq!(is_first_day_of_decade(day(2021, 1, 1)), Some(false));
        assert_eq!(is_last_day_of_decade(day(2028, 12, 31)), Some(false));
        assert_eq!(is_first_day_of_decade(day(0, 1, 1)), Some(true));
        assert_eq!(is_last_day_of_decade(day(-1, 12, 31)), Some(true));
    }

    #[test]
    fn recognizes_one_based_centuries_across_year_zero() {
        assert_eq!(is_first_day_of_century(day(2001, 1, 1)), Some(true));
        assert_eq!(is_last_day_of_century(day(2100, 12, 31)), Some(true));
        assert_eq!(is_last_day_of_century(day(2000, 12, 31)), Some(true));
        assert_eq!(is_first_day_of_century(day(2000, 1, 1)), Some(false));
        assert_eq!(is_first_day_of_century(day(-99, 1, 1)), Some(true));
        assert_eq!(is_last_day_of_century(day(0, 12, 31)), Some(true));
    }

    #[test]
    fn checks_chrono_extremes_without_panicking() {
        let _ = is_first_day_of_decade(NaiveDate::MIN);
        let _ = is_last_day_of_decade(NaiveDate::MAX);
        let _ = is_first_day_of_century(NaiveDate::MIN);
        let _ = is_last_day_of_century(NaiveDate::MAX);
    }
}
