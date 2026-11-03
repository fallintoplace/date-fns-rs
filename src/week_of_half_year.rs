//! Week numbers within January-June or July-December.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return the one-based week number within the date's calendar half-year.
///
/// Partial first weeks count as week one. An explicit week start supports
/// both Monday-first and Sunday-first calendars.
pub fn get_week_of_half_year(date: NaiveDate, week_starts_on: Weekday) -> u32 {
    let start = crate::start_of_half_year(date);
    let elapsed = date.signed_duration_since(start).num_days();
    let offset = i64::from(
        (start.weekday().num_days_from_monday() + 7 - week_starts_on.num_days_from_monday()) % 7,
    );
    u32::try_from((elapsed + offset) / 7 + 1)
        .expect("a half-year cannot contain more than 27 partial weeks")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_partial_weeks_from_each_half_year_start() {
        assert_eq!(get_week_of_half_year(day(2024, 1, 1), Weekday::Mon), 1);
        assert_eq!(get_week_of_half_year(day(2024, 1, 7), Weekday::Mon), 1);
        assert_eq!(get_week_of_half_year(day(2024, 1, 8), Weekday::Mon), 2);
        assert_eq!(get_week_of_half_year(day(2024, 6, 30), Weekday::Mon), 26);
        assert_eq!(get_week_of_half_year(day(2024, 6, 30), Weekday::Sun), 27);
        assert_eq!(get_week_of_half_year(day(2024, 7, 1), Weekday::Mon), 1);
        assert_eq!(get_week_of_half_year(day(2024, 7, 8), Weekday::Mon), 2);
    }

    #[test]
    fn handles_both_supported_date_limits() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!((1..=27).contains(&get_week_of_half_year(edge, Weekday::Sun)));
        }
    }
}
