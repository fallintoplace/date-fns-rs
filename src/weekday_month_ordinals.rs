//! Ordinals of a date's weekday occurrence within its month.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::end_of_month;

/// Return the 1-based occurrence of this weekday from the month start.
pub fn get_weekday_occurrence_in_month(date: NaiveDate) -> u32 {
    (date.day() - 1) / 7 + 1
}

/// Return the negative occurrence from the end (-1 for the last such weekday).
pub fn get_weekday_occurrence_from_end_of_month(date: NaiveDate) -> i8 {
    -(((end_of_month(date).day() - date.day()) / 7 + 1) as i8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn identifies_last_and_fifth_weekday_occurrences() {
        let fifth_friday = date(2024, 3, 29);
        assert_eq!(get_weekday_occurrence_in_month(fifth_friday), 5);
        assert_eq!(get_weekday_occurrence_from_end_of_month(fifth_friday), -1);
        assert_eq!(get_weekday_occurrence_in_month(date(2024, 2, 29)), 5);
        assert_eq!(
            get_weekday_occurrence_from_end_of_month(date(2024, 2, 1)),
            -5
        );
        assert_eq!(get_weekday_occurrence_in_month(date(2024, 3, 1)), 1);
    }

    #[test]
    fn supports_extreme_dates() {
        assert!((1..=5).contains(&get_weekday_occurrence_in_month(NaiveDate::MIN)));
        assert!((-5..=-1).contains(&get_weekday_occurrence_from_end_of_month(NaiveDate::MAX)));
    }
}
