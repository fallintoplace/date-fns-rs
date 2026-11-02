//! RFC 5545 floating local DATE-TIME values without implicit time zones.

use chrono::{NaiveDateTime, Timelike};

/// Parse a floating iCalendar DATE-TIME in exactly YYYYMMDDTHHMMSS form.
///
/// A floating time is not UTC. Offsets, Z suffixes and fractions are rejected.
pub fn parse_ical_datetime_floating(input: &str) -> Option<NaiveDateTime> {
    if input.len() != 15 {
        return None;
    }
    crate::parse_iso_basic_local_datetime(input)
}

/// Format a floating DATE-TIME without losing subsecond precision.
///
/// Only whole-second values with a four-digit year are representable.
pub fn format_ical_datetime_floating(value: NaiveDateTime) -> Option<String> {
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
    fn round_trips_leap_day_and_unambiguous_local_clocks() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(23, 59, 58)
            .unwrap();
        assert_eq!(parse_ical_datetime_floating("20240229T235958"), Some(value));
        assert_eq!(
            format_ical_datetime_floating(value).as_deref(),
            Some("20240229T235958")
        );
    }

    #[test]
    fn rejects_utc_offsets_fractions_and_invalid_fields() {
        for text in [
            "20240229T235958Z",
            "20240229T235958+0100",
            "20240229T235958.5",
            "20240230T235958",
            "20240229T240000",
            "20240229T235960",
            "2024-02-29T235958",
            "20240229T235958 ",
        ] {
            assert_eq!(parse_ical_datetime_floating(text), None, "{text}");
        }

        let fractional = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 1)
            .unwrap();
        assert_eq!(format_ical_datetime_floating(fractional), None);
        assert_eq!(format_ical_datetime_floating(NaiveDateTime::MAX), None);
    }
}
