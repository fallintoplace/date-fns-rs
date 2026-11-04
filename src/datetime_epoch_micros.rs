//! Explicit UTC Unix microsecond conversions for naive date-times.

use chrono::{DateTime, NaiveDateTime, Utc};

/// Count Unix microseconds, interpreting the naive input as UTC.
///
/// This is lossless for microsecond-aligned values. Sub-microsecond precision
/// is truncated toward the earlier integral Unix microsecond.
pub fn to_unix_timestamp_micros(value: NaiveDateTime) -> i64 {
    value.and_utc().timestamp_micros()
}

/// Convert Unix microseconds to a UTC naive date-time.
///
/// Returns None for values outside Chrono's supported date-time range.
pub fn from_unix_timestamp_micros(value: i64) -> Option<NaiveDateTime> {
    DateTime::<Utc>::from_timestamp_micros(value).map(|value| value.naive_utc())
}

/// Convert to Unix microseconds only when no nanoseconds are lost.
///
/// Returns None for timestamps that are not microsecond-aligned.
pub fn try_to_unix_timestamp_micros(value: NaiveDateTime) -> Option<i64> {
    (value.and_utc().timestamp_subsec_nanos() % 1_000 == 0).then(|| to_unix_timestamp_micros(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn preserves_subsecond_precision_and_pre_epoch_times() {
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_micro_opt(12, 34, 56, 987_654)
            .unwrap();
        assert_eq!(
            from_unix_timestamp_micros(to_unix_timestamp_micros(leap)),
            Some(leap)
        );

        let before_epoch = from_unix_timestamp_micros(-1).unwrap();
        assert_eq!(to_unix_timestamp_micros(before_epoch), -1);
        assert_eq!(before_epoch.and_utc().timestamp_subsec_micros(), 999_999);
        assert_eq!(
            from_unix_timestamp_micros(0),
            Some(
                NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn rejects_microsecond_values_outside_chrono_range() {
        assert_eq!(from_unix_timestamp_micros(i64::MAX), None);
        assert_eq!(from_unix_timestamp_micros(i64::MIN), None);
    }
    #[test]
    fn checked_microsecond_conversion_preserves_precision() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_micro_opt(12, 34, 56, 123_456)
            .unwrap();
        assert_eq!(
            try_to_unix_timestamp_micros(value),
            Some(to_unix_timestamp_micros(value))
        );
        let finer = value + chrono::TimeDelta::nanoseconds(1);
        assert_eq!(try_to_unix_timestamp_micros(finer), None);
        let before = from_unix_timestamp_micros(-1).unwrap();
        assert_eq!(try_to_unix_timestamp_micros(before), Some(-1));
    }
}
