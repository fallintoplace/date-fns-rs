//! Calendar-week boundary differences for an explicit week start.

use chrono::{Datelike, NaiveDate, Weekday};

fn week_index(date: NaiveDate, week_starts_on: Weekday) -> i64 {
    let day = i64::from(date.num_days_from_ce());
    let weekday = i64::from(date.weekday().num_days_from_monday());
    let first = i64::from(week_starts_on.num_days_from_monday());
    let offset = (weekday + 7 - first) % 7;
    (day - offset).div_euclid(7)
}

/// Count week boundaries from `right` to `left` using `week_starts_on`.
///
/// Partial weeks count as calendar boundaries, not as complete seven-day periods.
/// This also works when a week boundary falls outside Chrono's date range.
pub fn difference_in_calendar_weeks(
    left: NaiveDate,
    right: NaiveDate,
    week_starts_on: Weekday,
) -> i64 {
    week_index(left, week_starts_on) - week_index(right, week_starts_on)
}

/// Count ISO week-numbering year boundaries from right to left.
///
/// The ISO year may change before January 1 or after December 31.
pub fn difference_in_calendar_iso_week_years(left: NaiveDate, right: NaiveDate) -> i64 {
    i64::from(left.iso_week().year()) - i64::from(right.iso_week().year())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_monday_and_sunday_boundaries_differently() {
        let sunday = date(2024, 1, 7);
        let monday = date(2024, 1, 8);
        assert_eq!(
            difference_in_calendar_weeks(monday, sunday, Weekday::Mon),
            1
        );
        assert_eq!(
            difference_in_calendar_weeks(monday, sunday, Weekday::Sun),
            0
        );
        assert_eq!(
            difference_in_calendar_weeks(sunday, monday, Weekday::Mon),
            -1
        );
        assert_eq!(
            difference_in_calendar_weeks(sunday, sunday, Weekday::Mon),
            0
        );
    }

    #[test]
    fn counts_cross_year_partial_weeks() {
        let last = date(2023, 12, 31);
        let first = date(2024, 1, 1);
        assert_eq!(difference_in_calendar_weeks(first, last, Weekday::Mon), 1);
        assert_eq!(difference_in_calendar_weeks(first, last, Weekday::Sun), 0);
    }

    #[test]
    fn supports_minimum_and_maximum_dates_without_overflow() {
        for starts_on in [Weekday::Mon, Weekday::Sun, Weekday::Sat] {
            let forward = difference_in_calendar_weeks(NaiveDate::MAX, NaiveDate::MIN, starts_on);
            let backward = difference_in_calendar_weeks(NaiveDate::MIN, NaiveDate::MAX, starts_on);
            assert!(forward > 0);
            assert_eq!(forward, -backward);
        }
    }
    #[test]
    fn observes_iso_year_rollovers_in_december_and_january() {
        assert_eq!(
            difference_in_calendar_iso_week_years(date(2018, 12, 31), date(2018, 12, 30)),
            1
        );
        assert_eq!(
            difference_in_calendar_iso_week_years(date(2021, 1, 1), date(2020, 12, 31)),
            0
        );
        assert_eq!(
            difference_in_calendar_iso_week_years(date(2020, 1, 1), date(2019, 12, 29)),
            1
        );
    }

    #[test]
    fn is_antisymmetric_at_date_limits() {
        let count = difference_in_calendar_iso_week_years(NaiveDate::MAX, NaiveDate::MIN);
        assert!(count > 0);
        assert_eq!(
            difference_in_calendar_iso_week_years(NaiveDate::MIN, NaiveDate::MAX),
            -count
        );
    }
}
