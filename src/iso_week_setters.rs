//! Checked setters for ISO week numbers and week-numbering years.

use chrono::{Datelike, NaiveDate};

/// Replace the ISO week number while preserving the ISO week-year and weekday.
///
/// Returns None when the target week is not present in the current ISO
/// week-numbering year or the result falls outside Chrono's supported range.
pub fn set_iso_week(date: NaiveDate, week: u32) -> Option<NaiveDate> {
    NaiveDate::from_isoywd_opt(date.iso_week().year(), week, date.weekday())
}

/// Replace the ISO week-numbering year, keeping the week and weekday.
///
/// Returns None when the target year does not contain the source week number.
/// For example, ISO week 53 cannot be moved to a 52-week ISO year.
pub fn set_iso_week_year(date: NaiveDate, year: i32) -> Option<NaiveDate> {
    NaiveDate::from_isoywd_opt(year, date.iso_week().week(), date.weekday())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn changes_iso_weeks_without_losing_the_weekday() {
        let last_week = date(2020, 12, 30);
        assert_eq!(set_iso_week(last_week, 1), Some(date(2020, 1, 1)));
        assert_eq!(set_iso_week(last_week, 53), Some(last_week));
        assert_eq!(set_iso_week(date(2024, 1, 1), 52), Some(date(2024, 12, 23)));
        assert_eq!(set_iso_week(date(2024, 1, 1), 53), None);
    }

    #[test]
    fn changes_iso_week_years_across_calendar_year_boundaries() {
        let friday = date(2021, 1, 1);
        assert_eq!(set_iso_week_year(friday, 2015), Some(date(2016, 1, 1)));
        assert_eq!(set_iso_week_year(friday, 2016), None);
        assert_eq!(
            set_iso_week_year(date(2024, 1, 1), 2025),
            Some(date(2024, 12, 30))
        );
    }

    #[test]
    fn rejects_invalid_weeks_and_out_of_range_years() {
        let day = date(2024, 5, 15);
        assert_eq!(set_iso_week(day, 0), None);
        assert_eq!(set_iso_week(day, 54), None);
        assert_eq!(set_iso_week_year(day, i32::MAX), None);
        assert_eq!(set_iso_week_year(day, i32::MIN), None);
    }
}
