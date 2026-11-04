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

/// Format a timestamp without silently dropping fractional seconds.
///
/// RFC 2822 represents whole seconds, four-digit years, and minute-precision
/// offsets. Returns None when the timestamp cannot be represented faithfully.
pub fn try_format_rfc2822_datetime(value: DateTime<FixedOffset>) -> Option<String> {
    use chrono::Datelike;

    if value.timestamp_subsec_nanos() != 0
        || !(0..=9999).contains(&value.year())
        || value.offset().local_minus_utc() % 60 != 0
    {
        return None;
    }
    Some(format_rfc2822_datetime(value))
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

#[cfg(test)]
mod precision_tests {
    use super::*;

    #[test]
    fn rejects_fractional_seconds_before_serialization() {
        let whole = parse_rfc2822_datetime("Fri, 21 Nov 1997 09:55:06 -0600").unwrap();
        assert_eq!(
            try_format_rfc2822_datetime(whole).as_deref(),
            Some("Fri, 21 Nov 1997 09:55:06 -0600")
        );
        let fractional = whole + chrono::TimeDelta::milliseconds(1);
        assert_eq!(try_format_rfc2822_datetime(fractional), None);
    }

    #[test]
    fn rejects_years_without_four_digit_rfc2822_encoding() {
        let far = chrono::NaiveDate::from_ymd_opt(10000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .fixed_offset();
        assert_eq!(try_format_rfc2822_datetime(far), None);
    }
    #[test]
    fn checked_formatter_rejects_second_precision_offsets() {
        let utc = chrono::NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_utc();
        let seconds = utc.with_timezone(&FixedOffset::east_opt(-3_661).unwrap());
        assert_eq!(try_format_rfc2822_datetime(seconds), None);
        let minutes = utc.with_timezone(&FixedOffset::east_opt(-3_660).unwrap());
        assert!(try_format_rfc2822_datetime(minutes).is_some());
    }
}
