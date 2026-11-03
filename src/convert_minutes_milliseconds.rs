//! Checked integer conversions between milliseconds and minutes.

/// Convert milliseconds to complete minutes, truncating partial units toward zero.
pub fn milliseconds_to_minutes(milliseconds: i64) -> i64 {
    milliseconds / 60000
}

/// Convert minutes to milliseconds, returning None on integer overflow.
pub fn minutes_to_milliseconds(minutes: i64) -> Option<i64> {
    minutes.checked_mul(60000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(milliseconds_to_minutes(0), 0);
        assert_eq!(milliseconds_to_minutes(120000), 2);
        assert_eq!(milliseconds_to_minutes(119999), 1);
        assert_eq!(milliseconds_to_minutes(-119999), -1);
        assert_eq!(milliseconds_to_minutes(59999), 0);
        assert_eq!(milliseconds_to_minutes(-59999), 0);
        assert_eq!(milliseconds_to_minutes(i64::MAX), i64::MAX / 60000);
        assert_eq!(milliseconds_to_minutes(i64::MIN), i64::MIN / 60000);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(minutes_to_milliseconds(0), Some(0));
        assert_eq!(minutes_to_milliseconds(2), Some(120000));
        assert_eq!(minutes_to_milliseconds(-2), Some(-120000));
        assert_eq!(minutes_to_milliseconds(i64::MAX), None);
        assert_eq!(minutes_to_milliseconds(i64::MIN), None);
        let max_safe = i64::MAX / 60000;
        let min_safe = i64::MIN / 60000;
        assert_eq!(
            minutes_to_milliseconds(max_safe),
            max_safe.checked_mul(60000)
        );
        assert_eq!(minutes_to_milliseconds(max_safe + 1), None);
        assert_eq!(
            minutes_to_milliseconds(min_safe),
            min_safe.checked_mul(60000)
        );
        assert_eq!(minutes_to_milliseconds(min_safe - 1), None);
    }
}
