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
    let mut canonical = String::with_capacity(input.len());
    canonical.push_str(date);
    canonical.push('T');
    canonical.push_str(time);
    crate::parse_iso_local_datetime(&canonical)
}

/// Format a naive UTC timestamp as an explicit JSON-friendly ISO string.
///
/// Fractional nanoseconds are preserved. Returns None for years that cannot
/// be expressed as strict four-digit ISO dates or invalid leap-second values.
pub fn format_json_datetime_utc(value: NaiveDateTime) -> Option<String> {
    Some(format!("{}Z", crate::format_iso_local_datetime(value)?))
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
mod json_formatter_tests {
    use super::*;

    #[test]
    fn round_trips_utc_timestamps_without_losing_nanoseconds() {
        let value = chrono::NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_400_001)
            .unwrap();
        let expected = "2024-02-29T23:59:58.123400001Z";
        assert_eq!(format_json_datetime_utc(value).as_deref(), Some(expected));
        assert_eq!(parse_json_datetime_utc(expected), Some(value));

        let whole = value.date().and_hms_opt(0, 0, 0).unwrap();
        assert_eq!(
            format_json_datetime_utc(whole).as_deref(),
            Some("2024-02-29T00:00:00Z")
        );
    }

    #[test]
    fn rejects_unsupported_years_without_truncating() {
        assert_eq!(format_json_datetime_utc(NaiveDateTime::MAX), None);
    }
}
