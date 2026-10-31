//! Strict ISO ordinal dates in the form YYYY-DDD.

use chrono::{Datelike, NaiveDate};

/// Parse an exactly eight-byte ISO ordinal date, such as 2024-060.
///
/// Values are 1-based. Day 366 is valid only in leap years.
pub fn parse_iso_ordinal_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 8
        || bytes[4] != b'-'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let ordinal = input[5..].parse::<u32>().ok()?;
    NaiveDate::from_yo_opt(year, ordinal)
}

/// Format a date as YYYY-DDD when the year is between 0000 and 9999.
///
/// Expanded and negative calendar years return None to preserve round trips.
pub fn format_iso_ordinal_date(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    Some(format!("{year:04}-{:03}", date.ordinal()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn parses_leap_and_common_year_ordinals() {
        assert_eq!(parse_iso_ordinal_date("2024-060"), Some(date(2024, 2, 29)));
        assert_eq!(parse_iso_ordinal_date("2023-060"), Some(date(2023, 3, 1)));
        assert_eq!(parse_iso_ordinal_date("2024-366"), Some(date(2024, 12, 31)));
        assert_eq!(format_iso_ordinal_date(date(2024, 2, 29)).as_deref(), Some("2024-060"));
    }

    #[test]
    fn rejects_invalid_ordinals_and_spelling() {
        for value in [
            "2023-366",
            "2024-000",
            "2024-367",
            "2024-60",
            "2024-060 ",
            "2024/060",
            "2024-0a0",
            "-001-060",
        ] {
            assert_eq!(parse_iso_ordinal_date(value), None, "{value}");
        }
    }

    #[test]
    fn round_trips_year_zero_and_rejects_expanded_years() {
        let value = date(0, 12, 31);
        assert_eq!(format_iso_ordinal_date(value).as_deref(), Some("0000-366"));
        assert_eq!(parse_iso_ordinal_date("0000-366"), Some(value));
        assert_eq!(format_iso_ordinal_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_ordinal_date(NaiveDate::MAX), None);
    }
}
