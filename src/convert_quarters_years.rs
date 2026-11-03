//! Checked integer conversions between quarters and years.

/// Convert quarters to complete years, truncating partial units toward zero.
pub fn quarters_to_years(quarters: i64) -> i64 {
    quarters / 4
}

/// Convert years to quarters, returning None on integer overflow.
pub fn years_to_quarters(years: i64) -> Option<i64> {
    years.checked_mul(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(quarters_to_years(0), 0);
        assert_eq!(quarters_to_years(8), 2);
        assert_eq!(quarters_to_years(7), 1);
        assert_eq!(quarters_to_years(-7), -1);
        assert_eq!(quarters_to_years(3), 0);
        assert_eq!(quarters_to_years(-3), 0);
        assert_eq!(quarters_to_years(i64::MAX), i64::MAX / 4);
        assert_eq!(quarters_to_years(i64::MIN), i64::MIN / 4);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(years_to_quarters(0), Some(0));
        assert_eq!(years_to_quarters(2), Some(8));
        assert_eq!(years_to_quarters(-2), Some(-8));
        assert_eq!(years_to_quarters(i64::MAX), None);
        assert_eq!(years_to_quarters(i64::MIN), None);
        let max_safe = i64::MAX / 4;
        let min_safe = i64::MIN / 4;
        assert_eq!(years_to_quarters(max_safe), max_safe.checked_mul(4));
        assert_eq!(years_to_quarters(max_safe + 1), None);
        assert_eq!(years_to_quarters(min_safe), min_safe.checked_mul(4));
        assert_eq!(years_to_quarters(min_safe - 1), None);
    }
}
