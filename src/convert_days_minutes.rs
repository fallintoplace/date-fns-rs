//! Checked whole-unit conversions between days and minutes.

/// Convert whole days to minutes, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn days_to_minutes(days: i64) -> Option<i64> {
    days.checked_mul(1_440)
}

/// Convert minutes to complete days, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn minutes_to_days(minutes: i64) -> i64 {
    minutes / 1_440
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(minutes_to_days(0), 0);
        assert_eq!(minutes_to_days(1_440 * 2), 2);
        assert_eq!(minutes_to_days(1_440 * 2 - 1), 1);
        assert_eq!(minutes_to_days(-(1_440 * 2 - 1)), -1);
        assert_eq!(minutes_to_days(1_440 - 1), 0);
        assert_eq!(minutes_to_days(-(1_440 - 1)), 0);
        assert_eq!(minutes_to_days(i64::MAX), i64::MAX / 1_440);
        assert_eq!(minutes_to_days(i64::MIN), i64::MIN / 1_440);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(days_to_minutes(0), Some(0));
        assert_eq!(days_to_minutes(2), Some(1_440 * 2));
        assert_eq!(days_to_minutes(-2), Some(-1_440 * 2));
        assert_eq!(days_to_minutes(i64::MAX), None);
        assert_eq!(days_to_minutes(i64::MIN), None);
        let largest = i64::MAX / 1_440;
        let smallest = i64::MIN / 1_440;
        assert_eq!(days_to_minutes(largest), largest.checked_mul(1_440));
        assert_eq!(days_to_minutes(smallest), smallest.checked_mul(1_440));
        assert_eq!(days_to_minutes(largest + 1), None);
        assert_eq!(days_to_minutes(smallest - 1), None);
    }
}
