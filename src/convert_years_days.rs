//! Fixed-length year-to-day conversion for durations.

/// Convert `years` into 365-day years, returning None on integer overflow.
///
/// This matches date-fns yearsToDays, not elapsed calendar days across leap years.
pub fn years_to_days(years: i64) -> Option<i64> {
    years.checked_mul(365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_positive_negative_and_zero_durations() {
        assert_eq!(years_to_days(0), Some(0));
        assert_eq!(years_to_days(1), Some(365));
        assert_eq!(years_to_days(2), Some(730));
        assert_eq!(years_to_days(-2), Some(-730));
    }

    #[test]
    fn does_not_wrap_at_integer_limits() {
        assert_eq!(years_to_days(i64::MAX), None);
        assert_eq!(years_to_days(i64::MIN), None);
        let max = i64::MAX / 365;
        let min = i64::MIN / 365;
        assert_eq!(years_to_days(max), max.checked_mul(365));
        assert_eq!(years_to_days(min), min.checked_mul(365));
        assert_eq!(years_to_days(max + 1), None);
        assert_eq!(years_to_days(min - 1), None);
    }
}
