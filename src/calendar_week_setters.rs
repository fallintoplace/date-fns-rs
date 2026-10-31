//! Checked week-number and week-year replacement for configurable calendars.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::arithmetic::{add_days, sub_days};
use crate::calendar_week_metadata::{get_week, get_week_year};
use crate::difference::difference_in_calendar_days;

fn first_week_start(
    year: i32,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDate> {
    if !(1..=7).contains(&first_week_contains_date) {
        return None;
    }
    let anchor = NaiveDate::from_ymd_opt(year, 1, first_week_contains_date)?;
    let offset = (i64::from(anchor.weekday().num_days_from_monday())
        - i64::from(week_starts_on.num_days_from_monday()))
    .rem_euclid(7);
    sub_days(anchor, offset)
}

fn with_week(
    date: NaiveDate,
    week_year: i32,
    week: u32,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDate> {
    let first = first_week_start(week_year, week_starts_on, first_week_contains_date)?;
    let next = first_week_start(
        week_year.checked_add(1)?,
        week_starts_on,
        first_week_contains_date,
    )?;
    let weeks = difference_in_calendar_days(next, first) / 7;
    if week == 0 || i64::from(week) > weeks {
        return None;
    }
    let weekday = (i64::from(date.weekday().num_days_from_monday())
        - i64::from(week_starts_on.num_days_from_monday()))
    .rem_euclid(7);
    add_days(first, (i64::from(week) - 1) * 7 + weekday)
}

/// Change a date's week number while preserving its week-year and weekday.
///
/// The anchor day is between 1 and 7. Use (Mon, 4) for ISO weeks.
pub fn set_week(
    date: NaiveDate,
    week: u32,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDate> {
    let week_year = get_week_year(date, week_starts_on, first_week_contains_date)?;
    with_week(
        date,
        week_year,
        week,
        week_starts_on,
        first_week_contains_date,
    )
}

/// Change the week-numbering year while preserving week number and weekday.
///
/// Returns None if the target year has no corresponding week, such as week
/// 53 in a 52-week year.
pub fn set_week_year(
    date: NaiveDate,
    week_year: i32,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDate> {
    let week = get_week(date, week_starts_on, first_week_contains_date)?;
    with_week(
        date,
        week_year,
        week,
        week_starts_on,
        first_week_contains_date,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn preserves_weekday_and_rejects_nonexistent_iso_weeks() {
        let friday = date(2021, 1, 1);
        assert_eq!(set_week(friday, 1, Weekday::Mon, 4), Some(date(2020, 1, 3)));
        assert_eq!(set_week(friday, 53, Weekday::Mon, 4), Some(friday));
        assert_eq!(set_week(friday, 0, Weekday::Mon, 4), None);
        assert_eq!(set_week(friday, 54, Weekday::Mon, 4), None);
        assert_eq!(
            set_week_year(friday, 2015, Weekday::Mon, 4),
            Some(date(2016, 1, 1))
        );
        assert_eq!(set_week_year(friday, 2021, Weekday::Mon, 4), None);
    }

    #[test]
    fn supports_sunday_first_week_conventions() {
        let friday = date(2021, 1, 1);
        assert_eq!(set_week(friday, 2, Weekday::Sun, 1), Some(date(2021, 1, 8)));
        assert_eq!(
            set_week_year(friday, 2020, Weekday::Sun, 1),
            Some(date(2020, 1, 3))
        );
        assert_eq!(set_week(friday, 1, Weekday::Sun, 0), None);
        assert_eq!(set_week_year(friday, 2022, Weekday::Sun, 8), None);
        assert_eq!(set_week_year(friday, i32::MAX, Weekday::Sun, 1), None);
    }
}
