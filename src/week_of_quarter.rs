//! One-based week numbers within calendar quarters.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::quarters::start_of_quarter;

/// Return the 1-based week of the quarter for a chosen first weekday.
///
/// Partial weeks at the beginning of a quarter count as week one. The number
/// is independent of whether a week overlaps a neighboring quarter.
pub fn get_week_of_quarter(date: NaiveDate, week_starts_on: Weekday) -> u32 {
    let first = start_of_quarter(date);
    let offset = (first.weekday().num_days_from_monday() + 7
        - week_starts_on.num_days_from_monday())
        % 7;
    (date.ordinal() - first.ordinal() + offset) / 7 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn respects_configurable_partial_weeks() {
        assert_eq!(get_week_of_quarter(date(2024, 1, 1), Weekday::Mon), 1);
        assert_eq!(get_week_of_quarter(date(2024, 1, 7), Weekday::Mon), 1);
        assert_eq!(get_week_of_quarter(date(2024, 1, 8), Weekday::Mon), 2);
        assert_eq!(get_week_of_quarter(date(2024, 1, 7), Weekday::Sun), 2);
        assert_eq!(get_week_of_quarter(date(2024, 3, 31), Weekday::Mon), 13);
        assert_eq!(get_week_of_quarter(date(2024, 4, 1), Weekday::Mon), 1);
    }

    #[test]
    fn works_near_chrono_range_limits() {
        assert!((1..=14).contains(&get_week_of_quarter(NaiveDate::MIN, Weekday::Sun)));
        assert!((1..=14).contains(&get_week_of_quarter(NaiveDate::MAX, Weekday::Mon)));
    }
}
