//! Checked Unix nanosecond conversions with explicit UTC interpretation.

use chrono::{DateTime, NaiveDateTime, Utc};

/// Convert a naive UTC datetime to signed Unix nanoseconds.
///
/// Returns None outside the i64 nanosecond timestamp window.
pub fn to_unix_timestamp_nanos(value: NaiveDateTime) -> Option<i64> {
    value.and_utc().timestamp_nanos_opt()
}

/// Convert signed Unix nanoseconds to a naive UTC datetime.
///
/// Negative timestamps use Euclidean division to retain fractional seconds.
pub fn from_unix_timestamp_nanos(value: i64) -> Option<NaiveDateTime> {
    let seconds = value.div_euclid(1_000_000_000);
    let nanos = value.rem_euclid(1_000_000_000) as u32;
    DateTime::<Utc>::from_timestamp(seconds, nanos).map(|datetime| datetime.naive_utc())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_pre_epoch_subsecond_values() {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(to_unix_timestamp_nanos(epoch), Some(0));
        let previous = from_unix_timestamp_nanos(-1).unwrap();
        assert_eq!(previous.and_utc().timestamp(), -1);
        assert_eq!(previous.and_utc().timestamp_subsec_nanos(), 999_999_999);
        assert_eq!(to_unix_timestamp_nanos(previous), Some(-1));
    }

    #[test]
    fn accepts_i64_boundaries_and_rejects_out_of_range_dates() {
        for nanos in [i64::MIN, i64::MAX, -1, 0, 1] {
            assert_eq!(
                to_unix_timestamp_nanos(from_unix_timestamp_nanos(nanos).unwrap()),
                Some(nanos)
            );
        }
        assert_eq!(to_unix_timestamp_nanos(NaiveDateTime::MAX), None);
        assert_eq!(to_unix_timestamp_nanos(NaiveDateTime::MIN), None);
    }
}
