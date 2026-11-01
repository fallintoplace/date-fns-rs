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

/// Find the closest occurrence of a weekday, including the input date.
///
/// No tie is possible for a seven-day cycle.
pub fn nearest_weekday(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    let current = date.weekday().num_days_from_monday();
    let target = weekday.num_days_from_monday();
    let forward = (target + 7 - current) % 7;
    if forward <= 3 {
        add_days(date, i64::from(forward))
    } else {
        sub_days(date, i64::from(7 - forward))
    }
}

/// Find the next occurrence of a weekday, including the starting date.
///
/// Returns None only if moving forward would exceed the supported date range.
pub fn next_or_same_weekday(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    if date.weekday() == weekday {
        Some(date)
    } else {
        next_weekday(date, weekday)
    }
}

/// Find the previous occurrence of a weekday, including the starting date.
///
/// Returns None only if moving backward would exceed the supported date range.
pub fn previous_or_same_weekday(date: NaiveDate, weekday: Weekday) -> Option<NaiveDate> {
    if date.weekday() == weekday {
        Some(date)
    } else {
        previous_weekday(date, weekday)
    }
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
        assert_eq!(
            previous_weekday(monday, Weekday::Fri),
            Some(date(2023, 12, 29))
        );
        assert_eq!(next_weekday(monday, Weekday::Mon), Some(date(2024, 1, 8)));
        assert_eq!(
            previous_weekday(monday, Weekday::Mon),
            Some(date(2023, 12, 25))
        );
    }

    #[test]
    fn returns_none_near_supported_date_limits() {
        assert_eq!(next_weekday(NaiveDate::MAX, NaiveDate::MAX.weekday()), None);
        assert_eq!(
            previous_weekday(NaiveDate::MIN, NaiveDate::MIN.weekday()),
            None
        );
    }
    #[test]
    fn chooses_nearest_weekday_in_either_direction() {
        let monday = date(2024, 1, 1);
        assert_eq!(nearest_weekday(monday, Weekday::Mon), Some(monday));
        assert_eq!(
            nearest_weekday(date(2024, 1, 4), Weekday::Mon),
            Some(monday)
        );
        assert_eq!(
            nearest_weekday(date(2024, 1, 5), Weekday::Mon),
            Some(date(2024, 1, 8))
        );
        assert_eq!(
            nearest_weekday(date(2024, 1, 7), Weekday::Mon),
            Some(date(2024, 1, 8))
        );
    }

    #[test]
    fn identity_at_supported_date_limits() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert_eq!(nearest_weekday(edge, edge.weekday()), Some(edge));
        }
    }
    #[test]
    fn inclusive_weekday_navigation_keeps_matching_dates() {
        let monday = date(2024, 1, 1);
        assert_eq!(next_or_same_weekday(monday, Weekday::Mon), Some(monday));
        assert_eq!(previous_or_same_weekday(monday, Weekday::Mon), Some(monday));
        assert_eq!(
            next_or_same_weekday(monday, Weekday::Fri),
            Some(date(2024, 1, 5))
        );
        assert_eq!(
            previous_or_same_weekday(monday, Weekday::Fri),
            Some(date(2023, 12, 29))
        );
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert_eq!(next_or_same_weekday(edge, edge.weekday()), Some(edge));
            assert_eq!(previous_or_same_weekday(edge, edge.weekday()), Some(edge));
        }
    }

}
