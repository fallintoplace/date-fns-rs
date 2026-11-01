//! ISO 8601 basic local time strings without zone offsets.

use chrono::NaiveTime;

/// Parse HHMMSS followed by an optional decimal point and 1-9 digits.
///
/// Rejects whitespace, offsets, invalid clock values, and leap seconds.
pub fn parse_iso_basic_local_time(input: &str) -> Option<NaiveTime> {
    let bytes = input.as_bytes();
    if bytes.len() < 6 || !bytes[..6].iter().all(u8::is_ascii_digit) {
        return None;
    }
    if bytes.len() > 6
        && (bytes.len() < 8
            || bytes.len() > 16
            || bytes[6] != b'.'
            || !bytes[7..].iter().all(u8::is_ascii_digit))
    {
        return None;
    }
    let extended = format!("{}:{}:{}", &input[..2], &input[2..4], &input[4..]);
    crate::parse_iso_local_time(&extended)
}

/// Format HHMMSS and preserve nonzero nanosecond precision.
pub fn format_iso_basic_local_time(value: NaiveTime) -> Option<String> {
    let extended = crate::format_iso_local_time(value)?;
    Some(format!(
        "{}{}{}",
        &extended[..2],
        &extended[3..5],
        &extended[6..]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_fractional_local_times() {
        let value = NaiveTime::from_hms_nano_opt(23, 59, 58, 120_000_001).unwrap();
        assert_eq!(
            format_iso_basic_local_time(value).as_deref(),
            Some("235958.120000001")
        );
        assert_eq!(parse_iso_basic_local_time("235958.120000001"), Some(value));
        assert_eq!(
            parse_iso_basic_local_time("000000"),
            NaiveTime::from_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_noncanonical_text_and_invalid_clock_fields() {
        for input in [
            "240000",
            "235960",
            "2359",
            "23:59:59",
            "235959.",
            "235959.1234567890",
            "235959Z",
            "235959+0100",
            "235959 ",
            "235959.é",
        ] {
            assert_eq!(parse_iso_basic_local_time(input), None);
        }
    }
}
