//! Strict ISO basic yearless month-day values in --MMDD form.

use chrono::NaiveDate;

/// Parse --MMDD into a month and day, without selecting a calendar year.
///
/// February 29 is accepted because no year is supplied.
pub fn parse_iso_basic_month_day(input: &str) -> Option<(u32, u32)> {
    let bytes = input.as_bytes();
    if bytes.len() != 6
        || bytes[..2] != [b'-', b'-']
        || !bytes[2..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let month = input[2..4].parse::<u32>().ok()?;
    let day = input[4..6].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(2000, month, day)?;
    Some((month, day))
}

/// Format a valid month-day pair as --MMDD.
pub fn format_iso_basic_month_day(month: u32, day: u32) -> Option<String> {
    NaiveDate::from_ymd_opt(2000, month, day)?;
    Some(format!("--{month:02}{day:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_annual_dates_and_february_29() {
        for (month, day, text) in [(2, 29, "--0229"), (12, 31, "--1231"), (1, 1, "--0101")] {
            assert_eq!(parse_iso_basic_month_day(text), Some((month, day)));
            assert_eq!(format_iso_basic_month_day(month, day).as_deref(), Some(text));
        }
    }

    #[test]
    fn rejects_bad_calendar_days_and_noncanonical_input() {
        for bad in ["--0230", "--0431", "--0001", "--1301", "--0100",
                    "--02-29", "-0229", "--229", "--0229 ", "--02a9", "－－０２２９"] {
            assert_eq!(parse_iso_basic_month_day(bad), None, "{bad}");
        }
        assert_eq!(format_iso_basic_month_day(2, 30), None);
        assert_eq!(format_iso_basic_month_day(u32::MAX, 1), None);
    }
}
