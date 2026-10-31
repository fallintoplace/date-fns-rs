//! Checked replacement of the day within an existing calendar month.

use chrono::{Datelike, NaiveDate};

/// Replace the day of the month while preserving the month and year.
///
/// Returns None if `day` is zero or exceeds the length of the month.
/// Unlike JavaScript Date setters, this does not roll into another month.
pub fn set_day_of_month(date: NaiveDate, day: u32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(date.year(), date.month(), day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn replaces_day_without_changing_month_or_year() {
        let original = date(2024, 3, 31);
        assert_eq!(set_day_of_month(original, 1), Some(date(2024, 3, 1)));
        assert_eq!(set_day_of_month(original, 29), Some(date(2024, 3, 29)));
        assert_eq!(set_day_of_month(original, 31), Some(original));
    }

    #[test]
    fn rejects_impossible_month_days() {
        assert_eq!(set_day_of_month(date(2024, 2, 29), 30), None);
        assert_eq!(set_day_of_month(date(2023, 2, 28), 29), None);
        assert_eq!(set_day_of_month(date(2024, 4, 1), 31), None);
        assert_eq!(set_day_of_month(date(2024, 4, 1), 0), None);
        assert_eq!(set_day_of_month(date(2024, 4, 1), u32::MAX), None);
        assert_eq!(set_day_of_month(date(2024, 2, 1), 29), Some(date(2024, 2, 29)));
    }

    #[test]
    fn accepts_identity_at_chrono_date_limits() {
        assert_eq!(set_day_of_month(NaiveDate::MIN, NaiveDate::MIN.day()), Some(NaiveDate::MIN));
        assert_eq!(set_day_of_month(NaiveDate::MAX, NaiveDate::MAX.day()), Some(NaiveDate::MAX));
    }
}
