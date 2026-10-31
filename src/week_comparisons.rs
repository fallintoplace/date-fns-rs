//! Week comparisons with configurable start days.

use chrono::{Datelike, NaiveDate, Weekday};

fn week_start_day_number(date: NaiveDate, week_starts_on: Weekday) -> i64 {
    let day = i64::from(date.num_days_from_ce());
    let weekday = i64::from(date.weekday().num_days_from_monday());
    let first = i64::from(week_starts_on.num_days_from_monday());
    day - (weekday - first).rem_euclid(7)
}

/// Check whether two dates belong to the same calendar week.
///
/// The start day is explicit. Unlike calculating week-boundary dates,
/// comparison remains valid even at the supported calendar extremes.
pub fn is_same_week(left: NaiveDate, right: NaiveDate, week_starts_on: Weekday) -> bool {
    week_start_day_number(left, week_starts_on)
        == week_start_day_number(right, week_starts_on)
}

/// Check whether two dates belong to the same Monday-first ISO week.
pub fn is_same_iso_week(left: NaiveDate, right: NaiveDate) -> bool {
    is_same_week(left, right, Weekday::Mon)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn respects_sunday_and_monday_week_starts() {
        let sunday = date(2023, 12, 31);
        let monday = date(2024, 1, 1);
        assert!(!is_same_week(sunday, monday, Weekday::Mon));
        assert!(is_same_week(sunday, monday, Weekday::Sun));
        assert!(!is_same_iso_week(sunday, monday));
    }

    #[test]
    fn includes_iso_weeks_that_cross_year_boundaries() {
        assert!(is_same_iso_week(date(2019, 12, 30), date(2020, 1, 5)));
        assert!(!is_same_iso_week(date(2020, 1, 5), date(2020, 1, 6)));
        assert!(is_same_week(
            date(2024, 2, 29),
            date(2024, 2, 29),
            Weekday::Thu
        ));
    }

    #[test]
    fn handles_date_limits_without_boundary_overflow() {
        assert!(is_same_iso_week(NaiveDate::MIN, NaiveDate::MIN));
        assert!(is_same_iso_week(NaiveDate::MAX, NaiveDate::MAX));
        assert!(!is_same_week(
            NaiveDate::MIN,
            NaiveDate::MAX,
            Weekday::Mon
        ));
    }
}
