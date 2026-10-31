//! ISO 8601 basic ordinal dates in YYYYDDD form.

use chrono::{Datelike, NaiveDate};

/// Parse exactly seven ASCII digits as a basic ISO ordinal date.
///
/// The ordinal is one-based and day 366 is valid only in leap years.
pub fn parse_iso_basic_ordinal_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 7 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let ordinal = input[4..].parse::<u32>().ok()?;
    NaiveDate::from_yo_opt(year, ordinal)
}

/// Format an ISO ordinal date as YYYYDDD.
///
/// Returns None for years outside the four-digit range 0000 through 9999.
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

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn round_trips_leap_and_common_years() {
        let leap = date(2024, 2, 29);
        assert_eq!(parse_iso_basic_ordinal_date("2024060"), Some(leap));
        assert_eq!(
            format_iso_basic_ordinal_date(leap).as_deref(),
            Some("2024060")
        );
        assert_eq!(
            parse_iso_basic_ordinal_date("2023365"),
            Some(date(2023, 12, 31))
        );
        assert_eq!(
            parse_iso_basic_ordinal_date("0000366"),
            Some(date(0, 12, 31))
        );
    }

    #[test]
    fn rejects_invalid_and_noncanonical_inputs() {
        for input in [
            "2023366", "2024000", "2024367", "2024-060", "202406", "2024060 ",
            "+2024060", "2024a60",
        ] {
            assert_eq!(parse_iso_basic_ordinal_date(input), None, "{input}");
        }
        assert_eq!(format_iso_basic_ordinal_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_ordinal_date(NaiveDate::MAX), None);
    }
}
