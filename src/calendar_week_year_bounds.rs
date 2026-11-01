//! Week-year boundaries under configurable calendar conventions.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::calendar_week_metadata::get_week_year;
use crate::weeks::start_of_week;

/// Start of the week-numbering year containing `date`.
///
/// `first_week_contains_date` must be in 1..=7. Use Monday and 4
/// for ISO 8601, or Sunday and 1 for common US-style calendars.
pub fn start_of_week_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDate> {
    let year = get_week_year(date, week_starts_on, first_week_contains_date)?;
    let anchor = NaiveDate::from_ymd_opt(year, 1, first_week_contains_date)?;
    start_of_week(anchor, week_starts_on)
}

/// Last day of the week-numbering year containing `date`.
///
/// Returns None when the next week-year boundary is not representable.
pub fn end_of_week_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDate> {
    let year = get_week_year(date, week_starts_on, first_week_contains_date)?;
    let next_year = year.checked_add(1)?;
    let anchor = NaiveDate::from_ymd_opt(next_year, 1, first_week_contains_date)?;
    start_of_week(anchor, week_starts_on)?.pred_opt()
}

/// Count full weeks in a configurable week-numbering year.
///
/// Returns 52 or 53 for representable full year boundaries.
pub fn get_weeks_in_week_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<u32> {
    let start = start_of_week_year(date, week_starts_on, first_week_contains_date)?;
    let end = end_of_week_year(date, week_starts_on, first_week_contains_date)?;
    let days = i64::from(end.num_days_from_ce()) - i64::from(start.num_days_from_ce()) + 1;
    u32::try_from(days / 7).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn resolves_iso_week_year_2020_and_its_53_weeks() {
        let day = date(2021, 1, 1);
        assert_eq!(
            start_of_week_year(day, Weekday::Mon, 4),
            Some(date(2019, 12, 30))
        );
        assert_eq!(
            end_of_week_year(day, Weekday::Mon, 4),
            Some(date(2021, 1, 3))
        );
        assert_eq!(get_weeks_in_week_year(day, Weekday::Mon, 4), Some(53));
        assert_eq!(
            get_weeks_in_week_year(date(2021, 6, 1), Weekday::Mon, 4),
            Some(52)
        );
    }

    #[test]
    fn honors_sunday_first_conventions() {
        let day = date(2021, 1, 1);
        assert_eq!(
            start_of_week_year(day, Weekday::Sun, 1),
            Some(date(2020, 12, 27))
        );
        assert_eq!(
            end_of_week_year(day, Weekday::Sun, 1),
            Some(date(2021, 12, 25))
        );
        assert_eq!(get_weeks_in_week_year(day, Weekday::Sun, 1), Some(52));
    }

    #[test]
    fn validates_first_week_and_date_limits() {
        let day = date(2024, 5, 1);
        assert_eq!(start_of_week_year(day, Weekday::Mon, 0), None);
        assert_eq!(end_of_week_year(day, Weekday::Mon, 8), None);
        assert_eq!(get_weeks_in_week_year(day, Weekday::Mon, 0), None);
        assert_eq!(start_of_week_year(NaiveDate::MIN, Weekday::Sun, 7), None);
        assert_eq!(end_of_week_year(NaiveDate::MAX, Weekday::Mon, 4), None);
    }
}
