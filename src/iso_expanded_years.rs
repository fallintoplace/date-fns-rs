//! ISO calendar dates with optional signed six-digit expanded years.

use chrono::{Datelike, NaiveDate};

/// Parse YYYY-MM-DD or signed expanded year dates such as +010000-01-01.
/// Expanded form requires exactly six year digits, including leading zeroes.
pub fn parse_iso_date_extended(input: &str) -> Option<NaiveDate> {
    if let Some(date) = crate::parse_iso_date(input) {
        return Some(date);
    }
    let b = input.as_bytes();
    if b.len() != 13 || (b[0] != b'+' && b[0] != b'-') || b[7] != b'-' || b[10] != b'-'
        || !b[1..7].iter().all(u8::is_ascii_digit)
        || !b[8..10].iter().all(u8::is_ascii_digit)
        || !b[11..13].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let abs_year = input[1..7].parse::<i32>().ok()?;
    let year = if b[0] == b'-' { -abs_year } else { abs_year };
    let month = input[8..10].parse::<u32>().ok()?;
    let day = input[11..13].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

/// Format four-digit years normally and other years in signed expanded form.
pub fn format_iso_date_extended(date: NaiveDate) -> String {
    if let Some(ordinary) = crate::format_iso_date(date) {
        return ordinary;
    }
    let year = i64::from(date.year());
    let sign = if year < 0 { '-' } else { '+' };
    format!("{sign}{:06}-{:02}-{:02}", year.abs(), date.month(), date.day())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_extended_and_standard_years() {
        for date in [NaiveDate::MIN, NaiveDate::MAX, NaiveDate::from_ymd_opt(-1, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(10_000, 2, 29).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()] {
            let text = format_iso_date_extended(date);
            assert_eq!(parse_iso_date_extended(&text), Some(date));
        }
        assert_eq!(format_iso_date_extended(NaiveDate::from_ymd_opt(10_000, 1, 1).unwrap()), "+010000-01-01");
    }

    #[test]
    fn rejects_malformed_and_impossible_expanded_years() {
        for bad in ["+10000-01-01", "+010000-2-01", "+010000-02-30",
            "10000-01-01", "2024-01-01 ", "-000001-13-01", "+010000-01-01Z"] {
            assert_eq!(parse_iso_date_extended(bad), None);
        }
    }
}