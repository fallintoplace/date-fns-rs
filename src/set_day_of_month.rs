//! Checked day-of-month replacement.

use chrono::{Datelike, NaiveDate};

/// Replace the 1-based day of the month without changing the month or year.
///
/// Invalid days, including February 29 in common years, return None.
/// No overflow into another month occurs.
pub fn set_day_of_month(date: NaiveDate, day: u32) -> Option<NaiveDate> {
    date.with_day(day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn replaces_days_in_month_without_changing_the_month() {
        let leap = date(2024, 2, 10);
        assert_eq!(set_day_of_month(leap, 29), Some(date(2024, 2, 29)));
        assert_eq!(
            set_day_of_month(date(2024, 4, 1), 30),
            Some(date(2024, 4, 30))
        );
        assert_eq!(
            set_day_of_month(date(2024, 1, 31), 1),
            Some(date(2024, 1, 1))
        );
    }

    #[test]
    fn rejects_invalid_day_numbers() {
        assert_eq!(set_day_of_month(date(2023, 2, 1), 29), None);
        assert_eq!(set_day_of_month(date(2024, 4, 15), 31), None);
        assert_eq!(set_day_of_month(date(2024, 1, 1), 0), None);
        assert_eq!(set_day_of_month(date(2024, 1, 1), u32::MAX), None);
    }

    #[test]
    fn works_at_chrono_limits() {
        assert_eq!(
            set_day_of_month(NaiveDate::MIN, NaiveDate::MIN.day()),
            Some(NaiveDate::MIN)
        );
        assert_eq!(
            set_day_of_month(NaiveDate::MAX, NaiveDate::MAX.day()),
            Some(NaiveDate::MAX)
        );
    }
}
