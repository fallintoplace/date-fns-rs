//! Reduced-precision ISO 8601 basic week labels in YYYYWww form.

use chrono::{Datelike, NaiveDate, Weekday};

/// Parse a basic week label and return the ISO Monday beginning that week.
///
/// Weeks are validated for their ISO week-year, including week 53.
pub fn parse_iso_basic_week(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 7
        || bytes[4] != b'W'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let week = input[5..].parse::<u32>().ok()?;
    NaiveDate::from_isoywd_opt(year, week, Weekday::Mon)
}

/// Format the ISO week containing date, using the ISO week-year.
pub fn format_iso_basic_week(date: NaiveDate) -> Option<String> {
    let iso = date.iso_week();
    if !(0..=9999).contains(&iso.year()) {
        return None;
    }
    Some(format!("{:04}W{:02}", iso.year(), iso.week()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_iso_week_year_across_the_new_year() {
        let monday = NaiveDate::from_ymd_opt(2020, 12, 28).unwrap();
        assert_eq!(parse_iso_basic_week("2020W53"), Some(monday));
        assert_eq!(
            format_iso_basic_week(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()).as_deref(),
            Some("2020W53")
        );
        assert_eq!(format_iso_basic_week(monday).as_deref(), Some("2020W53"));
        assert_eq!(
            parse_iso_basic_week("2019W01"),
            NaiveDate::from_ymd_opt(2018, 12, 31)
        );
    }

    #[test]
    fn rejects_bad_week_numbers_and_nonbasic_text() {
        for bad in ["2021W53", "2024W00", "2024W54", "2024W1", "2024w01",
                    "2024-W01", "2024W011", " 2024W01", "+2024W01", "2024W0a"] {
            assert_eq!(parse_iso_basic_week(bad), None, "{bad}");
        }
        assert_eq!(format_iso_basic_week(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_week(NaiveDate::MAX), None);
    }
}
