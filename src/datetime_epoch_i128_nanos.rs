//! Unix nanoseconds covering Chrono's entire naive datetime range.

use chrono::{DateTime, NaiveDateTime, Utc};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

/// Convert a naive UTC datetime into signed i128 Unix nanoseconds.
///
/// Unlike i64 nanoseconds, i128 can represent all Chrono naive datetimes.
pub fn to_unix_timestamp_nanos_i128(value: NaiveDateTime) -> i128 {
    i128::from(value.and_utc().timestamp()) * NANOS_PER_SECOND
        + i128::from(value.and_utc().timestamp_subsec_nanos())
}

/// Convert signed i128 Unix nanoseconds to a UTC naive datetime.
///
/// Euclidean division preserves negative fractional timestamps. Values outside
/// Chrono's supported date range return None.
pub fn from_unix_timestamp_nanos_i128(value: i128) -> Option<NaiveDateTime> {
    let seconds = i64::try_from(value.div_euclid(NANOS_PER_SECOND)).ok()?;
    let nanoseconds = u32::try_from(value.rem_euclid(NANOS_PER_SECOND)).ok()?;
    DateTime::<Utc>::from_timestamp(seconds, nanoseconds).map(|value| value.naive_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_pre_epoch_nanos_and_chrono_extremes() {
        for nanos in [-1_i128, 0, 1, 1_000_000_001, -1_000_000_001] {
            let timestamp = from_unix_timestamp_nanos_i128(nanos).unwrap();
            assert_eq!(to_unix_timestamp_nanos_i128(timestamp), nanos);
        }
        for timestamp in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            assert_eq!(
                from_unix_timestamp_nanos_i128(to_unix_timestamp_nanos_i128(timestamp)),
                Some(timestamp)
            );
        }
    }

    #[test]
    fn rejects_unrepresentable_i128_boundaries() {
        assert_eq!(from_unix_timestamp_nanos_i128(i128::MIN), None);
        assert_eq!(from_unix_timestamp_nanos_i128(i128::MAX), None);
        assert_eq!(
            to_unix_timestamp_nanos_i128(from_unix_timestamp_nanos_i128(-1).unwrap()),
            -1
        );
    }
}
