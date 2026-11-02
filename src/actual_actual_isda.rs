//! Actual/Actual ISDA accrual fractions split at calendar year boundaries.

use chrono::{Datelike, NaiveDate};

/// Return the signed Actual/Actual ISDA year fraction for [start, end).
///
/// Each calendar-year segment uses 365 or 366 as its denominator. Reversed
/// intervals produce the negative result. No fixed-day approximation is used.
pub fn actual_actual_isda_year_fraction(start: NaiveDate, end: NaiveDate) -> f64 {
    if start > end {
        return -actual_actual_isda_year_fraction(end, start);
    }
    let mut cursor = start;
    let mut fraction = 0.0;
    while cursor < end {
        let next_year = cursor
            .year()
            .checked_add(1)
            .and_then(|year| NaiveDate::from_ymd_opt(year, 1, 1))
            .unwrap_or(end);
        let next = end.min(next_year);
        let days = next.signed_duration_since(cursor).num_days() as f64;
        let denominator = if crate::is_leap_year(cursor.year()) {
            366.0
        } else {
            365.0
        };
        fraction += days / denominator;
        cursor = next;
    }
    fraction
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn weights_each_year_using_its_actual_length() {
        assert_eq!(
            actual_actual_isda_year_fraction(date(2024, 1, 1), date(2025, 1, 1)),
            1.0
        );
        let fraction = actual_actual_isda_year_fraction(date(2023, 7, 1), date(2024, 7, 1));
        let expected = 184.0 / 365.0 + 182.0 / 366.0;
        assert!((fraction - expected).abs() < 1e-12);
    }

    #[test]
    fn is_antisymmetric_and_supports_chrono_limits() {
        let left = date(1999, 12, 30);
        let right = date(2001, 1, 2);
        let forward = actual_actual_isda_year_fraction(left, right);
        assert!((forward + actual_actual_isda_year_fraction(right, left)).abs() < 1e-12);
        assert_eq!(actual_actual_isda_year_fraction(left, left), 0.0);
        assert!(
            actual_actual_isda_year_fraction(NaiveDate::MAX.pred_opt().unwrap(), NaiveDate::MAX)
                > 0.0
        );
    }
}
