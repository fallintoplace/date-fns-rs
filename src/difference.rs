//! Signed differences in calendar units.

use chrono::{Datelike, NaiveDate};

/// Count calendar-day boundaries from `right` to `left`.
///
/// A date one day after another has difference 1, regardless of leap years.
pub fn difference_in_calendar_days(left: NaiveDate, right: NaiveDate) -> i64 {
    i64::from(left.num_days_from_ce()) - i64::from(right.num_days_from_ce())
}

/// Count calendar-month boundaries from `right` to `left`.
///
/// Days of the month are ignored: February 1 minus January 31 is 1.
pub fn difference_in_calendar_months(left: NaiveDate, right: NaiveDate) -> i64 {
    (i64::from(left.year()) - i64::from(right.year())) * 12 + i64::from(left.month())
        - i64::from(right.month())
}

/// Count calendar-year boundaries from `right` to `left`.
///
/// Months and days are ignored: January 1 minus December 31 is 1.
pub fn difference_in_calendar_years(left: NaiveDate, right: NaiveDate) -> i64 {
    i64::from(left.year()) - i64::from(right.year())
}

/// Count calendar-decade boundaries from right to left.
///
/// Years -10 through -1 form one decade, followed by years 0 through 9.
pub fn difference_in_calendar_decades(left: NaiveDate, right: NaiveDate) -> i64 {
    i64::from(left.year().div_euclid(10)) - i64::from(right.year().div_euclid(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn counts_calendar_days_through_leap_day() {
        let leap_days = difference_in_calendar_days(date(2024, 3, 1), date(2024, 2, 28));
        assert_eq!(leap_days, 2);
        let year_boundary = difference_in_calendar_days(date(2024, 1, 1), date(2023, 12, 31));
        assert_eq!(year_boundary, 1);
        let same_day = difference_in_calendar_days(date(2024, 1, 1), date(2024, 1, 1));
        assert_eq!(same_day, 0);
    }

    #[test]
    fn counts_month_and_year_boundaries() {
        let month_boundary = difference_in_calendar_months(date(2024, 2, 1), date(2024, 1, 31));
        assert_eq!(month_boundary, 1);
        let year_end_month = difference_in_calendar_months(date(2024, 1, 1), date(2023, 12, 31));
        assert_eq!(year_end_month, 1);
        let next_year = difference_in_calendar_years(date(2024, 1, 1), date(2023, 12, 31));
        assert_eq!(next_year, 1);
        let previous_year = difference_in_calendar_years(date(2023, 12, 31), date(2024, 1, 1));
        assert_eq!(previous_year, -1);
    }

    #[test]
    fn differences_are_antisymmetric_at_date_range_edges() {
        let days = difference_in_calendar_days(NaiveDate::MAX, NaiveDate::MIN);
        assert!(days > 0);
        let reversed_days = difference_in_calendar_days(NaiveDate::MIN, NaiveDate::MAX);
        assert_eq!(reversed_days, -days);
        let months = difference_in_calendar_months(NaiveDate::MAX, NaiveDate::MIN);
        let reversed_months = difference_in_calendar_months(NaiveDate::MIN, NaiveDate::MAX);
        assert_eq!(reversed_months, -months);
    }
    #[test]
    fn counts_decade_boundaries_not_elapsed_years() {
        assert_eq!(
            difference_in_calendar_decades(date(2020, 1, 1), date(2019, 12, 31)),
            1
        );
        assert_eq!(
            difference_in_calendar_decades(date(2019, 12, 31), date(2010, 1, 1)),
            0
        );
        assert_eq!(
            difference_in_calendar_decades(date(0, 1, 1), date(-1, 12, 31)),
            1
        );
    }

    #[test]
    fn supports_reversed_and_extreme_dates() {
        let count = difference_in_calendar_decades(NaiveDate::MAX, NaiveDate::MIN);
        assert!(count > 0);
        assert_eq!(
            difference_in_calendar_decades(NaiveDate::MIN, NaiveDate::MAX),
            -count
        );
    }
}
