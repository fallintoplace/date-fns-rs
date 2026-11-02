//! Checked clock component setters.

use chrono::{NaiveDateTime, Timelike};

/// Set the hour to 0 through 23 while retaining the date and other components.
pub fn set_hour_datetime(value: NaiveDateTime, hour: u32) -> Option<NaiveDateTime> {
    value.with_hour(hour)
}

/// Set the minute to 0 through 59.
pub fn set_minute_datetime(value: NaiveDateTime, minute: u32) -> Option<NaiveDateTime> {
    value.with_minute(minute)
}

/// Set the second to 0 through 59.
pub fn set_second_datetime(value: NaiveDateTime, second: u32) -> Option<NaiveDateTime> {
    value.with_second(second)
}

/// Read the current millisecond within the second.
pub fn get_millisecond_datetime(value: NaiveDateTime) -> u32 {
    value.nanosecond() / 1_000_000
}

/// Set the millisecond to 0 through 999, preserving finer precision.
pub fn set_millisecond_datetime(value: NaiveDateTime, millisecond: u32) -> Option<NaiveDateTime> {
    if millisecond > 999 {
        return None;
    }
    value.with_nanosecond(millisecond * 1_000_000 + value.nanosecond() % 1_000_000)
}


/// Return the whole microsecond component within the current second.
pub fn get_microsecond_datetime(value: NaiveDateTime) -> u32 {
    value.nanosecond() / 1_000
}

/// Set microseconds, preserving the remaining nanosecond component.
///
/// Values greater than 999_999 return None.
pub fn set_microsecond_datetime(value: NaiveDateTime, microsecond: u32) -> Option<NaiveDateTime> {
    if microsecond > 999_999 {
        return None;
    }
    value.with_nanosecond(microsecond * 1_000 + value.nanosecond() % 1_000)
}

/// Return the raw nanosecond component of the current second.
pub fn get_nanosecond_datetime(value: NaiveDateTime) -> u32 {
    value.nanosecond()
}

/// Set a nanosecond component from 0 through 999_999_999.
pub fn set_nanosecond_datetime(value: NaiveDateTime, nanosecond: u32) -> Option<NaiveDateTime> {
    if nanosecond >= 1_000_000_000 {
        return None;
    }
    value.with_nanosecond(nanosecond)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn subsecond_setters_preserve_unmodified_precision() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 34, 56, 987_654_321)
            .unwrap();
        assert_eq!(get_microsecond_datetime(start), 987_654);
        assert_eq!(get_nanosecond_datetime(start), 987_654_321);
        let changed = set_microsecond_datetime(start, 123_456).unwrap();
        assert_eq!(get_nanosecond_datetime(changed), 123_456_321);
        assert_eq!(
            get_nanosecond_datetime(set_nanosecond_datetime(start, 7).unwrap()),
            7
        );
        assert_eq!(changed.date(), start.date());
        assert_eq!(set_microsecond_datetime(start, 1_000_000), None);
        assert_eq!(set_nanosecond_datetime(start, 1_000_000_000), None);
    }

    #[test]
    fn modifies_only_the_requested_component() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 34, 56, 987_654_321)
            .unwrap();
        assert_eq!(set_hour_datetime(start, 5).unwrap().hour(), 5);
        assert_eq!(set_minute_datetime(start, 0).unwrap().second(), 56);
        assert_eq!(
            set_second_datetime(start, 1).unwrap().nanosecond(),
            987_654_321
        );
        let changed = set_millisecond_datetime(start, 123).unwrap();
        assert_eq!(changed.nanosecond(), 123_654_321);
        assert_eq!(get_millisecond_datetime(changed), 123);
        assert_eq!(changed.date(), start.date());
    }

    #[test]
    fn rejects_invalid_clock_fields() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(set_hour_datetime(start, 24), None);
        assert_eq!(set_minute_datetime(start, 60), None);
        assert_eq!(set_second_datetime(start, 60), None);
        assert_eq!(set_millisecond_datetime(start, 1_000), None);
    }
}
