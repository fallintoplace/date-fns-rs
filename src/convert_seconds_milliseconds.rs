//! Checked integer conversions between milliseconds and seconds.

/// Convert milliseconds to complete seconds, truncating partial units toward zero.
pub fn milliseconds_to_seconds(milliseconds: i64) -> i64 {
    milliseconds / 1000
}

/// Convert seconds to milliseconds, returning None on integer overflow.
pub fn seconds_to_milliseconds(seconds: i64) -> Option<i64> {
    seconds.checked_mul(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(milliseconds_to_seconds(0), 0);
        assert_eq!(milliseconds_to_seconds(2000), 2);
        assert_eq!(milliseconds_to_seconds(1999), 1);
        assert_eq!(milliseconds_to_seconds(-1999), -1);
        assert_eq!(milliseconds_to_seconds(999), 0);
        assert_eq!(milliseconds_to_seconds(-999), 0);
        assert_eq!(milliseconds_to_seconds(i64::MAX), i64::MAX / 1000);
        assert_eq!(milliseconds_to_seconds(i64::MIN), i64::MIN / 1000);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(seconds_to_milliseconds(0), Some(0));
        assert_eq!(seconds_to_milliseconds(2), Some(2000));
        assert_eq!(seconds_to_milliseconds(-2), Some(-2000));
        assert_eq!(seconds_to_milliseconds(i64::MAX), None);
        assert_eq!(seconds_to_milliseconds(i64::MIN), None);
        let max_safe = i64::MAX / 1000;
        let min_safe = i64::MIN / 1000;
        assert_eq!(seconds_to_milliseconds(max_safe), max_safe.checked_mul(1000));
        assert_eq!(seconds_to_milliseconds(max_safe + 1), None);
        assert_eq!(seconds_to_milliseconds(min_safe), min_safe.checked_mul(1000));
        assert_eq!(seconds_to_milliseconds(min_safe - 1), None);
    }
}
