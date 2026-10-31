//! Checked year replacement, including leap-day clamping.

use chrono::{Datelike, NaiveDate};

use crate::metadata::get_days_in_month;

/// Replace the calendar year, preserving the month and day where valid.
///
/// February 29 becomes February 28 in non-leap target years. Years outside
/// Chrono's supported date range return None without wrapping.
pub fn set_year(date: NaiveDate, year: i32) -> Option<NaiveDate> {
    let month = date.month();
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let day = date.day().min(get_days_in_month(first));
    NaiveDate::from_ymd_opt(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn clamps_leap_days_in_nonleap_years() {
        let leap_day = date(2024, 2, 29);
        assert_eq!(set_year(leap_day, 2025), Some(date(2025, 2, 28)));
        assert_eq!(set_year(leap_day, 2020), Some(date(2020, 2, 29)));
        assert_eq!(set_year(leap_day, 2000), Some(date(2000, 2, 29)));
        assert_eq!(set_year(leap_day, 1900), Some(date(1900, 2, 28)));
    }

    #[test]
    fn preserves_other_months_and_rejects_invalid_years() {
        let date = date(2024, 12, 31);
        assert_eq!(
            set_year(date, 2023),
            Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap())
        );
        assert_eq!(set_year(date, i32::MAX), None);
        assert_eq!(set_year(date, i32::MIN), None);
    }

    #[test]
    fn identity_works_at_date_limits() {
        assert_eq!(
            set_year(NaiveDate::MIN, NaiveDate::MIN.year()),
            Some(NaiveDate::MIN)
        );
        assert_eq!(
            set_year(NaiveDate::MAX, NaiveDate::MAX.year()),
            Some(NaiveDate::MAX)
        );
    }
}
