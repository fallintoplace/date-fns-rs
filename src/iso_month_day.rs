//! Strict ISO 8601 yearless month-day values in --MM-DD form.

use chrono::NaiveDate;

/// Parse a yearless --MM-DD value into a month and day.
///
/// February 29 is accepted because the result has no fixed calendar year.
pub fn parse_iso_month_day(input: &str) -> Option<(u32, u32)> {
    let bytes = input.as_bytes();
    let valid = bytes.len() == 7
        && bytes[0] == b'-'
        && bytes[1] == b'-'
        && bytes[4] == b'-'
        && bytes[2..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit);
    if !valid {
        return None;
    }
    let month = input[2..4].parse::<u32>().ok()?;
    let day = input[5..7].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(2000, month, day)?;
    Some((month, day))
}

/// Format a valid month and day as exactly --MM-DD.
pub fn format_iso_month_day(month: u32, day: u32) -> Option<String> {
    NaiveDate::from_ymd_opt(2000, month, day)?;
    Some(format!("--{month:02}-{day:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_yearless_days_and_leap_day() {
        assert_eq!(parse_iso_month_day("--02-29"), Some((2, 29)));
        assert_eq!(format_iso_month_day(2, 29).as_deref(), Some("--02-29"));
        assert_eq!(parse_iso_month_day("--12-31"), Some((12, 31)));
        assert_eq!(format_iso_month_day(1, 1).as_deref(), Some("--01-01"));
    }

    #[test]
    fn rejects_impossible_or_noncanonical_month_days() {
        for bad in [
            "--02-30",
            "--04-31",
            "--00-01",
            "--13-01",
            "--01-00",
            "-02-29",
            "--2-29",
            "--02-9",
            "--02-29 ",
            "--02-a9",
            "－－０２－２９",
        ] {
            assert_eq!(parse_iso_month_day(bad), None);
        }
        assert_eq!(format_iso_month_day(2, 30), None);
        assert_eq!(format_iso_month_day(u32::MAX, 1), None);
    }
}
