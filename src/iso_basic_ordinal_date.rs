//! Strict ISO 8601 basic ordinal dates in YYYYDDD form.

use chrono::{Datelike, NaiveDate};

/// Parse an exactly seven-byte basic ordinal date, such as 2024060.
///
/// The ordinal is 1-based. Day 366 is valid only in leap years.
pub fn parse_iso_basic_ordinal_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 7 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let ordinal = input[4..].parse::<u32>().ok()?;
    NaiveDate::from_yo_opt(year, ordinal)
}

/// Format an ISO basic ordinal date with exactly four year digits.
///
/// Dates outside calendar years 0000..=9999 return None.
pub fn format_iso_basic_ordinal_date(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    Some(format!("{year:04}{:03}", date.ordinal()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn parses_and_formats_leap_and_non_leap_years() {
        let leap_day = date(2024, 2, 29);
        assert_eq!(parse_iso_basic_ordinal_date("2024060"), Some(leap_day));
        assert_eq!(
            format_iso_basic_ordinal_date(leap_day).as_deref(),
            Some("2024060")
        );
        assert_eq!(parse_iso_basic_ordinal_date("2023060"), Some(date(2023, 3, 1)));
        assert_eq!(parse_iso_basic_ordinal_date("2024366"), Some(date(2024, 12, 31)));
    }

    #[test]
    fn rejects_invalid_ordinals_and_noncanonical_input() {
        for value in [
            "2023366",
            "2024000",
            "2024367",
            "202406",
            "2024-060",
            "2024060 ",
            "+2024060",
            "2024a60",
        ] {
            assert_eq!(parse_iso_basic_ordinal_date(value), None, "{value}");
        }
    }

    #[test]
    fn supports_year_zero_but_not_expanded_years() {
        let day = date(0, 12, 31);
        assert_eq!(format_iso_basic_ordinal_date(day).as_deref(), Some("0000366"));
        assert_eq!(parse_iso_basic_ordinal_date("0000366"), Some(day));
        assert_eq!(format_iso_basic_ordinal_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_ordinal_date(NaiveDate::MAX), None);
    }
}
