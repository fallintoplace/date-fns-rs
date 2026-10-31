//! Signed differences between calendar-quarter indices.

use chrono::{Datelike, NaiveDate};

fn quarter_index(date: NaiveDate) -> i64 {
    i64::from(date.year()) * 4 + (i64::from(date.month()) - 1) / 3
}

/// Count calendar quarter boundaries from `right` to `left`.
///
/// The day of the month does not matter. April 1 minus March 31 is one quarter.
pub fn difference_in_calendar_quarters(left: NaiveDate, right: NaiveDate) -> i64 {
    quarter_index(left) - quarter_index(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_quarter_boundaries_not_completed_quarters() {
        assert_eq!(
            difference_in_calendar_quarters(date(2024, 4, 1), date(2024, 3, 31)),
            1
        );
        assert_eq!(
            difference_in_calendar_quarters(date(2024, 3, 31), date(2024, 1, 1)),
            0
        );
        assert_eq!(
            difference_in_calendar_quarters(date(2025, 1, 1), date(2024, 12, 31)),
            1
        );
    }

    #[test]
    fn handles_negative_differences_and_date_limits() {
        assert_eq!(
            difference_in_calendar_quarters(date(2023, 12, 31), date(2024, 1, 1)),
            -1
        );
        let forward = difference_in_calendar_quarters(NaiveDate::MAX, NaiveDate::MIN);
        assert!(forward > 0);
        assert_eq!(
            difference_in_calendar_quarters(NaiveDate::MIN, NaiveDate::MAX),
            -forward
        );
    }
}
