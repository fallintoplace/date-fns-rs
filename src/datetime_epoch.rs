//! Explicit UTC Unix timestamps for naive date-times.

use chrono::{DateTime, NaiveDateTime, Utc};

/// Count whole Unix seconds, assuming the naive input represents UTC.
pub fn to_unix_timestamp_seconds(value: NaiveDateTime) -> i64 {
    value.and_utc().timestamp()
}

/// Count whole Unix milliseconds, assuming the naive input represents UTC.
pub fn to_unix_timestamp_millis(value: NaiveDateTime) -> i64 {
    value.and_utc().timestamp_millis()
}

/// Convert whole Unix seconds to a UTC naive date-time.
pub fn from_unix_timestamp_seconds(value: i64) -> Option<NaiveDateTime> {
    DateTime::<Utc>::from_timestamp(value, 0).map(|dt| dt.naive_utc())
}

/// Convert whole Unix milliseconds to a UTC naive date-time.
pub fn from_unix_timestamp_millis(value: i64) -> Option<NaiveDateTime> {
    DateTime::<Utc>::from_timestamp_millis(value).map(|dt| dt.naive_utc())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn supports_pre_epoch_and_millisecond_values() {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(to_unix_timestamp_seconds(epoch), 0);
        assert_eq!(to_unix_timestamp_millis(epoch), 0);
        assert_eq!(
            from_unix_timestamp_seconds(-1),
            epoch.checked_sub_signed(chrono::TimeDelta::seconds(1))
        );
        let previous = from_unix_timestamp_millis(-1).unwrap();
        assert_eq!(previous.and_utc().timestamp_subsec_millis(), 999);
        assert_eq!(to_unix_timestamp_millis(previous), -1);
    }

    #[test]
    fn round_trips_utc_values_and_rejects_overflow() {
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_milli_opt(12, 34, 56, 789)
            .unwrap();
        assert_eq!(
            from_unix_timestamp_millis(to_unix_timestamp_millis(leap)),
            Some(leap)
        );
        assert_eq!(from_unix_timestamp_seconds(i64::MAX), None);
        assert_eq!(from_unix_timestamp_millis(i64::MIN), None);
    }
}
