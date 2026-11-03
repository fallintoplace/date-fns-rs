//! Read basic components from a naive local date-time.

use chrono::{NaiveDateTime, Timelike};

/// Return the clock hour, in 0..=23.
pub fn get_hour_datetime(value: NaiveDateTime) -> u32 {
    value.hour()
}

/// Return the clock minute, in 0..=59.
pub fn get_minute_datetime(value: NaiveDateTime) -> u32 {
    value.minute()
}

/// Return the clock second, in 0..=59.
pub fn get_second_datetime(value: NaiveDateTime) -> u32 {
    value.second()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn reads_each_clock_component_without_losing_precision() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 58, 57, 123_456_789)
            .unwrap();
        assert_eq!(get_hour_datetime(value), 23);
        assert_eq!(get_minute_datetime(value), 58);
        assert_eq!(get_second_datetime(value), 57);
        assert_eq!(value.nanosecond(), 123_456_789);
    }

    #[test]
    fn works_at_supported_datetime_limits() {
        for value in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            assert!(get_hour_datetime(value) < 24);
            assert!(get_minute_datetime(value) < 60);
            assert!(get_second_datetime(value) < 60);
        }
    }
}
