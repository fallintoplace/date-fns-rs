//! Checked time-based arithmetic on naive date-times.

use chrono::{NaiveDateTime, TimeDelta};

/// Shift a naive date-time by whole hours without wrapping at date limits.
pub fn add_hours_datetime(value: NaiveDateTime, hours: i64) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::try_seconds(hours.checked_mul(3_600)?)?)
}

/// Subtract whole hours, accepting negative offsets.
pub fn sub_hours_datetime(value: NaiveDateTime, hours: i64) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::try_seconds(hours.checked_mul(3_600)?)?)
}

/// Shift a naive date-time by whole minutes.
pub fn add_minutes_datetime(value: NaiveDateTime, minutes: i64) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::try_seconds(minutes.checked_mul(60)?)?)
}

/// Subtract whole minutes from a naive date-time.
pub fn sub_minutes_datetime(value: NaiveDateTime, minutes: i64) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::try_seconds(minutes.checked_mul(60)?)?)
}

/// Shift a naive date-time by whole seconds.
pub fn add_seconds_datetime(value: NaiveDateTime, seconds: i64) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::try_seconds(seconds)?)
}

/// Subtract whole seconds.
pub fn sub_seconds_datetime(value: NaiveDateTime, seconds: i64) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::try_seconds(seconds)?)
}

/// Shift a naive date-time by milliseconds.
pub fn add_milliseconds_datetime(value: NaiveDateTime, millis: i64) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::try_milliseconds(millis)?)
}

/// Subtract milliseconds without overflowing intermediate negations.
pub fn sub_milliseconds_datetime(value: NaiveDateTime, millis: i64) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::try_milliseconds(millis)?)
}

/// Shift a naive date-time by microseconds.
pub fn add_microseconds_datetime(value: NaiveDateTime, micros: i64) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::microseconds(micros))
}

/// Subtract microseconds.
pub fn sub_microseconds_datetime(value: NaiveDateTime, micros: i64) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::microseconds(micros))
}

/// Shift a naive date-time by signed nanoseconds without wrapping.
pub fn add_nanoseconds_datetime(value: NaiveDateTime, nanos: i64) -> Option<NaiveDateTime> {
    value.checked_add_signed(TimeDelta::nanoseconds(nanos))
}

/// Subtract signed nanoseconds without wrapping at date-time limits.
pub fn sub_nanoseconds_datetime(value: NaiveDateTime, nanos: i64) -> Option<NaiveDateTime> {
    value.checked_sub_signed(TimeDelta::nanoseconds(nanos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, min, s)
            .unwrap()
    }

    #[test]
    fn crosses_midnight_and_leap_days() {
        let before = dt(2024, 2, 28, 23, 30, 0);
        assert_eq!(
            add_hours_datetime(before, 1),
            Some(dt(2024, 2, 29, 0, 30, 0))
        );
        assert_eq!(
            sub_minutes_datetime(before, -60),
            add_hours_datetime(before, 1)
        );
        assert_eq!(
            add_seconds_datetime(before, 60),
            Some(dt(2024, 2, 28, 23, 31, 0))
        );
    }

    #[test]
    fn retains_subsecond_precision() {
        let start = dt(2024, 1, 1, 0, 0, 0);
        let shifted = add_milliseconds_datetime(start, 1).unwrap();
        assert_eq!(
            add_microseconds_datetime(shifted, 1)
                .unwrap()
                .and_utc()
                .timestamp_subsec_nanos(),
            1_001_000
        );
        assert_eq!(sub_milliseconds_datetime(shifted, 1), Some(start));
        assert_eq!(
            sub_microseconds_datetime(start, -1),
            add_microseconds_datetime(start, 1)
        );
    }

    #[test]
    fn huge_durations_return_none_without_panicking() {
        let value = dt(2024, 1, 1, 0, 0, 0);
        for amount in [i64::MIN, i64::MAX] {
            assert_eq!(add_seconds_datetime(value, amount), None);
            assert_eq!(sub_seconds_datetime(value, amount), None);
            assert_eq!(add_milliseconds_datetime(value, amount), None);
            assert_eq!(sub_milliseconds_datetime(value, amount), None);
        }
        let large_hours = i64::MAX / 3_600;
        assert_eq!(add_hours_datetime(value, large_hours), None);
        assert_eq!(sub_hours_datetime(value, large_hours), None);
        let large_minutes = i64::MAX / 60;
        assert_eq!(add_minutes_datetime(value, large_minutes), None);
        assert_eq!(sub_minutes_datetime(value, large_minutes), None);
    }

    #[test]
    fn checks_supported_limits() {
        assert_eq!(add_seconds_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_seconds_datetime(NaiveDateTime::MIN, 1), None);
        assert_eq!(add_hours_datetime(dt(2024, 1, 1, 0, 0, 0), i64::MAX), None);
        assert_eq!(
            sub_minutes_datetime(dt(2024, 1, 1, 0, 0, 0), i64::MIN),
            None
        );
    }
    #[test]
    fn nanosecond_arithmetic_carries_across_midnight() {
        let last = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, 999_999_999)
            .unwrap();
        let next = dt(2024, 3, 1, 0, 0, 0);
        assert_eq!(add_nanoseconds_datetime(last, 1), Some(next));
        assert_eq!(sub_nanoseconds_datetime(next, 1), Some(last));
        assert_eq!(add_nanoseconds_datetime(next, -1), Some(last));
        assert_eq!(sub_nanoseconds_datetime(last, -1), Some(next));
        assert_eq!(add_nanoseconds_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_nanoseconds_datetime(NaiveDateTime::MIN, 1), None);
    }
}
