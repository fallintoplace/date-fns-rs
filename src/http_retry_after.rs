//! RFC 9110 Retry-After field values without implicit clock access.

use chrono::{DateTime, Utc};

/// A Retry-After value expressed as seconds or an absolute UTC date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetryAfter {
    /// The number of seconds to wait after receiving the response.
    DelaySeconds(u64),
    /// A canonical HTTP-date, interpreted in UTC.
    DateTime(DateTime<Utc>),
}

/// Parse an HTTP Retry-After header using either accepted wire form.
///
/// The delay form accepts ASCII decimal digits only. Absolute dates use
/// canonical IMF-fixdate and reject obsolete date forms and wrong weekdays.
pub fn parse_http_retry_after(input: &str) -> Option<RetryAfter> {
    if !input.is_empty() && input.bytes().all(|byte| byte.is_ascii_digit()) {
        return input.parse::<u64>().ok().map(RetryAfter::DelaySeconds);
    }
    crate::parse_rfc7231_datetime(input).map(RetryAfter::DateTime)
}

/// Format a Retry-After value without losing timestamp precision.
///
/// Absolute dates require whole seconds and four-digit years. Delay values
/// are emitted as canonical decimal integers without leading zeroes.
pub fn format_http_retry_after(value: &RetryAfter) -> Option<String> {
    match value {
        RetryAfter::DelaySeconds(seconds) => Some(seconds.to_string()),
        RetryAfter::DateTime(value) => crate::format_rfc7231_datetime(value.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_delta_seconds_and_http_dates() {
        let date_text = "Sun, 06 Nov 1994 08:49:37 GMT";
        let date = parse_http_retry_after(date_text).unwrap();
        assert_eq!(format_http_retry_after(&date).as_deref(), Some(date_text));
        assert_eq!(
            parse_http_retry_after("0"),
            Some(RetryAfter::DelaySeconds(0))
        );
        assert_eq!(
            parse_http_retry_after("000120"),
            Some(RetryAfter::DelaySeconds(120))
        );
        assert_eq!(
            format_http_retry_after(&RetryAfter::DelaySeconds(120)).as_deref(),
            Some("120")
        );
    }

    #[test]
    fn rejects_invalid_whitespace_dates_and_overflowing_delays() {
        for invalid in [
            "",
            " 120",
            "120 ",
            "+120",
            "-1",
            "1.5",
            "18446744073709551616",
            "Mon, 06 Nov 1994 08:49:37 GMT",
            "Sun, 06 Nov 1994 08:49:37 UTC",
        ] {
            assert_eq!(parse_http_retry_after(invalid), None, "{invalid}");
        }
    }

    #[test]
    fn refuses_precision_loss_for_absolute_dates() {
        let date = crate::parse_rfc7231_datetime("Sun, 06 Nov 1994 08:49:37 GMT").unwrap();
        let fractional = date + chrono::TimeDelta::milliseconds(1);
        assert_eq!(
            format_http_retry_after(&RetryAfter::DateTime(fractional)),
            None
        );
    }
}
