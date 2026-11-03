//! Checked whole-unit conversions between weeks and seconds.

/// Convert whole weeks to seconds, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn weeks_to_seconds(weeks: i64) -> Option<i64> {
    weeks.checked_mul(604_800)
}

/// Convert seconds to complete weeks, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn seconds_to_weeks(seconds: i64) -> i64 {
    seconds / 604_800
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(seconds_to_weeks(0), 0);
        assert_eq!(seconds_to_weeks(604_800 * 2), 2);
        assert_eq!(seconds_to_weeks(604_800 * 2 - 1), 1);
        assert_eq!(seconds_to_weeks(-(604_800 * 2 - 1)), -1);
        assert_eq!(seconds_to_weeks(604_800 - 1), 0);
        assert_eq!(seconds_to_weeks(-(604_800 - 1)), 0);
        assert_eq!(seconds_to_weeks(i64::MAX), i64::MAX / 604_800);
        assert_eq!(seconds_to_weeks(i64::MIN), i64::MIN / 604_800);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(weeks_to_seconds(0), Some(0));
        assert_eq!(weeks_to_seconds(2), Some(604_800 * 2));
        assert_eq!(weeks_to_seconds(-2), Some(-604_800 * 2));
        assert_eq!(weeks_to_seconds(i64::MAX), None);
        assert_eq!(weeks_to_seconds(i64::MIN), None);
        let largest = i64::MAX / 604_800;
        let smallest = i64::MIN / 604_800;
        assert_eq!(weeks_to_seconds(largest), largest.checked_mul(604_800));
        assert_eq!(weeks_to_seconds(smallest), smallest.checked_mul(604_800));
        assert_eq!(weeks_to_seconds(largest + 1), None);
        assert_eq!(weeks_to_seconds(smallest - 1), None);
    }
}
