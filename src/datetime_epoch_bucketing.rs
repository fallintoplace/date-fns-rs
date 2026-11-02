//! Epoch-aligned nanosecond bucketing for UTC-assumed naive datetimes.

use chrono::{DateTime, NaiveDateTime, Utc};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

fn epoch_nanos(value: NaiveDateTime) -> Option<i128> {
    let utc = value.and_utc();
    let nanos = utc.timestamp_subsec_nanos();
    // Chrono can encode leap seconds with nanoseconds >= 1_000_000_000.
    if nanos >= 1_000_000_000 {
        return None;
    }
    Some(i128::from(utc.timestamp()) * NANOS_PER_SECOND + i128::from(nanos))
}

fn from_epoch_nanos(value: i128) -> Option<NaiveDateTime> {
    let seconds = i64::try_from(value.div_euclid(NANOS_PER_SECOND)).ok()?;
    let nanos = u32::try_from(value.rem_euclid(NANOS_PER_SECOND)).ok()?;
    DateTime::<Utc>::from_timestamp(seconds, nanos).map(|value| value.naive_utc())
}

fn step(value: NaiveDateTime, step_nanos: i64) -> Option<(i128, i128, i128)> {
    if step_nanos <= 0 {
        return None;
    }
    let nanos = epoch_nanos(value)?;
    let interval = i128::from(step_nanos);
    let remainder = nanos.rem_euclid(interval);
    Some((nanos, interval, remainder))
}

/// Round down to an epoch-aligned multiple of `step_nanos`.
///
/// Uses Euclidean division for timestamps before 1970. Returns None when the
/// step is nonpositive or the rounded result exceeds Chrono's supported range.
pub fn floor_datetime_to_epoch_step(
    value: NaiveDateTime,
    step_nanos: i64,
) -> Option<NaiveDateTime> {
    let (nanos, _, remainder) = step(value, step_nanos)?;
    from_epoch_nanos(nanos - remainder)
}

/// Round up to an epoch-aligned multiple, preserving already-aligned inputs.
pub fn ceil_datetime_to_epoch_step(
    value: NaiveDateTime,
    step_nanos: i64,
) -> Option<NaiveDateTime> {
    let (nanos, interval, remainder) = step(value, step_nanos)?;
    let rounded = if remainder == 0 {
        nanos
    } else {
        nanos + interval - remainder
    };
    from_epoch_nanos(rounded)
}

/// Round to the nearest epoch-aligned multiple; ties go toward later instants.
pub fn round_datetime_to_epoch_step(
    value: NaiveDateTime,
    step_nanos: i64,
) -> Option<NaiveDateTime> {
    let (nanos, interval, remainder) = step(value, step_nanos)?;
    let floor = nanos - remainder;
    let rounded = if remainder >= interval - remainder {
        floor + interval
    } else {
        floor
    };
    from_epoch_nanos(rounded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    fn epoch() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    #[test]
    fn handles_negative_timestamps_and_ties() {
        let point = epoch()
            .checked_sub_signed(TimeDelta::milliseconds(500))
            .unwrap();
        let second = 1_000_000_000;
        assert_eq!(
            floor_datetime_to_epoch_step(point, second),
            epoch().checked_sub_signed(TimeDelta::seconds(1))
        );
        assert_eq!(ceil_datetime_to_epoch_step(point, second), Some(epoch()));
        assert_eq!(round_datetime_to_epoch_step(point, second), Some(epoch()));
        assert_eq!(
            round_datetime_to_epoch_step(
                epoch().checked_add_signed(TimeDelta::milliseconds(500)).unwrap(),
                second
            ),
            epoch().checked_add_signed(TimeDelta::seconds(1))
        );
    }

    #[test]
    fn checks_subsecond_rounding_and_alignment() {
        let date = epoch()
            .checked_add_signed(TimeDelta::nanoseconds(2_250_000_001))
            .unwrap();
        assert_eq!(
            floor_datetime_to_epoch_step(date, 500_000_000),
            epoch().checked_add_signed(TimeDelta::seconds(2))
        );
        assert_eq!(
            ceil_datetime_to_epoch_step(date, 500_000_000),
            epoch().checked_add_signed(TimeDelta::milliseconds(2500))
        );
        assert_eq!(
            round_datetime_to_epoch_step(date, 500_000_000),
            epoch().checked_add_signed(TimeDelta::milliseconds(2500))
        );
        assert_eq!(floor_datetime_to_epoch_step(epoch(), 17), Some(epoch()));
        assert_eq!(ceil_datetime_to_epoch_step(epoch(), 17), Some(epoch()));
    }

    #[test]
    fn rejects_invalid_steps_and_out_of_range_buckets() {
        assert_eq!(floor_datetime_to_epoch_step(epoch(), 0), None);
        assert_eq!(ceil_datetime_to_epoch_step(epoch(), -1), None);
        assert_eq!(round_datetime_to_epoch_step(epoch(), 0), None);
        assert_eq!(ceil_datetime_to_epoch_step(NaiveDateTime::MAX, i64::MAX), None);
        assert_eq!(floor_datetime_to_epoch_step(NaiveDateTime::MIN, i64::MAX), None);
    }
}
