//! Checked whole-unit conversions between weeks and hours.

/// Convert whole weeks to hours, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn weeks_to_hours(weeks: i64) -> Option<i64> {
    weeks.checked_mul(168)
}

/// Convert hours to complete weeks, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn hours_to_weeks(hours: i64) -> i64 {
    hours / 168
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(hours_to_weeks(0), 0);
        assert_eq!(hours_to_weeks(168 * 2), 2);
        assert_eq!(hours_to_weeks(168 * 2 - 1), 1);
        assert_eq!(hours_to_weeks(-(168 * 2 - 1)), -1);
        assert_eq!(hours_to_weeks(168 - 1), 0);
        assert_eq!(hours_to_weeks(-(168 - 1)), 0);
        assert_eq!(hours_to_weeks(i64::MAX), i64::MAX / 168);
        assert_eq!(hours_to_weeks(i64::MIN), i64::MIN / 168);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(weeks_to_hours(0), Some(0));
        assert_eq!(weeks_to_hours(2), Some(168 * 2));
        assert_eq!(weeks_to_hours(-2), Some(-168 * 2));
        assert_eq!(weeks_to_hours(i64::MAX), None);
        assert_eq!(weeks_to_hours(i64::MIN), None);
        let largest = i64::MAX / 168;
        let smallest = i64::MIN / 168;
        assert_eq!(weeks_to_hours(largest), largest.checked_mul(168));
        assert_eq!(weeks_to_hours(smallest), smallest.checked_mul(168));
        assert_eq!(weeks_to_hours(largest + 1), None);
        assert_eq!(weeks_to_hours(smallest - 1), None);
    }
}
