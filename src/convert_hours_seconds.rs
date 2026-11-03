//! Checked integer conversions between seconds and hours.

/// Convert seconds to complete hours, truncating partial units toward zero.
pub fn seconds_to_hours(seconds: i64) -> i64 {
    seconds / 3600
}

/// Convert hours to seconds, returning None on integer overflow.
pub fn hours_to_seconds(hours: i64) -> Option<i64> {
    hours.checked_mul(3600)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(seconds_to_hours(0), 0);
        assert_eq!(seconds_to_hours(7200), 2);
        assert_eq!(seconds_to_hours(7199), 1);
        assert_eq!(seconds_to_hours(-7199), -1);
        assert_eq!(seconds_to_hours(3599), 0);
        assert_eq!(seconds_to_hours(-3599), 0);
        assert_eq!(seconds_to_hours(i64::MAX), i64::MAX / 3600);
        assert_eq!(seconds_to_hours(i64::MIN), i64::MIN / 3600);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(hours_to_seconds(0), Some(0));
        assert_eq!(hours_to_seconds(2), Some(7200));
        assert_eq!(hours_to_seconds(-2), Some(-7200));
        assert_eq!(hours_to_seconds(i64::MAX), None);
        assert_eq!(hours_to_seconds(i64::MIN), None);
        let max_safe = i64::MAX / 3600;
        let min_safe = i64::MIN / 3600;
        assert_eq!(hours_to_seconds(max_safe), max_safe.checked_mul(3600));
        assert_eq!(hours_to_seconds(max_safe + 1), None);
        assert_eq!(hours_to_seconds(min_safe), min_safe.checked_mul(3600));
        assert_eq!(hours_to_seconds(min_safe - 1), None);
    }
}
