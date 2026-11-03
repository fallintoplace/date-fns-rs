//! Complete ISO week ranges in YYYY-Www/YYYY-Www form.

use chrono::{Datelike, NaiveDate, Weekday};

/// Parse two ISO week labels into an inclusive Monday-to-Sunday date range.
///
/// Invalid, reversed, or unrepresentable ranges return None.
pub fn parse_iso_week_interval(input: &str) -> Option<(NaiveDate, NaiveDate)> {
    let (left, right) = input.split_once('/')?;
    let start = crate::parse_iso_week(left)?;
    let last_monday = crate::parse_iso_week(right)?;
    if start > last_monday {
        return None;
    }
    let end = crate::add_days(last_monday, 6)?;
    Some((start, end))
}

/// Format an inclusive range of complete ISO weeks.
///
/// The start must be Monday and the end Sunday; partial weeks are rejected.
pub fn format_iso_week_interval(start: NaiveDate, end: NaiveDate) -> Option<String> {
    if start > end || start.weekday() != Weekday::Mon || end.weekday() != Weekday::Sun {
        return None;
    }
    let left = crate::format_iso_week(start)?;
    let right = crate::format_iso_week(end)?;
    Some(format!("{left}/{right}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn round_trips_weeks_across_iso_year_boundaries() {
        let start = date(2020, 12, 28);
        let end = date(2021, 1, 10);
        let label = "2020-W53/2021-W01";
        assert_eq!(parse_iso_week_interval(label), Some((start, end)));
        assert_eq!(format_iso_week_interval(start, end).as_deref(), Some(label));
        assert_eq!(
            parse_iso_week_interval("2019-W01/2019-W01"),
            Some((date(2018, 12, 31), date(2019, 1, 6)))
        );
    }

    #[test]
    fn rejects_invalid_and_partial_week_ranges() {
        for invalid in [
            "2021-W53/2022-W01",
            "2024-W03/2024-W02",
            "2024-W01/2024-W02/2024-W03",
            "2024-W1/2024-W02",
            "2024-W01 /2024-W02",
        ] {
            assert_eq!(parse_iso_week_interval(invalid), None, "{invalid}");
        }
        assert_eq!(
            format_iso_week_interval(date(2024, 1, 2), date(2024, 1, 7)),
            None
        );
        assert_eq!(
            format_iso_week_interval(date(2024, 1, 1), date(2024, 1, 6)),
            None
        );
        assert_eq!(
            format_iso_week_interval(date(2024, 1, 8), date(2024, 1, 7)),
            None
        );
    }
}
