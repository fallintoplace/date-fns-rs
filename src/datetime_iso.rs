//! Strict ISO 8601 local date-times without implicit UTC interpretation.

use chrono::{NaiveDateTime, Timelike};

/// Parse YYYY-MM-DDTHH:MM:SS with optional 1-9 digit fractional seconds.
/// No offset, trailing whitespace or leap second is accepted.
pub fn parse_iso_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (day, clock) = input.split_once('T')?;
    let date = crate::parse_iso_date(day)?;
    let b = clock.as_bytes();
    if b.len() < 8
        || b[2] != b':'
        || b[5] != b':'
        || !b[..2].iter().all(u8::is_ascii_digit)
        || !b[3..5].iter().all(u8::is_ascii_digit)
        || !b[6..8].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let hour = clock[..2].parse().ok()?;
    let minute = clock[3..5].parse().ok()?;
    let second = clock[6..8].parse().ok()?;
    let nano = if b.len() == 8 {
        0
    } else {
        if b.len() < 10 || b.len() > 18 || b[8] != b'.'
            || !b[9..].iter().all(u8::is_ascii_digit)
        {
            return None;
        }
        let fraction = &clock[9..];
        let mut value = fraction.parse::<u32>().ok()?;
        for _ in fraction.len()..9 {
            value *= 10;
        }
        value
    };
    date.and_hms_nano_opt(hour, minute, second, nano)
}

/// Format an ISO local date-time, preserving nonzero fractional nanoseconds.
pub fn format_iso_local_datetime(value: NaiveDateTime) -> Option<String> {
    let date = crate::format_iso_date(value.date())?;
    let nano = value.nanosecond();
    if nano >= 1_000_000_000 {
        return None;
    }
    let suffix = if nano == 0 {
        String::new()
    } else {
        let mut fraction = format!("{nano:09}");
        while fraction.ends_with('0') {
            fraction.pop();
        }
        format!(".{fraction}")
    };
    Some(format!("{date}T{:02}:{:02}:{:02}{suffix}", value.hour(), value.minute(), value.second()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_fractional_leap_day_date_times() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
            .and_hms_nano_opt(23, 59, 58, 120_000_001).unwrap();
        let text = "2024-02-29T23:59:58.120000001";
        assert_eq!(format_iso_local_datetime(value).as_deref(), Some(text));
        assert_eq!(parse_iso_local_datetime(text), Some(value));
        assert_eq!(parse_iso_local_datetime("2024-02-29T00:00:00"), Some(value.date().and_hms_opt(0, 0, 0).unwrap()));
        assert_eq!(parse_iso_local_datetime("2024-02-29T12:12:12.1").unwrap().nanosecond(), 100_000_000);
    }

    #[test]
    fn rejects_noncanonical_and_offset_inputs() {
        for input in ["2024-02-30T00:00:00", "2024-02-29T24:00:00", "2024-02-29T12:00:60",
            "2024-02-29 00:00:00", "2024-02-29T00:00:00Z", "2024-02-29T00:00:00+01:00",
            "2024-02-29T00:00:00.", "2024-02-29T00:00:00.1234567890", "2024-02-29T00:00:00 "] {
            assert_eq!(parse_iso_local_datetime(input), None);
        }
        assert_eq!(format_iso_local_datetime(NaiveDateTime::MAX), None);
    }
}