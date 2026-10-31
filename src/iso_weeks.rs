//! ISO 8601 week-number and week-year metadata.

use chrono::{Datelike, NaiveDate};

/// Return the ISO week number (1 through 52 or 53) for this date.
pub fn get_iso_week(date: NaiveDate) -> u32 {
    date.iso_week().week()
}

/// Return the ISO week-numbering year, which may differ from calendar year.
pub fn get_iso_week_year(date: NaiveDate) -> i32 {
    date.iso_week().year()
}

/// Return the number of ISO weeks in a calendar year.
///
/// December 28 always belongs to the final ISO week of its calendar year.
/// Returns None if the requested year is outside Chrono's date range.
pub fn get_iso_weeks_in_year(year: i32) -> Option<u32> {
    let final_week = NaiveDate::from_ymd_opt(year, 12, 28)?;
    Some(get_iso_week(final_week))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn week_year_crosses_calendar_year_boundaries() {
        assert_eq!(get_iso_week(date(2021, 1, 1)), 53);
        assert_eq!(get_iso_week_year(date(2021, 1, 1)), 2020);
        assert_eq!(get_iso_week(date(2018, 12, 31)), 1);
        assert_eq!(get_iso_week_year(date(2018, 12, 31)), 2019);
        assert_eq!(get_iso_week(date(2024, 1, 1)), 1);
        assert_eq!(get_iso_week_year(date(2024, 1, 1)), 2024);
    }

    #[test]
    fn year_length_varies_between_52_and_53_weeks() {
        assert_eq!(get_iso_weeks_in_year(2015), Some(53));
        assert_eq!(get_iso_weeks_in_year(2020), Some(53));
        assert_eq!(get_iso_weeks_in_year(2021), Some(52));
        assert_eq!(get_iso_weeks_in_year(2024), Some(52));
        assert_eq!(get_iso_weeks_in_year(i32::MAX), None);
    }
}
