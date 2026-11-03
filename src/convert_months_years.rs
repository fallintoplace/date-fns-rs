//! Checked integer conversions between months and years.

/// Convert months to complete years, truncating partial units toward zero.
pub fn months_to_years(months: i64) -> i64 {
    months / 12
}

/// Convert years to months, returning None on integer overflow.
pub fn years_to_months(years: i64) -> Option<i64> {
    years.checked_mul(12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(months_to_years(0), 0);
        assert_eq!(months_to_years(24), 2);
        assert_eq!(months_to_years(23), 1);
        assert_eq!(months_to_years(-23), -1);
        assert_eq!(months_to_years(11), 0);
        assert_eq!(months_to_years(-11), 0);
        assert_eq!(months_to_years(i64::MAX), i64::MAX / 12);
        assert_eq!(months_to_years(i64::MIN), i64::MIN / 12);
    }

    #[test]
    fn converts_large_counts_without_wrapping() {
        assert_eq!(years_to_months(0), Some(0));
        assert_eq!(years_to_months(2), Some(24));
        assert_eq!(years_to_months(-2), Some(-24));
        assert_eq!(years_to_months(i64::MAX), None);
        assert_eq!(years_to_months(i64::MIN), None);
        let max_safe = i64::MAX / 12;
        let min_safe = i64::MIN / 12;
        assert_eq!(years_to_months(max_safe), max_safe.checked_mul(12));
        assert_eq!(years_to_months(max_safe + 1), None);
        assert_eq!(years_to_months(min_safe), min_safe.checked_mul(12));
        assert_eq!(years_to_months(min_safe - 1), None);
    }
}
