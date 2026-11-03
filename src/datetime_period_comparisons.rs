//! Calendar quarter and half-year comparisons for naive date-times.

use chrono::{Datelike, NaiveDateTime};

/// Whether both timestamps belong to the same Gregorian quarter.
pub fn is_same_quarter_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    left.year() == right.year() && crate::get_quarter(left.date()) == crate::get_quarter(right.date())
}

/// Whether both timestamps belong to the same January-June or July-December half.
pub fn is_same_half_year_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    left.year() == right.year()
        && crate::get_half_year(left.date()) == crate::get_half_year(right.date())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn datetime(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
    }

    #[test]
    fn quarter_checks_ignore_clock_time_and_cross_months() {
        let start = datetime(2024, 1, 1);
        let end = datetime(2024, 3, 31);
        assert!(is_same_quarter_datetime(start, end));
        assert!(!is_same_quarter_datetime(end, datetime(2024, 4, 1)));
        assert!(!is_same_quarter_datetime(start, datetime(2025, 1, 1)));
    }

    #[test]
    fn half_year_checks_cross_leap_days_but_not_years() {
        assert!(is_same_half_year_datetime(
            datetime(2024, 2, 29),
            datetime(2024, 6, 30)
        ));
        assert!(!is_same_half_year_datetime(
            datetime(2024, 6, 30),
            datetime(2024, 7, 1)
        ));
        assert!(!is_same_half_year_datetime(
            datetime(2024, 12, 31),
            datetime(2025, 1, 1)
        ));
    }

    #[test]
    fn supported_limits_compare_with_themselves() {
        for value in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            assert!(is_same_quarter_datetime(value, value));
            assert!(is_same_half_year_datetime(value, value));
        }
    }
}
