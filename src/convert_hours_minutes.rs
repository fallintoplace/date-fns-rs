//! Checked integer conversions between minutes and hours.

/// Convert minutes to complete hours, truncating partial units toward zero.
pub fn minutes_to_hours(minutes: i64) -> i64 {
    minutes / 60
}

/// Convert hours to minutes, returning None on integer overflow.
pub fn hours_to_minutes(hours: i64) -> Option<i64> {
    hours.checked_mul(60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(minutes_to_hours(0), 0);
        assert_eq!(minutes_to_hours(120), 2);
        assert_eq!(minutes_to_hours(119), 1);
        assert_eq!(minutes_to_hours(-119), -1);
        assert_eq!(minutes_to_hours(59), 0);
        assert_eq!(minutes_to_hours(-59), 0);
        assert_eq!(minutes_to_hours(i64::MAX), i64::MAX / 60);
        assert_eq!(minutes_to_hours(i64::MIN), i64::MIN / 60);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(hours_to_minutes(0), Some(0));
        assert_eq!(hours_to_minutes(2), Some(120));
        assert_eq!(hours_to_minutes(-2), Some(-120));
        assert_eq!(hours_to_minutes(i64::MAX), None);
        assert_eq!(hours_to_minutes(i64::MIN), None);
        let max_safe = i64::MAX / 60;
        let min_safe = i64::MIN / 60;
        assert_eq!(hours_to_minutes(max_safe), max_safe.checked_mul(60));
        assert_eq!(hours_to_minutes(max_safe + 1), None);
        assert_eq!(hours_to_minutes(min_safe), min_safe.checked_mul(60));
        assert_eq!(hours_to_minutes(min_safe - 1), None);
    }
}
