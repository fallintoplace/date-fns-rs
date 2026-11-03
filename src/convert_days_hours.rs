//! Checked whole-unit conversions between days and hours.

/// Convert whole days to hours, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn days_to_hours(days: i64) -> Option<i64> {
    days.checked_mul(24)
}

/// Convert hours to complete days, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn hours_to_days(hours: i64) -> i64 {
    hours / 24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(hours_to_days(0), 0);
        assert_eq!(hours_to_days(24 * 2), 2);
        assert_eq!(hours_to_days(24 * 2 - 1), 1);
        assert_eq!(hours_to_days(-(24 * 2 - 1)), -1);
        assert_eq!(hours_to_days(24 - 1), 0);
        assert_eq!(hours_to_days(-(24 - 1)), 0);
        assert_eq!(hours_to_days(i64::MAX), i64::MAX / 24);
        assert_eq!(hours_to_days(i64::MIN), i64::MIN / 24);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(days_to_hours(0), Some(0));
        assert_eq!(days_to_hours(2), Some(24 * 2));
        assert_eq!(days_to_hours(-2), Some(-24 * 2));
        assert_eq!(days_to_hours(i64::MAX), None);
        assert_eq!(days_to_hours(i64::MIN), None);
        let largest = i64::MAX / 24;
        let smallest = i64::MIN / 24;
        assert_eq!(days_to_hours(largest), largest.checked_mul(24));
        assert_eq!(days_to_hours(smallest), smallest.checked_mul(24));
        assert_eq!(days_to_hours(largest + 1), None);
        assert_eq!(days_to_hours(smallest - 1), None);
    }
}
