//! Calendar century comparisons in the proleptic Gregorian calendar.

use chrono::{Datelike, NaiveDate};

fn century_index(date: NaiveDate) -> i32 {
    (date.year() - 1).div_euclid(100)
}

/// Count one-based Gregorian century boundaries from right to left.
///
/// The 21st century spans years 2001-2100; years -99 through 0
/// form the century immediately before year 1.
pub fn difference_in_calendar_centuries(left: NaiveDate, right: NaiveDate) -> i32 {
    century_index(left) - century_index(right)
}

/// Check whether two dates belong to the same one-based century.
pub fn is_same_century(left: NaiveDate, right: NaiveDate) -> bool {
    century_index(left) == century_index(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap()
    }

    #[test]
    fn counts_2001_and_2101_as_new_centuries() {
        assert!(is_same_century(date(2001), date(2100)));
        assert!(!is_same_century(date(2000), date(2001)));
        assert_eq!(difference_in_calendar_centuries(date(2001), date(2000)), 1);
        assert_eq!(difference_in_calendar_centuries(date(2101), date(2001)), 1);
        assert_eq!(difference_in_calendar_centuries(date(2000), date(2101)), -2);
    }

    #[test]
    fn uses_euclidean_years_at_the_era_boundary() {
        assert!(is_same_century(date(-99), date(0)));
        assert!(!is_same_century(date(-100), date(-99)));
        assert_eq!(difference_in_calendar_centuries(date(1), date(0)), 1);
        assert_eq!(difference_in_calendar_centuries(date(0), date(-100)), 1);
    }

    #[test]
    fn handles_chrono_date_limits_without_constructing_boundaries() {
        let forward = difference_in_calendar_centuries(NaiveDate::MAX, NaiveDate::MIN);
        assert!(forward > 0);
        assert_eq!(
            difference_in_calendar_centuries(NaiveDate::MIN, NaiveDate::MAX),
            -forward
        );
    }
}
