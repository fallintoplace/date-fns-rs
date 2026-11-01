//! Day ordinals and lengths for January-June and July-December.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::is_leap_year;

/// Return the one-based day ordinal within the current calendar half-year.
pub fn get_day_of_half_year(date: NaiveDate) -> u32 {
    let preceding_days = if date.month() <= 6 {
        0
    } else if is_leap_year(date.year()) {
        182
    } else {
        181
    };
    date.ordinal() - preceding_days
}

/// Return 181 or 182 days for the first half, or 184 for the second.
pub fn get_days_in_half_year(date: NaiveDate) -> u32 {
    if date.month() > 6 {
        184
    } else if is_leap_year(date.year()) {
        182
    } else {
        181
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_leap_and_common_years() {
        assert_eq!(get_days_in_half_year(date(2024, 1, 1)), 182);
        assert_eq!(get_days_in_half_year(date(2023, 1, 1)), 181);
        assert_eq!(get_days_in_half_year(date(2024, 12, 1)), 184);
        assert_eq!(get_day_of_half_year(date(2024, 2, 29)), 60);
        assert_eq!(get_day_of_half_year(date(2024, 6, 30)), 182);
        assert_eq!(get_day_of_half_year(date(2023, 6, 30)), 181);
        assert_eq!(get_day_of_half_year(date(2024, 7, 1)), 1);
        assert_eq!(get_day_of_half_year(date(2024, 12, 31)), 184);
    }

    #[test]
    fn ordinals_stay_in_range_at_chrono_limits() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!((1..=get_days_in_half_year(edge)).contains(&get_day_of_half_year(edge)));
        }
    }
}
