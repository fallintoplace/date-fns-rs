//! Ordinals of a date's weekday occurrence in its calendar year.

use chrono::{Datelike, NaiveDate};

/// The 1-based occurrence of this weekday from the start of the year.
///
/// For example, the second Monday in a year returns 2.
pub fn get_weekday_occurrence_in_year(date: NaiveDate) -> u32 {
    (date.ordinal() - 1) / 7 + 1
}

/// The negative weekday occurrence from the year end (-1 is the last).
pub fn get_weekday_occurrence_from_end_of_year(date: NaiveDate) -> i8 {
    let last = crate::end_of_year(date);
    -(((last.ordinal() - date.ordinal()) / 7 + 1) as i8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_forward_and_backward_in_leap_years() {
        assert_eq!(get_weekday_occurrence_in_year(date(2024, 1, 1)), 1);
        assert_eq!(get_weekday_occurrence_in_year(date(2024, 1, 8)), 2);
        assert_eq!(get_weekday_occurrence_in_year(date(2024, 12, 30)), 53);
        assert_eq!(get_weekday_occurrence_from_end_of_year(date(2024, 12, 30)), -1);
        assert_eq!(get_weekday_occurrence_from_end_of_year(date(2024, 12, 23)), -2);
        assert_eq!(get_weekday_occurrence_from_end_of_year(date(2023, 12, 31)), -1);
    }

    #[test]
    fn respects_the_calendar_year_and_supported_limits() {
        assert_eq!(get_weekday_occurrence_in_year(date(2025, 1, 1)), 1);
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!((1..=53).contains(&get_weekday_occurrence_in_year(edge)));
            assert!((-53..=-1).contains(&get_weekday_occurrence_from_end_of_year(edge)));
        }
    }
}
