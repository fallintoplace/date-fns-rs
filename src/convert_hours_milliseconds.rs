//! Checked integer conversions between milliseconds and hours.

/// Convert milliseconds to complete hours, truncating partial units toward zero.
pub fn milliseconds_to_hours(milliseconds: i64) -> i64 {
    milliseconds / 3600000
}

/// Convert hours to milliseconds, returning None on integer overflow.
pub fn hours_to_milliseconds(hours: i64) -> Option<i64> {
    hours.checked_mul(3600000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(milliseconds_to_hours(0), 0);
        assert_eq!(milliseconds_to_hours(7200000), 2);
        assert_eq!(milliseconds_to_hours(7199999), 1);
        assert_eq!(milliseconds_to_hours(-7199999), -1);
        assert_eq!(milliseconds_to_hours(3599999), 0);
        assert_eq!(milliseconds_to_hours(-3599999), 0);
        assert_eq!(milliseconds_to_hours(i64::MAX), i64::MAX / 3600000);
        assert_eq!(milliseconds_to_hours(i64::MIN), i64::MIN / 3600000);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(hours_to_milliseconds(0), Some(0));
        assert_eq!(hours_to_milliseconds(2), Some(7200000));
        assert_eq!(hours_to_milliseconds(-2), Some(-7200000));
        assert_eq!(hours_to_milliseconds(i64::MAX), None);
        assert_eq!(hours_to_milliseconds(i64::MIN), None);
        let max_safe = i64::MAX / 3600000;
        let min_safe = i64::MIN / 3600000;
        assert_eq!(hours_to_milliseconds(max_safe), max_safe.checked_mul(3600000));
        assert_eq!(hours_to_milliseconds(max_safe + 1), None);
        assert_eq!(hours_to_milliseconds(min_safe), min_safe.checked_mul(3600000));
        assert_eq!(hours_to_milliseconds(min_safe - 1), None);
    }
}
