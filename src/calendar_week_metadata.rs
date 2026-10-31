//! Calendar week numbers with explicit week numbering conventions.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::boundaries::is_leap_year;

fn first_week_start(
    jan1: i64,
    jan1_weekday: i64,
    week_starts_on: i64,
    first_week_contains_date: i64,
) -> i64 {
    let anchor = first_week_contains_date - 1;
    let anchor_weekday = (jan1_weekday + anchor).rem_euclid(7);
    jan1 + anchor - (anchor_weekday - week_starts_on).rem_euclid(7)
}

fn week_and_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<(i32, u32)> {
    if !(1..=7).contains(&first_week_contains_date) {
        return None;
    }

    let year = date.year();
    let jan1 = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let jan1_day = i64::from(jan1.num_days_from_ce());
    let jan1_weekday = i64::from(jan1.weekday().num_days_from_monday());
    let first = i64::from(first_week_contains_date);
    let starts_on = i64::from(week_starts_on.num_days_from_monday());
    let current_start = first_week_start(jan1_day, jan1_weekday, starts_on, first);
    let day = i64::from(date.num_days_from_ce());

    let (week_year, week_start) = if day < current_start {
        let previous_year = year.checked_sub(1)?;
        let previous_days = if is_leap_year(previous_year) { 366 } else { 365 };
        let previous_jan1 = jan1_day - previous_days;
        let previous_weekday = (jan1_weekday - previous_days).rem_euclid(7);
        (
            previous_year,
            first_week_start(previous_jan1, previous_weekday, starts_on, first),
        )
    } else {
        let current_days = if is_leap_year(year) { 366 } else { 365 };
        let next_jan1 = jan1_day + current_days;
        let next_weekday = (jan1_weekday + current_days).rem_euclid(7);
        let next_start = first_week_start(next_jan1, next_weekday, starts_on, first);
        if day >= next_start {
            (year.checked_add(1)?, next_start)
        } else {
            (year, current_start)
        }
    };

    let week = (day - week_start).div_euclid(7) + 1;
    Some((week_year, u32::try_from(week).ok()?))
}

/// Return the 1-based week number using an explicit week convention.
///
/// `first_week_contains_date` must be between 1 and 7. For ISO numbering,
/// use Monday and 4; for common US numbering, use Sunday and 1.
pub fn get_week(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<u32> {
    week_and_year(date, week_starts_on, first_week_contains_date).map(|(_, week)| week)
}

/// Return the week-numbering year for a configurable week convention.
///
/// The week-year may differ from the calendar year around New Year's Day.
pub fn get_week_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<i32> {
    week_and_year(date, week_starts_on, first_week_contains_date).map(|(year, _)| year)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_iso_week_numbers_across_year_boundaries() {
        for year in 2000..=2030 {
            for month in 1..=12 {
                for day in [1, 7, 15, 28] {
                    let day = date(year, month, day);
                    assert_eq!(get_week(day, Weekday::Mon, 4), Some(day.iso_week().week()));
                    assert_eq!(
                        get_week_year(day, Weekday::Mon, 4),
                        Some(day.iso_week().year())
                    );
                }
            }
        }
    }

    #[test]
    fn supports_sunday_first_week_years() {
        let new_year = date(2020, 12, 31);
        assert_eq!(get_week(new_year, Weekday::Sun, 1), Some(1));
        assert_eq!(get_week_year(new_year, Weekday::Sun, 1), Some(2021));
        assert_eq!(get_week(date(2021, 1, 3), Weekday::Sun, 1), Some(2));
        assert_eq!(get_week(date(2021, 1, 1), Weekday::Mon, 4), Some(53));
        assert_eq!(get_week_year(date(2021, 1, 1), Weekday::Mon, 4), Some(2020));
    }

    #[test]
    fn rejects_invalid_conventions_and_supports_date_limits() {
        assert_eq!(get_week(date(2024, 1, 1), Weekday::Mon, 0), None);
        assert_eq!(get_week_year(date(2024, 1, 1), Weekday::Mon, 8), None);
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let week = get_week(edge, Weekday::Mon, 4).unwrap();
            assert!((1..=53).contains(&week));
            assert!(get_week_year(edge, Weekday::Sun, 1).is_some());
        }
    }
}
