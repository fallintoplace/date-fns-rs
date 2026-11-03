//! Checked whole-unit conversions between days and seconds.

/// Convert whole days to seconds, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn days_to_seconds(days: i64) -> Option<i64> {
    days.checked_mul(86_400)
}

/// Convert seconds to complete days, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn seconds_to_days(seconds: i64) -> i64 {
    seconds / 86_400
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(seconds_to_days(0), 0);
        assert_eq!(seconds_to_days(86_400 * 2), 2);
        assert_eq!(seconds_to_days(86_400 * 2 - 1), 1);
        assert_eq!(seconds_to_days(-(86_400 * 2 - 1)), -1);
        assert_eq!(seconds_to_days(86_400 - 1), 0);
        assert_eq!(seconds_to_days(-(86_400 - 1)), 0);
        assert_eq!(seconds_to_days(i64::MAX), i64::MAX / 86_400);
        assert_eq!(seconds_to_days(i64::MIN), i64::MIN / 86_400);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(days_to_seconds(0), Some(0));
        assert_eq!(days_to_seconds(2), Some(86_400 * 2));
        assert_eq!(days_to_seconds(-2), Some(-86_400 * 2));
        assert_eq!(days_to_seconds(i64::MAX), None);
        assert_eq!(days_to_seconds(i64::MIN), None);
        let largest = i64::MAX / 86_400;
        let smallest = i64::MIN / 86_400;
        assert_eq!(days_to_seconds(largest), largest.checked_mul(86_400));
        assert_eq!(days_to_seconds(smallest), smallest.checked_mul(86_400));
        assert_eq!(days_to_seconds(largest + 1), None);
        assert_eq!(days_to_seconds(smallest - 1), None);
    }
}
