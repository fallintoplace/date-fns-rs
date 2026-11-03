//! Checked integer conversions between days and weeks.

/// Convert days to complete weeks, truncating partial units toward zero.
pub fn days_to_weeks(days: i64) -> i64 {
    days / 7
}

/// Convert weeks to days, returning None on integer overflow.
pub fn weeks_to_days(weeks: i64) -> Option<i64> {
    weeks.checked_mul(7)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(days_to_weeks(0), 0);
        assert_eq!(days_to_weeks(14), 2);
        assert_eq!(days_to_weeks(13), 1);
        assert_eq!(days_to_weeks(-13), -1);
        assert_eq!(days_to_weeks(6), 0);
        assert_eq!(days_to_weeks(-6), 0);
        assert_eq!(days_to_weeks(i64::MAX), i64::MAX / 7);
        assert_eq!(days_to_weeks(i64::MIN), i64::MIN / 7);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(weeks_to_days(0), Some(0));
        assert_eq!(weeks_to_days(2), Some(14));
        assert_eq!(weeks_to_days(-2), Some(-14));
        assert_eq!(weeks_to_days(i64::MAX), None);
        assert_eq!(weeks_to_days(i64::MIN), None);
        let max_safe = i64::MAX / 7;
        let min_safe = i64::MIN / 7;
        assert_eq!(weeks_to_days(max_safe), max_safe.checked_mul(7));
        assert_eq!(weeks_to_days(max_safe + 1), None);
        assert_eq!(weeks_to_days(min_safe), min_safe.checked_mul(7));
        assert_eq!(weeks_to_days(min_safe - 1), None);
    }
}
