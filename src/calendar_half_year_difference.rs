//! Signed differences between calendar half-year labels.

use chrono::{Datelike, NaiveDate};

/// Count half-year boundaries from right to left.
///
/// Dates in the same January-June or July-December period differ by zero.
/// Results are signed and do not depend on the days within either period.
pub fn difference_in_calendar_half_years(left: NaiveDate, right: NaiveDate) -> i64 {
    fn index(date: NaiveDate) -> i64 {
        i64::from(date.year()) * 2 + i64::from((date.month() - 1) / 6)
    }

    index(left) - index(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_boundaries_in_both_directions() {
        let june = date(2024, 6, 30);
        let july = date(2024, 7, 1);
        assert_eq!(difference_in_calendar_half_years(july, june), 1);
        assert_eq!(difference_in_calendar_half_years(june, july), -1);
        assert_eq!(difference_in_calendar_half_years(june, date(2024, 1, 1)), 0);
        assert_eq!(difference_in_calendar_half_years(date(2025, 1, 1), june), 2);
    }

    #[test]
    fn works_across_year_zero_and_supported_limits() {
        assert_eq!(
            difference_in_calendar_half_years(date(0, 1, 1), date(-1, 12, 31)),
            1
        );
        assert_eq!(
            difference_in_calendar_half_years(NaiveDate::MAX, NaiveDate::MAX),
            0
        );
        assert!(difference_in_calendar_half_years(NaiveDate::MAX, NaiveDate::MIN) > 0);
    }
}
