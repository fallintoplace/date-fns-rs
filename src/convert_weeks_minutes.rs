//! Checked whole-unit conversions between weeks and minutes.

/// Convert whole weeks to minutes, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn weeks_to_minutes(weeks: i64) -> Option<i64> {
    weeks.checked_mul(10_080)
}

/// Convert minutes to complete weeks, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn minutes_to_weeks(minutes: i64) -> i64 {
    minutes / 10_080
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(minutes_to_weeks(0), 0);
        assert_eq!(minutes_to_weeks(10_080 * 2), 2);
        assert_eq!(minutes_to_weeks(10_080 * 2 - 1), 1);
        assert_eq!(minutes_to_weeks(-(10_080 * 2 - 1)), -1);
        assert_eq!(minutes_to_weeks(10_080 - 1), 0);
        assert_eq!(minutes_to_weeks(-(10_080 - 1)), 0);
        assert_eq!(minutes_to_weeks(i64::MAX), i64::MAX / 10_080);
        assert_eq!(minutes_to_weeks(i64::MIN), i64::MIN / 10_080);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(weeks_to_minutes(0), Some(0));
        assert_eq!(weeks_to_minutes(2), Some(10_080 * 2));
        assert_eq!(weeks_to_minutes(-2), Some(-10_080 * 2));
        assert_eq!(weeks_to_minutes(i64::MAX), None);
        assert_eq!(weeks_to_minutes(i64::MIN), None);
        let largest = i64::MAX / 10_080;
        let smallest = i64::MIN / 10_080;
        assert_eq!(weeks_to_minutes(largest), largest.checked_mul(10_080));
        assert_eq!(weeks_to_minutes(smallest), smallest.checked_mul(10_080));
        assert_eq!(weeks_to_minutes(largest + 1), None);
        assert_eq!(weeks_to_minutes(smallest - 1), None);
    }
}
