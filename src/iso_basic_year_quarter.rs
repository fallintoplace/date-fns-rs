//! Compact Gregorian year-quarter labels in YYYYQn form.
//! This is a convenience format matching the crate's YYYY-Qn convention.

use chrono::{Datelike, NaiveDate};

/// Parse a compact Gregorian quarter label into its first calendar day.
pub fn parse_iso_basic_year_quarter(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 6
        || bytes[4] != b'Q'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !(b'1'..=b'4').contains(&bytes[5])
    {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let quarter = u32::from(bytes[5] - b'0');
    NaiveDate::from_ymd_opt(year, (quarter - 1) * 3 + 1, 1)
}

/// Format the Gregorian quarter containing a date as YYYYQn.
pub fn format_iso_basic_year_quarter(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    let quarter = (date.month() - 1) / 3 + 1;
    Some(format!("{year:04}Q{quarter}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quarter_starts_and_formats_middle_dates() {
        for (text, month) in [("2024Q1", 1), ("2024Q2", 4), ("2024Q3", 7),
                              ("2024Q4", 10), ("0000Q1", 1)] {
            let start = parse_iso_basic_year_quarter(text).unwrap();
            assert_eq!(start.month(), month);
            assert_eq!(start.day(), 1);
            assert_eq!(format_iso_basic_year_quarter(start).as_deref(), Some(text));
        }
        let date = NaiveDate::from_ymd_opt(2024, 8, 17).unwrap();
        assert_eq!(format_iso_basic_year_quarter(date).as_deref(), Some("2024Q3"));
    }

    #[test]
    fn rejects_bad_quarters_and_years() {
        for bad in ["2024Q0", "2024Q5", "2024q1", "2024-Q1",
                    "24Q1", "+2024Q1", "2024Q01", "2024Q1 "] {
            assert_eq!(parse_iso_basic_year_quarter(bad), None, "{bad}");
        }
        assert_eq!(format_iso_basic_year_quarter(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_year_quarter(NaiveDate::MAX), None);
    }
}
