//! Strict ISO 8601 basic local date-times without implied timezone.

use chrono::{NaiveDateTime, Timelike};

/// Parse YYYYMMDDTHHMMSS with an optional 1-9 digit fractional second.
/// Missing seconds, timezone offsets, whitespace and invalid dates are rejected.
pub fn parse_iso_basic_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (day, clock) = input.split_once('T')?;
    let date = crate::parse_iso_basic_date(day)?;
    let bytes = clock.as_bytes();
    if bytes.len() < 6 || !bytes[..6].iter().all(u8::is_ascii_digit) {
        return None;
    }
    let nanos = if bytes.len() == 6 {
        0
    } else {
        if bytes.len() < 8
            || bytes.len() > 16
            || bytes[6] != b'.'
            || !bytes[7..].iter().all(u8::is_ascii_digit)
        {
            return None;
        }
        let fraction = &clock[7..];
        let mut nanos = fraction.parse::<u32>().ok()?;
        for _ in fraction.len()..9 {
            nanos *= 10;
        }
        nanos
    };
    date.and_hms_nano_opt(
        clock[..2].parse().ok()?,
        clock[2..4].parse().ok()?,
        clock[4..6].parse().ok()?,
        nanos,
    )
}

/// Format a local datetime as YYYYMMDDTHHMMSS with trimmed fractional digits.
/// Returns None for years outside 0000-9999 or unsupported leap seconds.
pub fn format_iso_basic_local_datetime(value: NaiveDateTime) -> Option<String> {
    let day = crate::format_iso_basic_date(value.date())?;
    let nanos = value.nanosecond();
    if nanos >= 1_000_000_000 {
        return None;
    }
    let mut result = format!(
        "{day}T{:02}{:02}{:02}",
        value.hour(),
        value.minute(),
        value.second()
    );
    if nanos != 0 {
        let fraction = format!("{nanos:09}");
        result.push('.');
        result.push_str(fraction.trim_end_matches('0'));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_leap_days_and_fractional_seconds() {
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_450_000)
            .unwrap();
        assert_eq!(
            format_iso_basic_local_datetime(leap).as_deref(),
            Some("20240229T235958.12345")
        );
        assert_eq!(
            parse_iso_basic_local_datetime("20240229T235958.12345"),
            Some(leap)
        );
        let midnight = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            format_iso_basic_local_datetime(midnight).as_deref(),
            Some("20240301T000000")
        );
        assert_eq!(
            parse_iso_basic_local_datetime("20240301T000000"),
            Some(midnight)
        );
    }

    #[test]
    fn rejects_invalid_and_noncanonical_values() {
        for input in [
            "20230229T235958",
            "20240229T240000",
            "20240229T235960",
            "20240229T2359",
            "20240229T235958.",
            "20240229T235958.1234567890",
            "20240229T235958Z",
            "2024-02-29T235958",
            "20240229t235958",
            "20240229T235958 ",
        ] {
            assert_eq!(parse_iso_basic_local_datetime(input), None, "{input}");
        }
        assert_eq!(format_iso_basic_local_datetime(NaiveDateTime::MAX), None);
    }
}
