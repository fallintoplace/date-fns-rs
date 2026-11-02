//! RFC 2822 email timestamps with explicit numeric or named UTC offsets.

use chrono::{DateTime, FixedOffset};

/// Parse an email header timestamp and retain its explicit UTC offset.
///
/// This uses Chrono's RFC 2822 parser. No system timezone is consulted.
pub fn parse_rfc2822_datetime(input: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc2822(input).ok()
}

/// Format an offset-aware timestamp using Chrono's RFC 2822 serializer.
pub fn format_rfc2822_datetime(value: DateTime<FixedOffset>) -> String {
    value.to_rfc2822()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_known_rfc_2822_examples_and_offsets() {
        let original = "Fri, 21 Nov 1997 09:55:06 -0600";
        let value = parse_rfc2822_datetime(original).unwrap();
        assert_eq!(value.offset().local_minus_utc(), -21_600);
        assert_eq!(format_rfc2822_datetime(value), original);
        assert_eq!(
            parse_rfc2822_datetime(&format_rfc2822_datetime(value)),
            Some(value)
        );
    }

    #[test]
    fn rejects_non_email_timestamp_syntax_and_invalid_clocks() {
        for invalid in [
            "2024-02-29T12:00:00Z",
            "Thu, 30 Feb 2024 12:00:00 +0000",
            "Thu, 29 Feb 2024 25:00:00 +0000",
            "Thu, 29 Feb 2024 12:00:00",
            "",
        ] {
            assert_eq!(parse_rfc2822_datetime(invalid), None, "{invalid}");
        }
    }
}
