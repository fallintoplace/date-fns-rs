//! Strict ISO 8601 basic calendar-date strings in YYYYMMDD form.

use chrono::{Datelike, NaiveDate};

/// Parse an exactly eight-byte basic ISO calendar date such as 20240229.
///
/// Requires ASCII digits throughout, with no separators or time component.
/// Invalid calendar dates and expanded year formats return None.
pub fn parse_iso_basic_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 8 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let month = input[4..6].parse::<u32>().ok()?;
    let day = input[6..].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

/// Format an ISO 8601 basic calendar date as YYYYMMDD.
///
/// Only years 0000 through 9999 are supported for lossless round trips.
pub fn format_iso_basic_date(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    Some(format!("{year:04}{:02}{:02}", date.month(), date.day()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn round_trips_basic_calendar_dates() {
        let leap_day = date(2024, 2, 29);
        assert_eq!(parse_iso_basic_date("20240229"), Some(leap_day));
        assert_eq!(format_iso_basic_date(leap_day).as_deref(), Some("20240229"));
        assert_eq!(parse_iso_basic_date("00000101"), Some(date(0, 1, 1)));
        assert_eq!(
            format_iso_basic_date(date(9999, 12, 31)).as_deref(),
            Some("99991231")
        );
    }

    #[test]
    fn rejects_noncanonical_and_impossible_dates() {
        for value in [
            "20230229",
            "20241301",
            "20240100",
            "20240230",
            "2024-02-29",
            "2024022",
            "20240229 ",
            "+20240229",
            "2024a229",
        ] {
            assert_eq!(parse_iso_basic_date(value), None, "{value}");
        }
    }

    #[test]
    fn rejects_expanded_and_negative_year_formatting() {
        assert_eq!(format_iso_basic_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_date(NaiveDate::MAX), None);
    }
}
