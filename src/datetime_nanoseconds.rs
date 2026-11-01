//! Nanosecond-precision arithmetic with checked datetime and duration bounds.

use chrono::{NaiveDateTime, TimeDelta};

/// Add signed nanoseconds without overflowing Chrono's date-time range.
pub fn add_nanoseconds_datetime(
    value: NaiveDateTime,
    nanoseconds: i64,
) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::nanoseconds(nanoseconds))
}

/// Subtract signed nanoseconds without overflowing Chrono's date-time range.
pub fn sub_nanoseconds_datetime(
    value: NaiveDateTime,
    nanoseconds: i64,
) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::nanoseconds(nanoseconds))
}

/// Signed nanoseconds from right to left, or None if i64 cannot hold the span.
pub fn difference_in_nanoseconds_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> Option<i64> {
    left.signed_duration_since(right).num_nanoseconds()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn crosses_leap_day_with_exact_nanoseconds() {
        let last = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, 999_999_999)
            .unwrap();
        let first = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(add_nanoseconds_datetime(last, 1), Some(first));
        assert_eq!(sub_nanoseconds_datetime(first, 1), Some(last));
        assert_eq!(difference_in_nanoseconds_datetime(first, last), Some(1));
        assert_eq!(difference_in_nanoseconds_datetime(last, first), Some(-1));
        assert_eq!(add_nanoseconds_datetime(first, -1), Some(last));
    }

    #[test]
    fn checks_chrono_and_i64_limits() {
        assert_eq!(add_nanoseconds_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_nanoseconds_datetime(NaiveDateTime::MIN, 1), None);
        assert_eq!(
            difference_in_nanoseconds_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN),
            None
        );
        assert_eq!(
            difference_in_nanoseconds_datetime(NaiveDateTime::MIN, NaiveDateTime::MIN),
            Some(0)
        );
    }
}
