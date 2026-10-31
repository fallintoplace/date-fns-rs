//! One-based week numbers within a calendar month.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return the 1-based week number in the month for a chosen week start.
///
/// A partial week at the beginning of the month counts as week 1.
/// Results range from 1 to 6. No neighboring calendar date is needed,
/// so this also works at the minimum and maximum supported dates.
pub fn get_week_of_month(date: NaiveDate, week_starts_on: Weekday) -> u32 {
    let first = date
        .with_day(1)
        .expect("first day of a valid month must exist");
    let first_weekday = first.weekday().num_days_from_monday();
    let week_start = week_starts_on.num_days_from_monday();
    let offset = (first_weekday + 7 - week_start) % 7;
    (date.day() - 1 + offset) / 7 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn respects_monday_and_sunday_first_calendars() {
        assert_eq!(get_week_of_month(date(2024, 3, 1), Weekday::Mon), 1);
        assert_eq!(get_week_of_month(date(2024, 3, 3), Weekday::Mon), 1);
        assert_eq!(get_week_of_month(date(2024, 3, 4), Weekday::Mon), 2);
        assert_eq!(get_week_of_month(date(2024, 3, 3), Weekday::Sun), 2);
        assert_eq!(get_week_of_month(date(2024, 3, 31), Weekday::Sun), 6);
    }

    #[test]
    fn works_at_calendar_boundaries() {
        assert!((1..=6).contains(&get_week_of_month(NaiveDate::MIN, Weekday::Mon)));
        assert!((1..=6).contains(&get_week_of_month(NaiveDate::MAX, Weekday::Sun)));
        assert_eq!(get_week_of_month(date(2024, 1, 1), Weekday::Mon), 1);
        assert_eq!(get_week_of_month(date(2024, 2, 29), Weekday::Mon), 5);
    }
}
