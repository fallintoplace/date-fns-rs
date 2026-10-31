//! Checked month replacement with day-of-month clamping.

use chrono::{Datelike, NaiveDate};

use crate::metadata::get_days_in_month;

/// Replace the 1-based month while preserving the year and day when possible.
///
/// The day clamps to the target month's last day. Invalid month numbers,
/// including zero and values above twelve, return None.
pub fn set_month(date: NaiveDate, month: u32) -> Option<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(date.year(), month, 1)?;
    let day = date.day().min(get_days_in_month(first));
    NaiveDate::from_ymd_opt(date.year(), month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn clamps_at_shorter_month_ends() {
        let leap_january = date(2024, 1, 31);
        let normal_january = date(2023, 1, 31);
        assert_eq!(set_month(leap_january, 2), Some(date(2024, 2, 29)));
        assert_eq!(set_month(normal_january, 2), Some(date(2023, 2, 28)));
        assert_eq!(set_month(date(2024, 5, 31), 4), Some(date(2024, 4, 30)));
    }

    #[test]
    fn validates_month_and_preserves_same_month() {
        let start = date(2024, 7, 15);
        assert_eq!(set_month(start, 7), Some(start));
        assert_eq!(set_month(start, 0), None);
        assert_eq!(set_month(start, 13), None);
        assert_eq!(set_month(start, u32::MAX), None);
    }

    #[test]
    fn works_at_supported_date_limits() {
        assert_eq!(set_month(NaiveDate::MIN, 1), Some(NaiveDate::MIN));
        assert_eq!(set_month(NaiveDate::MAX, 12), Some(NaiveDate::MAX));
    }
}
