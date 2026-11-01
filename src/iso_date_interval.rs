//! Strict ISO date intervals in YYYY-MM-DD/YYYY-MM-DD form.

use chrono::NaiveDate;

use crate::iso::{format_iso_date, parse_iso_date};

/// Parse two ISO dates separated by a single slash as an inclusive interval.
///
/// Reversed intervals, whitespace, time parts and expanded years return None.
pub fn parse_iso_date_interval(input: &str) -> Option<(NaiveDate, NaiveDate)> {
    let (left, right) = input.split_once('/')?;
    let start = parse_iso_date(left)?;
    let end = parse_iso_date(right)?;
    (start <= end).then_some((start, end))
}

/// Format an inclusive interval as two strict four-digit ISO calendar dates.
///
/// Returns None for reversed intervals or dates outside the four-digit range.
pub fn format_iso_date_interval(start: NaiveDate, end: NaiveDate) -> Option<String> {
    if start > end {
        return None;
    }
    let left = format_iso_date(start)?;
    let right = format_iso_date(end)?;
    Some(format!("{left}/{right}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn round_trips_leap_day_intervals_and_single_day_ranges() {
        let start = date(2024, 2, 28);
        let end = date(2024, 3, 1);
        let text = "2024-02-28/2024-03-01";
        assert_eq!(parse_iso_date_interval(text), Some((start, end)));
        assert_eq!(format_iso_date_interval(start, end).as_deref(), Some(text));
        assert_eq!(
            parse_iso_date_interval("2024-02-29/2024-02-29"),
            Some((date(2024, 2, 29), date(2024, 2, 29)))
        );
    }

    #[test]
    fn rejects_reversed_noncanonical_and_out_of_range_values() {
        for invalid in [
            "2024-03-01/2024-02-28",
            "2024-02-30/2024-03-01",
            "2024-02-28/2024-03-01/2024-04-01",
            "2024-2-28/2024-03-01",
            "2024-02-28 /2024-03-01",
            "2024-02-28",
        ] {
            assert_eq!(parse_iso_date_interval(invalid), None);
        }
        assert_eq!(
            format_iso_date_interval(NaiveDate::MAX, NaiveDate::MAX),
            None
        );
        assert_eq!(
            format_iso_date_interval(date(2024, 3, 1), date(2024, 2, 29)),
            None
        );
    }
}
