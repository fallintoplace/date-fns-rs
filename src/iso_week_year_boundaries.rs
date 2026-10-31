//! Boundaries of ISO week-numbering years, which can cross calendar years.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::iso_weeks::get_iso_weeks_in_year;

/// Return the Monday starting the ISO week-numbering year containing date.
///
/// Returns None if that Monday is outside Chrono's supported range.
pub fn start_of_iso_week_year(date: NaiveDate) -> Option<NaiveDate> {
    NaiveDate::from_isoywd_opt(date.iso_week().year(), 1, Weekday::Mon)
}

/// Return the Sunday ending the ISO week-numbering year containing date.
///
/// ISO years have either 52 or 53 weeks. Returns None if the boundary is
/// outside Chrono's supported date range.
pub fn end_of_iso_week_year(date: NaiveDate) -> Option<NaiveDate> {
    let year = date.iso_week().year();
    let weeks = get_iso_weeks_in_year(year)?;
    NaiveDate::from_isoywd_opt(year, weeks, Weekday::Sun)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_53_week_years_crossing_both_calendar_boundaries() {
        let new_years_day = date(2021, 1, 1);
        assert_eq!(
            start_of_iso_week_year(new_years_day),
            Some(date(2019, 12, 30))
        );
        assert_eq!(end_of_iso_week_year(new_years_day), Some(date(2021, 1, 3)));
    }

    #[test]
    fn handles_52_week_years_and_december_week_one() {
        let start = date(2024, 7, 1);
        assert_eq!(start_of_iso_week_year(start), Some(date(2024, 1, 1)));
        assert_eq!(end_of_iso_week_year(start), Some(date(2024, 12, 29)));
        let next_week_year = date(2018, 12, 31);
        assert_eq!(
            start_of_iso_week_year(next_week_year),
            Some(date(2018, 12, 31))
        );
        assert_eq!(
            end_of_iso_week_year(next_week_year),
            Some(date(2019, 12, 29))
        );
    }
}
