//! Strict next/previous weekday navigation without time-zone assumptions.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::arithmetic::{add_days, sub_days};

/// Find the next occurrence of a weekday strictly after date.
///
/// If date already falls on the requested weekday, this returns seven days
/// later. None means the result is outside Chrono's supported date range.
pub fn next_day(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    let current = date.weekday().num_days_from_monday();
    let wanted = weekday.num_days_from_monday();
    let offset = (wanted + 7 - current) % 7;
    add_days(date, i64::from(if offset == 0 { 7 } else { offset }))
}

/// Find the previous occurrence of a weekday strictly before date.
///
/// A match on date itself resolves to seven days earlier.
pub fn previous_day(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    let current = date.weekday().num_days_from_monday();
    let wanted = weekday.num_days_from_monday();
    let offset = (current + 7 - wanted) % 7;
    sub_days(date, i64::from(if offset == 0 { 7 } else { offset }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_weekdays_across_month_and_year_boundaries() {
        assert_eq!(next_day(date(2024, 2, 28), Weekday::Fri), Some(date(2024, 3, 1)));
        assert_eq!(previous_day(date(2024, 3, 1), Weekday::Wed), Some(date(2024, 2, 28)));
        assert_eq!(next_day(date(2023, 12, 31), Weekday::Mon), Some(date(2024, 1, 1)));
    }

    #[test]
    fn exact_matches_advance_or_retreat_a_whole_week() {
        let monday = date(2024, 1, 1);
        assert_eq!(next_day(monday, Weekday::Mon), Some(date(2024, 1, 8)));
        assert_eq!(previous_day(monday, Weekday::Mon), Some(date(2023, 12, 25)));
        assert_eq!(next_day(monday, Weekday::Sun), Some(date(2024, 1, 7)));
        assert_eq!(previous_day(monday, Weekday::Sun), Some(date(2023, 12, 31)));
    }

    #[test]
    fn checks_overflow_instead_of_wrapping() {
        assert_eq!(next_day(NaiveDate::MAX, NaiveDate::MAX.weekday()), None);
        assert_eq!(previous_day(NaiveDate::MIN, NaiveDate::MIN.weekday()), None);
    }
}
