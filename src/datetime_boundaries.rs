//! Boundaries for naive date-time periods, with nanosecond precision.

use chrono::{NaiveDateTime, Timelike};

/// Midnight at the start of the date.
pub fn start_of_day_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_opt(0, 0, 0).unwrap()
}

/// Last representable nanosecond of the date.
pub fn end_of_day_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_nano_opt(23, 59, 59, 999_999_999).unwrap()
}

/// Start of the current hour.
pub fn start_of_hour_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_opt(value.hour(), 0, 0).unwrap()
}

/// Last representable nanosecond of the current hour.
pub fn end_of_hour_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_nano_opt(value.hour(), 59, 59, 999_999_999).unwrap()
}

/// Start of the current minute.
pub fn start_of_minute_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_opt(value.hour(), value.minute(), 0).unwrap()
}

/// Last representable nanosecond of the current minute.
pub fn end_of_minute_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_nano_opt(value.hour(), value.minute(), 59, 999_999_999).unwrap()
}

/// Start of the current second.
pub fn start_of_second_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_opt(value.hour(), value.minute(), value.second()).unwrap()
}

/// Last representable nanosecond of the current second.
pub fn end_of_second_datetime(value: NaiveDateTime) -> NaiveDateTime {
    value.date().and_hms_nano_opt(value.hour(), value.minute(), value.second(), 999_999_999).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn boundaries_preserve_date_and_the_selected_unit() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let value = date.and_hms_nano_opt(16, 35, 27, 123_456_789).unwrap();
        assert_eq!(start_of_day_datetime(value), date.and_hms_opt(0, 0, 0).unwrap());
        assert_eq!(end_of_day_datetime(value), date.and_hms_nano_opt(23, 59, 59, 999_999_999).unwrap());
        assert_eq!(start_of_hour_datetime(value), date.and_hms_opt(16, 0, 0).unwrap());
        assert_eq!(end_of_hour_datetime(value), date.and_hms_nano_opt(16, 59, 59, 999_999_999).unwrap());
        assert_eq!(start_of_minute_datetime(value), date.and_hms_opt(16, 35, 0).unwrap());
        assert_eq!(end_of_minute_datetime(value), date.and_hms_nano_opt(16, 35, 59, 999_999_999).unwrap());
        assert_eq!(start_of_second_datetime(value), date.and_hms_opt(16, 35, 27).unwrap());
        assert_eq!(end_of_second_datetime(value), date.and_hms_nano_opt(16, 35, 27, 999_999_999).unwrap());
    }

    #[test]
    fn supports_maximum_datetime() {
        assert_eq!(end_of_day_datetime(NaiveDateTime::MAX), NaiveDateTime::MAX);
    }
}