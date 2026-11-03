//! Checked whole-unit conversions between weeks and milliseconds.

/// Convert whole weeks to milliseconds, returning None on i64 overflow.
///
/// Uses fixed-length calendar days of 24 hours, not time-zone days.
pub fn weeks_to_milliseconds(weeks: i64) -> Option<i64> {
    weeks.checked_mul(604_800_000)
}

/// Convert milliseconds to complete weeks, truncating toward zero.
///
/// Negative partial units do not round away from zero.
pub fn milliseconds_to_weeks(milliseconds: i64) -> i64 {
    milliseconds / 604_800_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_partial_units_toward_zero() {
        assert_eq!(milliseconds_to_weeks(0), 0);
        assert_eq!(milliseconds_to_weeks(604_800_000 * 2), 2);
        assert_eq!(milliseconds_to_weeks(604_800_000 * 2 - 1), 1);
        assert_eq!(milliseconds_to_weeks(-(604_800_000 * 2 - 1)), -1);
        assert_eq!(milliseconds_to_weeks(604_800_000 - 1), 0);
        assert_eq!(milliseconds_to_weeks(-(604_800_000 - 1)), 0);
        assert_eq!(milliseconds_to_weeks(i64::MAX), i64::MAX / 604_800_000);
        assert_eq!(milliseconds_to_weeks(i64::MIN), i64::MIN / 604_800_000);
    }

    #[test]
    fn checks_overflow_on_exact_multiplication() {
        assert_eq!(weeks_to_milliseconds(0), Some(0));
        assert_eq!(weeks_to_milliseconds(2), Some(604_800_000 * 2));
        assert_eq!(weeks_to_milliseconds(-2), Some(-604_800_000 * 2));
        assert_eq!(weeks_to_milliseconds(i64::MAX), None);
        assert_eq!(weeks_to_milliseconds(i64::MIN), None);
        let largest = i64::MAX / 604_800_000;
        let smallest = i64::MIN / 604_800_000;
        assert_eq!(weeks_to_milliseconds(largest), largest.checked_mul(604_800_000));
        assert_eq!(weeks_to_milliseconds(smallest), smallest.checked_mul(604_800_000));
        assert_eq!(weeks_to_milliseconds(largest + 1), None);
        assert_eq!(weeks_to_milliseconds(smallest - 1), None);
    }
}
