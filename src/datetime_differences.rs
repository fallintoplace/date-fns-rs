//! Signed elapsed-time differences between naive date-times.

use chrono::NaiveDateTime;

/// Whole elapsed hours from right to left, truncated toward zero.
pub fn difference_in_hours_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    left.signed_duration_since(right).num_hours()
}

/// Whole elapsed minutes from right to left, truncated toward zero.
pub fn difference_in_minutes_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    left.signed_duration_since(right).num_minutes()
}

/// Whole elapsed seconds from right to left, truncated toward zero.
pub fn difference_in_seconds_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    left.signed_duration_since(right).num_seconds()
}

/// Signed difference in milliseconds.
pub fn difference_in_milliseconds_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    left.signed_duration_since(right).num_milliseconds()
}

/// Signed difference in microseconds, or None for an i64 overflow.
pub fn difference_in_microseconds_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> Option<i64> {
    left.signed_duration_since(right).num_microseconds()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn respects_elapsed_units_and_sign() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap().and_hms_opt(23, 30, 0).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap().and_hms_milli_opt(0, 30, 1, 500).unwrap();
        assert_eq!(difference_in_hours_datetime(end, start), 1);
        assert_eq!(difference_in_minutes_datetime(end, start), 60);
        assert_eq!(difference_in_seconds_datetime(end, start), 3_601);
        assert_eq!(difference_in_milliseconds_datetime(end, start), 3_601_500);
        assert_eq!(difference_in_milliseconds_datetime(start, end), -3_601_500);
        assert_eq!(difference_in_microseconds_datetime(end, start), Some(3_601_500_000));
        assert_eq!(difference_in_hours_datetime(start, end), -1);
    }

    #[test]
    fn large_microsecond_differences_are_checked() {
        assert_eq!(difference_in_microseconds_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN), None);
        assert_eq!(difference_in_milliseconds_datetime(NaiveDateTime::MAX, NaiveDateTime::MAX), 0);
    }
}