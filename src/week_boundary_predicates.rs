//! Week boundary predicates for caller-chosen calendar conventions.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return true when date is the first day of a week with the chosen start.
pub fn is_first_day_of_week(date: NaiveDate, week_starts_on: Weekday) -> bool {
    date.weekday() == week_starts_on
}

/// Return true when date is the last day of a week with the chosen start.
pub fn is_last_day_of_week(date: NaiveDate, week_starts_on: Weekday) -> bool {
    date.weekday() == week_starts_on.pred()
}

/// Check whether date is Monday, the first day of an ISO week.
pub fn is_first_day_of_iso_week(date: NaiveDate) -> bool {
    is_first_day_of_week(date, Weekday::Mon)
}

/// Check whether date is Sunday, the last day of an ISO week.
pub fn is_last_day_of_iso_week(date: NaiveDate) -> bool {
    is_last_day_of_week(date, Weekday::Mon)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn respects_iso_and_sunday_first_calendars() {
        let sunday = date(2023, 12, 31);
        let monday = date(2024, 1, 1);
        assert!(is_last_day_of_iso_week(sunday));
        assert!(is_first_day_of_iso_week(monday));
        assert!(!is_first_day_of_iso_week(sunday));
        assert!(!is_last_day_of_iso_week(monday));
        assert!(is_first_day_of_week(sunday, Weekday::Sun));
        assert!(is_last_day_of_week(monday, Weekday::Tue));
        assert!(!is_last_day_of_week(monday, Weekday::Mon));
    }

    #[test]
    fn checks_limits_without_needing_out_of_range_week_boundaries() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!(is_first_day_of_week(date, date.weekday()));
            assert!(is_last_day_of_week(date, date.weekday().succ()));
        }
    }
}
