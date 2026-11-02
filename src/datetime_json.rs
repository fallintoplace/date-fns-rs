//! JSON-API timestamp parsing with explicit UTC semantics.

use chrono::{DateTime, NaiveDateTime};

/// Parse a JSON datetime and return UTC as a naive datetime.
///
/// RFC 3339 offsets, compact +HHMM offsets, and Z are converted to UTC.
/// Timestamp strings without offsets, using T or a space, are interpreted
/// as UTC (never as the machine's local timezone).
pub fn parse_json_datetime_utc(input: &str) -> Option<NaiveDateTime> {
    if let Ok(value) = DateTime::parse_from_rfc3339(input) {
        return Some(value.naive_utc());
    }

    let bytes = input.as_bytes();
    if bytes.len() >= 5 {
        let len = bytes.len();
        if matches!(bytes[len - 5], b'+' | b'-') && bytes[len - 4..].iter().all(u8::is_ascii_digit)
        {
            let mut fixed = input[..len - 2].to_owned();
            fixed.push(':');
            fixed.push_str(&input[len - 2..]);
            if let Ok(value) = DateTime::parse_from_rfc3339(&fixed) {
                return Some(value.naive_utc());
            }
        }
    }

    if let Some(value) = crate::parse_iso_local_datetime(input) {
        return Some(value);
    }
    let (date, time) = input.split_once(' ')?;
    if date.is_empty() || time.is_empty() || time.contains(' ') {
        return None;
    }
    let mut canonical = String::with_capacity(input.len());
    canonical.push_str(date);
    canonical.push('T');
    canonical.push_str(time);
    parse_json_datetime_utc(&canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn converts_offsets_to_utc_across_day_boundaries() {
        let expected = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(18, 15, 0)
            .unwrap();
        assert_eq!(
            parse_json_datetime_utc("2024-03-01T00:00:00+05:45"),
            Some(expected)
        );
        assert_eq!(
            parse_json_datetime_utc("2024-03-01T00:00:00+0545"),
            Some(expected)
        );
        assert_eq!(
            parse_json_datetime_utc("2024-02-29T18:15:00Z"),
            Some(expected)
        );
    }

    #[test]
    fn handles_offset_free_sql_and_iso_inputs_as_utc() {
        let expected = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 30, 40, 123_456_700)
            .unwrap();
        assert_eq!(
            parse_json_datetime_utc("2024-02-29T12:30:40.1234567"),
            Some(expected)
        );
        assert_eq!(
            parse_json_datetime_utc("2024-02-29 12:30:40.1234567"),
            Some(expected)
        );
    }

    #[test]
    fn rejects_bad_offsets_dates_and_extra_characters() {
        for input in [
            "2024-02-30T00:00:00Z",
            "2024-02-29T00:00:00+25:00",
            "2024-02-29T00:00:00+0560",
            "2024-02-29T00:00:00Z ",
            "2024-02-29 00:00:00 ",
            "2024-02-29T00:00:00.1234567890",
            "",
        ] {
            assert_eq!(parse_json_datetime_utc(input), None);
        }
    }
}

#[cfg(test)]
mod space_offset_tests {
    use super::*;

    #[test]
    fn converts_space_separated_rfc3339_and_compact_offsets() {
        let expected = crate::NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(18, 15, 0)
            .unwrap();
        for input in [
            "2024-03-01 00:00:00+05:45",
            "2024-03-01 00:00:00+0545",
            "2024-02-29 18:15:00Z",
        ] {
            assert_eq!(parse_json_datetime_utc(input), Some(expected));
        }
    }

    #[test]
    fn rejects_repeated_spaces_and_invalid_offsets() {
        for input in [
            "2024-02-29  18:15:00Z",
            "2024-02-29 18:15:00 Z",
            "2024-02-29 18:15:00+2560",
        ] {
            assert_eq!(parse_json_datetime_utc(input), None);
        }
    }
}
