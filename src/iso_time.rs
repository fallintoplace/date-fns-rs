//! Strict ISO local time-only strings without an implicit date or time zone.

use chrono::{NaiveTime, Timelike};

/// Parse an HH:MM:SS local time, optionally followed by 1-9 fractional digits.
///
/// Offset markers, whitespace, leap seconds and 24:00:00 are rejected.
pub fn parse_iso_local_time(input: &str) -> Option<NaiveTime> {
    let (clock, fractional) = match input.split_once('.') {
        Some((clock, fractional)) => (clock, Some(fractional)),
        None => (input, None),
    };
    let bytes = clock.as_bytes();
    if bytes.len() != 8
        || bytes[2] != b':'
        || bytes[5] != b':'
        || !bytes[..2].iter().all(u8::is_ascii_digit)
        || !bytes[3..5].iter().all(u8::is_ascii_digit)
        || !bytes[6..8].iter().all(u8::is_ascii_digit)
    {
        return None;
    }

    let nanos = match fractional {
        Some(digits)
            if !digits.is_empty()
                && digits.len() <= 9
                && digits.as_bytes().iter().all(u8::is_ascii_digit) =>
        {
            let mut value = digits.parse::<u32>().ok()?;
            for _ in digits.len()..9 {
                value *= 10;
            }
            value
        }
        Some(_) => return None,
        None => 0,
    };

    NaiveTime::from_hms_nano_opt(
        clock[..2].parse().ok()?,
        clock[3..5].parse().ok()?,
        clock[6..8].parse().ok()?,
        nanos,
    )
}

/// Format a time as HH:MM:SS, with nonzero nanoseconds preserved.
///
/// Chrono leap-second representations are rejected because the parser rejects them.
pub fn format_iso_local_time(value: NaiveTime) -> Option<String> {
    let nanos = value.nanosecond();
    if nanos >= 1_000_000_000 {
        return None;
    }
    let suffix = if nanos == 0 {
        String::new()
    } else {
        let mut digits = format!("{nanos:09}");
        while digits.ends_with('0') {
            digits.pop();
        }
        format!(".{digits}")
    };
    Some(format!(
        "{:02}:{:02}:{:02}{suffix}",
        value.hour(),
        value.minute(),
        value.second()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_seconds_and_nanoseconds() {
        let precise = NaiveTime::from_hms_nano_opt(7, 8, 9, 123_400_001).unwrap();
        assert_eq!(
            format_iso_local_time(precise).as_deref(),
            Some("07:08:09.123400001")
        );
        assert_eq!(parse_iso_local_time("07:08:09.123400001"), Some(precise));
        assert_eq!(
            parse_iso_local_time("07:08:09.12"),
            NaiveTime::from_hms_milli_opt(7, 8, 9, 120)
        );
        assert_eq!(
            format_iso_local_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()).as_deref(),
            Some("00:00:00")
        );
    }

    #[test]
    fn rejects_offsets_leap_seconds_and_noncanonical_text() {
        for text in [
            "24:00:00",
            "23:59:60",
            "7:08:09",
            "07:8:09",
            "07:08:09.",
            "07:08:09.1234567890",
            "07:08:09Z",
            "07:08:09+01:00",
            "07:08:09 ",
            "07:08:09.x",
        ] {
            assert_eq!(parse_iso_local_time(text), None, "{text}");
        }
    }
}
