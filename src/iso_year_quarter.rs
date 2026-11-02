//! Strict YYYY-Qn calendar quarter labels, without implicit fiscal offsets.

use chrono::{Datelike, NaiveDate};

/// Parse a four-digit year and quarter label to the first day of that quarter.
pub fn parse_iso_year_quarter(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 7
        || bytes[4] != b'-'
        || bytes[5] != b'Q'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !(b'1'..=b'4').contains(&bytes[6])
    {
        return None;
    }
    let year = input[..4].parse::<i32>().ok()?;
    let quarter = u32::from(bytes[6] - b'0');
    NaiveDate::from_ymd_opt(year, (quarter - 1) * 3 + 1, 1)
}

/// Format any date as the YYYY-Qn label of its Gregorian quarter.
///
/// Returns None for negative or expanded years.
pub fn format_iso_year_quarter(date: NaiveDate) -> Option<String> {
    let year = date.year();
    if !(0..=9999).contains(&year) {
        return None;
    }
    let quarter = (date.month() - 1) / 3 + 1;
    Some(format!("{year:04}-Q{quarter}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_quarter_starts_and_round_trips_labels() {
        for (label, month) in [
            ("2024-Q1", 1),
            ("2024-Q2", 4),
            ("2024-Q3", 7),
            ("2024-Q4", 10),
            ("0000-Q1", 1),
        ] {
            let start = parse_iso_year_quarter(label).unwrap();
            assert_eq!(start.month(), month);
            assert_eq!(start.day(), 1);
            assert_eq!(format_iso_year_quarter(start).as_deref(), Some(label));
        }
        let middle = NaiveDate::from_ymd_opt(2024, 5, 12).unwrap();
        assert_eq!(format_iso_year_quarter(middle).as_deref(), Some("2024-Q2"));
    }

    #[test]
    fn rejects_malformed_or_out_of_range_quarters() {
        for text in [
            "2024-Q0", "2024-Q5", "2024-q1", "24-Q1", "+2024-Q1", "2024Q1", "2024-Q01", "2024-Q1 ",
        ] {
            assert_eq!(parse_iso_year_quarter(text), None, "{text}");
        }
        assert_eq!(format_iso_year_quarter(NaiveDate::MIN), None);
    }
}
