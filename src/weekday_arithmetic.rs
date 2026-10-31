//! Checked navigation to a specific weekday.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::arithmetic::{add_days, sub_days};

/// Return the next occurrence of `weekday` strictly after `date`.
///
/// Selecting the current weekday advances seven days. Returns None on overflow.
pub fn next_weekday(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    let current = i64::from(date.weekday().num_days_from_monday());
    let target = i64::from(weekday.num_days_from_monday());
    let offset = (target - current).rem_euclid(7);
    add_days(date, if offset == 0 { 7 } else { offset })
}

/// Return the previous occurrence of `weekday` strictly before `date`.
///
/// Selecting the current weekday moves back seven days. Returns None on overflow.
pub fn previous_weekday(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    let current = i64::from(date.weekday().num_days_from_monday());
    let target = i64::from(weekday.num_days_from_monday());
    let offset = (current - target).rem_euclid(7);
    sub_days(date, if offset == 0 { 7 } else { offset })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn navigates_across_week_and_year_boundaries() {
        let monday = date(2024, 1, 1);
        assert_eq!(next_weekday(monday, Weekday::Fri), Some(date(2024, 1, 5)));
        assert_eq!(previous_weekday(monday, Weekday::Fri), Some(date(2023, 12, 29)));
        assert_eq!(next_weekday(monday, Weekday::Mon), Some(date(2024, 1, 8)));
        assert_eq!(previous_weekday(monday, Weekday::Mon), Some(date(2023, 12, 25)));
    }

    #[test]
    fn returns_none_near_supported_date_limits() {
        assert_eq!(next_weekday(NaiveDate::MAX, NaiveDate::MAX.weekday()), None);
        assert_eq!(
            previous_weekday(NaiveDate::MIN, NaiveDate::MIN.weekday()),
            None
        );
    }
}
