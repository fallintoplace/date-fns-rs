//! Canonical RFC 7231 (IMF-fixdate) timestamps for HTTP headers.

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};

const HTTP_DATE: &str = "%a, %d %b %Y %H:%M:%S GMT";

/// Format an HTTP date using the required GMT zone and English names.
///
/// Dates with years outside 0000-9999 or fractional seconds cannot be
/// encoded without losing precision, so they return None.
pub fn format_rfc7231_datetime(value: DateTime<Utc>) -> Option<String> {
    if value.timestamp_subsec_nanos() != 0 || !(0..=9999).contains(&value.year()) {
        return None;
    }
    Some(value.format(HTTP_DATE).to_string())
}

/// Parse only canonical IMF-fixdate values, rejecting weekday mismatches.
///
/// Obsolete HTTP-date formats are deliberately not accepted.
pub fn parse_rfc7231_datetime(input: &str) -> Option<DateTime<Utc>> {
    if input.len() != 29 || !input.ends_with(" GMT") {
        return None;
    }
    let naive = NaiveDateTime::parse_from_str(input, HTTP_DATE).ok()?;
    let value = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
    (format_rfc7231_datetime(value).as_deref() == Some(input)).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_the_http_specification_example() {
        let input = "Sun, 06 Nov 1994 08:49:37 GMT";
        let value = parse_rfc7231_datetime(input).unwrap();
        assert_eq!(
            value.naive_utc().date(),
            NaiveDate::from_ymd_opt(1994, 11, 6).unwrap()
        );
        assert_eq!(format_rfc7231_datetime(value).as_deref(), Some(input));
    }

    #[test]
    fn validates_weekdays_zones_and_whitespace() {
        for input in [
            "Mon, 06 Nov 1994 08:49:37 GMT",
            "Sun, 06 Nov 1994 08:49:37 UTC",
            "Sun, 06 Nov 1994 08:49:37 GMT ",
            "Sun, 31 Nov 1994 08:49:37 GMT",
            "Sun, 06 Nov 1994 24:49:37 GMT",
            "Sunday, 06-Nov-94 08:49:37 GMT",
        ] {
            assert_eq!(parse_rfc7231_datetime(input), None);
        }
    }

    #[test]
    fn rejects_non_four_digit_years() {
        let value = NaiveDate::from_ymd_opt(10000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        assert_eq!(format_rfc7231_datetime(value), None);
    }
}

#[cfg(test)]
mod precision_tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    #[test]
    fn rejects_fractional_seconds_instead_of_silently_truncating_them() {
        let second = NaiveDate::from_ymd_opt(1994, 11, 6)
            .unwrap()
            .and_hms_opt(8, 49, 37)
            .unwrap()
            .and_utc();
        assert_eq!(
            format_rfc7231_datetime(second).as_deref(),
            Some("Sun, 06 Nov 1994 08:49:37 GMT")
        );
        let subsecond = second + TimeDelta::milliseconds(1);
        assert_eq!(format_rfc7231_datetime(subsecond), None);
        assert_eq!(
            parse_rfc7231_datetime("Sun, 06 Nov 1994 08:49:37 GMT"),
            Some(second)
        );
    }
}
