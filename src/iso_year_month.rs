//! Strict reduced-precision ISO year-month values in YYYY-MM form.

use chrono::{Datelike, NaiveDate};

/// Parse an ISO year-month as the first day of the represented month.
///
/// Requires exactly four year digits, a hyphen, and two month digits.
pub fn parse_iso_year_month(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 7
        || bytes[4] != b'-'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let month = input[5..].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, 1)
}

/// Format the year and month of a date as YYYY-MM, omitting its day.
///
/// Returns None for dates outside the four-digit year range.
pub fn format_iso_year_month(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    Some(format!("{year:04}-{:02}", date.month()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn parses_first_day_and_formats_month_of_any_date() {
        assert_eq!(
            parse_iso_year_month("2024-02"),
            Some(date(2024, 2, 1))
        );
        assert_eq!(format_iso_year_month(date(2024, 2, 29)).as_deref(), Some("2024-02"));
        assert_eq!(parse_iso_year_month("0000-01"), Some(date(0, 1, 1)));
        assert_eq!(format_iso_year_month(date(9999, 12, 31)).as_deref(), Some("9999-12"));
    }

    #[test]
    fn rejects_invalid_or_noncanonical_months() {
        for input in [
            "2024-00", "2024-13", "2024-2", "202402", "2024/02",
            "2024-02 ", "+2024-02", "2024-0a",
        ] {
            assert_eq!(parse_iso_year_month(input), None, "{input}");
        }
        assert_eq!(format_iso_year_month(NaiveDate::MIN), None);
        assert_eq!(format_iso_year_month(NaiveDate::MAX), None);
    }
}
