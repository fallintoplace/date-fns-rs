//! Floating DATE-TIME values in RFC 5545 iCalendar format.

use chrono::{NaiveDateTime, Timelike};

/// Parse a floating local iCalendar DATE-TIME as YYYYMMDDTHHMMSS.
///
/// The input has no time-zone offset or UTC suffix. Interpreting the local
/// wall time in a real time zone remains the responsibility of the caller.
pub fn parse_ical_floating_datetime(input: &str) -> Option<NaiveDateTime> {
    crate::parse_iso_basic_local_datetime(input)
}

/// Format a floating local DATE-TIME without a UTC suffix.
///
/// Subsecond timestamps cannot be represented and return None instead of
/// silently losing precision.
pub fn format_ical_floating_datetime(value: NaiveDateTime) -> Option<String> {
    if value.nanosecond() != 0 {
        return None;
    }
    crate::format_iso_basic_local_datetime(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_leap_day_without_inferring_timezone() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(23, 59, 58)
            .unwrap();
        assert_eq!(parse_ical_floating_datetime("20240229T235958"), Some(value));
        assert_eq!(
            format_ical_floating_datetime(value).as_deref(),
            Some("20240229T235958")
        );
    }

    #[test]
    fn rejects_utc_offsets_invalid_dates_and_fractional_precision() {
        for invalid in [
            "20240229T235958Z",
            "20240229T235958+0100",
            "20240230T235958",
            "20240229T240000",
            "2024-02-29T23:59:58",
            "20240229T235958.0",
        ] {
            assert_eq!(parse_ical_floating_datetime(invalid), None);
        }
        let fractional = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 1)
            .unwrap();
        assert_eq!(format_ical_floating_datetime(fractional), None);
    }
}
