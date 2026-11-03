//! Checked integer conversions between months and quarters.

/// Convert months to complete quarters, truncating partial units toward zero.
pub fn months_to_quarters(months: i64) -> i64 {
    months / 3
}

/// Convert quarters to months, returning None on integer overflow.
pub fn quarters_to_months(quarters: i64) -> Option<i64> {
    quarters.checked_mul(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(months_to_quarters(0), 0);
        assert_eq!(months_to_quarters(6), 2);
        assert_eq!(months_to_quarters(5), 1);
        assert_eq!(months_to_quarters(-5), -1);
        assert_eq!(months_to_quarters(2), 0);
        assert_eq!(months_to_quarters(-2), 0);
        assert_eq!(months_to_quarters(i64::MAX), i64::MAX / 3);
        assert_eq!(months_to_quarters(i64::MIN), i64::MIN / 3);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(quarters_to_months(0), Some(0));
        assert_eq!(quarters_to_months(2), Some(6));
        assert_eq!(quarters_to_months(-2), Some(-6));
        assert_eq!(quarters_to_months(i64::MAX), None);
        assert_eq!(quarters_to_months(i64::MIN), None);
        let max_safe = i64::MAX / 3;
        let min_safe = i64::MIN / 3;
        assert_eq!(quarters_to_months(max_safe), max_safe.checked_mul(3));
        assert_eq!(quarters_to_months(max_safe + 1), None);
        assert_eq!(quarters_to_months(min_safe), min_safe.checked_mul(3));
        assert_eq!(quarters_to_months(min_safe - 1), None);
    }
}
