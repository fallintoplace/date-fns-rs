//! RFC 3339 timestamps with explicit fixed UTC offsets.

use chrono::{DateTime, Datelike, FixedOffset, SecondsFormat};

/// Parse RFC3339 timestamps including Z or explicit numeric offsets.
pub fn parse_rfc3339_datetime(input: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(input).ok()
}

/// Format a timestamp using an RFC3339 offset and optional fractional seconds.
/// Returns None for years outside the four-digit RFC3339 range.
pub fn format_rfc3339_datetime(value: DateTime<FixedOffset>) -> Option<String> {
    if !(0..=9999).contains(&value.year()) {
        return None;
    }
    Some(value.to_rfc3339_opts(SecondsFormat::AutoSi, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_offsets_and_utc() {
        let original = "2024-02-29T23:59:58.123456789+05:30";
        let parsed = parse_rfc3339_datetime(original).unwrap();
        assert_eq!(parsed.offset().local_minus_utc(), 19_800);
        assert_eq!(format_rfc3339_datetime(parsed).as_deref(), Some(original));
        assert_eq!(format_rfc3339_datetime(parse_rfc3339_datetime("2024-01-01T00:00:00Z").unwrap()).as_deref(), Some("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn rejects_missing_offsets_and_impossible_dates() {
        assert_eq!(parse_rfc3339_datetime("2024-02-29T10:00:00"), None);
        assert_eq!(parse_rfc3339_datetime("2023-02-29T10:00:00Z"), None);
        assert_eq!(parse_rfc3339_datetime("bad"), None);
    }
}