//! Checked Gregorian century shifts for naive datetimes.

use chrono::NaiveDateTime;

/// Add signed centuries, clamping February 29 in non-leap target years.
pub fn add_centuries_datetime(value: NaiveDateTime, centuries: i32) -> Option<NaiveDateTime> {
    Some(crate::add_centuries(value.date(), centuries)?.and_time(value.time()))
}

/// Subtract signed centuries while preserving nanosecond precision.
pub fn sub_centuries_datetime(value: NaiveDateTime, centuries: i32) -> Option<NaiveDateTime> {
    Some(crate::sub_centuries(value.date(), centuries)?.and_time(value.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(11, 12, 13, 123_456_789)
            .unwrap()
    }

    #[test]
    fn shifts_centuries_without_losing_clock_precision() {
        assert_eq!(add_centuries_datetime(at(2000, 2, 29), 1), Some(at(2100, 2, 28)));
        assert_eq!(sub_centuries_datetime(at(2000, 2, 29), 1), Some(at(1900, 2, 28)));
        assert_eq!(add_centuries_datetime(at(2024, 6, 15), -1), Some(at(1924, 6, 15)));
        assert_eq!(sub_centuries_datetime(at(2024, 6, 15), -1), Some(at(2124, 6, 15)));
    }

    #[test]
    fn handles_extremes_and_overflowing_offsets() {
        assert_eq!(add_centuries_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_centuries_datetime(NaiveDateTime::MIN, 1), None);
        assert_eq!(add_centuries_datetime(at(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_centuries_datetime(at(2024, 1, 1), i32::MIN), None);
        assert_eq!(
            sub_centuries_datetime(NaiveDateTime::MIN, 0),
            Some(NaiveDateTime::MIN)
        );
    }
}
