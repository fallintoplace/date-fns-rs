//! Checked arithmetic in ISO week-numbering years.

use chrono::{Datelike, NaiveDate};

/// Shift ISO week-numbering years, retaining the week number and weekday.
///
/// A week-53 date cannot move into a year with only 52 ISO weeks.
/// Returns None on invalid targets and arithmetic overflow.
pub fn add_iso_week_years(date: NaiveDate, years: i32) -> Option<NaiveDate> {
    let target = date.iso_week().year().checked_add(years)?;
    crate::set_iso_week_year(date, target)
}

/// Subtract ISO week-numbering years with checked arithmetic.
pub fn sub_iso_week_years(date: NaiveDate, years: i32) -> Option<NaiveDate> {
    let target = date.iso_week().year().checked_sub(years)?;
    crate::set_iso_week_year(date, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn shifts_iso_years_across_calendar_years() {
        let monday = date(2024, 1, 1);
        assert_eq!(add_iso_week_years(monday, 1), Some(date(2024, 12, 30)));
        assert_eq!(sub_iso_week_years(monday, 1), Some(date(2023, 1, 2)));
        assert_eq!(
            add_iso_week_years(monday, -1),
            sub_iso_week_years(monday, 1)
        );
        assert_eq!(
            sub_iso_week_years(monday, -1),
            add_iso_week_years(monday, 1)
        );
        assert_eq!(add_iso_week_years(monday, 0), Some(monday));
    }

    #[test]
    fn checks_missing_week_53() {
        let friday = date(2021, 1, 1);
        assert_eq!(add_iso_week_years(friday, 1), None);
        assert_eq!(sub_iso_week_years(friday, 5), Some(date(2016, 1, 1)));
        assert_eq!(add_iso_week_years(date(2016, 1, 1), 5), Some(friday));
    }

    #[test]
    fn checks_overflow_and_supported_date_bounds() {
        let day = date(2024, 5, 1);
        assert_eq!(add_iso_week_years(day, i32::MAX), None);
        assert_eq!(sub_iso_week_years(day, i32::MIN), None);
        assert_eq!(add_iso_week_years(NaiveDate::MAX, 1), None);
    }
}
