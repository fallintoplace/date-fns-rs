//! ISO 8601 basic year-month values in YYYYMM form.

use chrono::{Datelike, NaiveDate};

/// Parse exactly YYYYMM, returning the first calendar day of that month.
pub fn parse_iso_basic_year_month(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 6 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let month = input[4..6].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, 1)
}

/// Format a date's month as YYYYMM without changing the day of the input.
pub fn format_iso_basic_year_month(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    Some(format!("{year:04}{:02}", date.month()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_basic_months_and_leap_years() {
        let february = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(parse_iso_basic_year_month("202402"), Some(february));
        assert_eq!(format_iso_basic_year_month(february).as_deref(), Some("202402"));
        assert_eq!(
            format_iso_basic_year_month(NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()).as_deref(),
            Some("202402")
        );
        assert_eq!(
            parse_iso_basic_year_month("000001"),
            NaiveDate::from_ymd_opt(0, 1, 1)
        );
    }

    #[test]
    fn rejects_bad_lengths_characters_months_and_expanded_years() {
        for bad in ["20240", "2024-02", "202413", "202400", "2024a2", " 202402",
                    "+202402", "202402 ", "２０２４０２"] {
            assert_eq!(parse_iso_basic_year_month(bad), None, "{bad}");
        }
        assert_eq!(format_iso_basic_year_month(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_year_month(NaiveDate::MAX), None);
    }
}
