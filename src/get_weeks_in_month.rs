//! Calendar weeks covered by a month, including partial boundary weeks.
use chrono::{NaiveDate, Weekday};

/// Count calendar weeks touched by the month containing `date`.
///
/// Week boundaries use `week_starts_on`, and partial weeks count as full rows.
/// Works even when adjacent padding dates exceed Chrono's range.
pub fn get_weeks_in_month(date: NaiveDate, week_starts_on: Weekday) -> u32 {
    crate::get_week_of_month(crate::end_of_month(date), week_starts_on)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn counts_partial_weeks_from_date_fns_examples() {
        assert_eq!(get_weeks_in_month(day(2015, 2, 8), Weekday::Sun), 4);
        assert_eq!(get_weeks_in_month(day(2017, 7, 5), Weekday::Mon), 6);
        assert_eq!(get_weeks_in_month(day(2024, 2, 29), Weekday::Mon), 5);
        assert_eq!(get_weeks_in_month(day(2024, 3, 15), Weekday::Sun), 6);
    }

    #[test]
    fn handles_calendar_extremes_without_padded_boundaries() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            for week_start in [Weekday::Mon, Weekday::Sun] {
                assert!((4..=6).contains(&get_weeks_in_month(date, week_start)));
            }
        }
    }
}
