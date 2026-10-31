//! Strict ISO calendar-date strings in `YYYY-MM-DD` form.

use chrono::{Datelike, NaiveDate};

/// Parse an exactly ten-byte `YYYY-MM-DD` date with a four-digit year.
///
/// Only ASCII digits and hyphens at the expected positions are accepted.
/// Returns `None` for impossible dates, whitespace, times, and expanded years.
pub fn parse_iso_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    let valid = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit);
    if !valid {
        return None;
    }

    let year = input[..4].parse::<i32>().ok()?;
    let month = input[5..7].parse::<u32>().ok()?;
    let day = input[8..10].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

/// Format a date as `YYYY-MM-DD` if its year is between 0000 and 9999.
///
/// Chrono also supports expanded and negative years. This intentionally
/// returns `None` for those rather than producing a non-round-trippable string.
pub fn format_iso_date(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }

    Some(format!("{year:04}-{:02}-{:02}", date.month(), date.day()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn round_trips_leap_days_and_zero_padded_dates() {
        let leap_day = date(2024, 2, 29);
        assert_eq!(format_iso_date(leap_day).as_deref(), Some("2024-02-29"));
        assert_eq!(parse_iso_date("2024-02-29"), Some(leap_day));
        assert_eq!(parse_iso_date("0001-01-01"), Some(date(1, 1, 1)));
    }

    #[test]
    fn rejects_invalid_and_noncanonical_dates() {
        assert_eq!(parse_iso_date("2023-02-29"), None);
        assert_eq!(parse_iso_date("2024-13-01"), None);
        assert_eq!(parse_iso_date("2024-01-00"), None);
        assert_eq!(parse_iso_date("2024-1-01"), None);
        assert_eq!(parse_iso_date("2024-01-1"), None);
        assert_eq!(parse_iso_date("2024-01-01 "), None);
        assert_eq!(parse_iso_date("2024-01-01T00:00:00Z"), None);
        assert_eq!(parse_iso_date("+10000-01-01"), None);
    }

    #[test]
    fn formats_only_four_digit_years() {
        assert_eq!(
            format_iso_date(date(0, 1, 1)).as_deref(),
            Some("0000-01-01")
        );
        assert_eq!(parse_iso_date("0000-01-01"), Some(date(0, 1, 1)));
        assert_eq!(format_iso_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_date(NaiveDate::MAX), None);
    }
}
