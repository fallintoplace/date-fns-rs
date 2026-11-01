//! Strict ISO week-only values in YYYY-Www form.

use chrono::{Datelike, NaiveDate, Weekday};

/// Parse a reduced ISO week string as the Monday starting that ISO week.
///
/// Week 53 is accepted only in ISO years that contain it. Expanded years,
/// whitespace and explicit weekday suffixes are rejected.
pub fn parse_iso_week(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 8
        || bytes[4] != b'-'
        || bytes[5] != b'W'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[6..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let week = input[6..8].parse::<u32>().ok()?;
    NaiveDate::from_isoywd_opt(year, week, Weekday::Mon)
}

/// Format the ISO week containing date as a four-digit week-year and week.
///
/// The ISO week-year may differ from the date's calendar year near New Year.
pub fn format_iso_week(date: NaiveDate) -> Option<String> {
    let week = date.iso_week();
    if !(0..=9999).contains(&week.year()) {
        return None;
    }
    Some(format!("{:04}-W{:02}", week.year(), week.week()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn formats_iso_week_year_and_parses_monday_start() {
        assert_eq!(parse_iso_week("2020-W53"), Some(date(2020, 12, 28)));
        assert_eq!(format_iso_week(date(2021, 1, 1)).as_deref(), Some("2020-W53"));
        assert_eq!(parse_iso_week("2019-W01"), Some(date(2018, 12, 31)));
        assert_eq!(format_iso_week(date(2018, 12, 31)).as_deref(), Some("2019-W01"));
        assert_eq!(format_iso_week(date(2024, 2, 29)).as_deref(), Some("2024-W09"));
    }

    #[test]
    fn rejects_invalid_weeks_and_noncanonical_input() {
        for invalid in [
            "2021-W53",
            "2024-W00",
            "2024-W54",
            "2024-W1",
            "2024-w01",
            "2024-W01-1",
            "2024-W01 ",
            "+2024-W01",
        ] {
            assert_eq!(parse_iso_week(invalid), None, "{invalid}");
        }
        assert_eq!(format_iso_week(NaiveDate::MIN), None);
        assert_eq!(format_iso_week(NaiveDate::MAX), None);
    }
}
