//! ISO 8601 minute-precision local times without implicit time zones.

use chrono::{NaiveTime, Timelike};

/// Parse exactly HH:MM as a local time with zero seconds.
pub fn parse_iso_hour_minute(input: &str) -> Option<NaiveTime> {
    let bytes = input.as_bytes();
    if bytes.len() != 5
        || bytes[2] != b':'
        || !bytes[..2].iter().all(u8::is_ascii_digit)
        || !bytes[3..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let hour = input[..2].parse::<u32>().ok()?;
    let minute = input[3..].parse::<u32>().ok()?;
    NaiveTime::from_hms_opt(hour, minute, 0)
}

/// Format as HH:MM, refusing to discard seconds or nanoseconds.
pub fn format_iso_hour_minute(value: NaiveTime) -> Option<String> {
    if value.second() != 0 || value.nanosecond() != 0 {
        return None;
    }
    Some(format!("{:02}:{:02}", value.hour(), value.minute()))
}

/// Parse exactly HHMM using the basic ISO time representation.
pub fn parse_iso_basic_hour_minute(input: &str) -> Option<NaiveTime> {
    let bytes = input.as_bytes();
    if bytes.len() != 4 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let hour = input[..2].parse::<u32>().ok()?;
    let minute = input[2..].parse::<u32>().ok()?;
    NaiveTime::from_hms_opt(hour, minute, 0)
}

/// Format a minute-precision local time as HHMM without precision loss.
pub fn format_iso_basic_hour_minute(value: NaiveTime) -> Option<String> {
    let extended = format_iso_hour_minute(value)?;
    Some(format!("{}{}", &extended[..2], &extended[3..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_extended_and_basic_minutes() {
        let value = NaiveTime::from_hms_opt(23, 59, 0).unwrap();
        assert_eq!(parse_iso_hour_minute("23:59"), Some(value));
        assert_eq!(parse_iso_basic_hour_minute("2359"), Some(value));
        assert_eq!(format_iso_hour_minute(value).as_deref(), Some("23:59"));
        assert_eq!(format_iso_basic_hour_minute(value).as_deref(), Some("2359"));
        assert_eq!(
            parse_iso_hour_minute("00:00"),
            NaiveTime::from_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_invalid_clocks_and_discarded_precision() {
        for input in ["24:00", "23:60", "9:05", "23:59:00", "23:59Z", "23:59 "] {
            assert_eq!(parse_iso_hour_minute(input), None);
        }
        for input in ["2400", "2360", "959", "23:59", "235900", "2359Z"] {
            assert_eq!(parse_iso_basic_hour_minute(input), None);
        }

        let seconds = NaiveTime::from_hms_opt(12, 30, 1).unwrap();
        let nanos = NaiveTime::from_hms_nano_opt(12, 30, 0, 1).unwrap();
        for value in [seconds, nanos] {
            assert_eq!(format_iso_hour_minute(value), None);
            assert_eq!(format_iso_basic_hour_minute(value), None);
        }
    }
}
