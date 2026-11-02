//! Checked integer conversions between seconds and minutes.

/// Convert seconds to complete minutes, truncating partial units toward zero.
pub fn seconds_to_minutes(seconds: i64) -> i64 {
    seconds / 60
}

/// Convert minutes to seconds, returning None on integer overflow.
pub fn minutes_to_seconds(minutes: i64) -> Option<i64> {
    minutes.checked_mul(60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(seconds_to_minutes(0), 0);
        assert_eq!(seconds_to_minutes(120), 2);
        assert_eq!(seconds_to_minutes(119), 1);
        assert_eq!(seconds_to_minutes(-119), -1);
        assert_eq!(seconds_to_minutes(59), 0);
        assert_eq!(seconds_to_minutes(-59), 0);
        assert_eq!(seconds_to_minutes(i64::MAX), i64::MAX / 60);
        assert_eq!(seconds_to_minutes(i64::MIN), i64::MIN / 60);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(minutes_to_seconds(0), Some(0));
        assert_eq!(minutes_to_seconds(2), Some(120));
        assert_eq!(minutes_to_seconds(-2), Some(-120));
        assert_eq!(minutes_to_seconds(i64::MAX), None);
        assert_eq!(minutes_to_seconds(i64::MIN), None);
        let max_safe = i64::MAX / 60;
        let min_safe = i64::MIN / 60;
        assert_eq!(minutes_to_seconds(max_safe), max_safe.checked_mul(60));
        assert_eq!(minutes_to_seconds(max_safe + 1), None);
        assert_eq!(minutes_to_seconds(min_safe), min_safe.checked_mul(60));
        assert_eq!(minutes_to_seconds(min_safe - 1), None);
    }
}
