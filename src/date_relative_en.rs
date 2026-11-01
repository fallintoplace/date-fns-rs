//! English relative-day labels anchored to a caller-provided calendar date.

use chrono::{Datelike, NaiveDate};

const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// Format a calendar day relative to another explicit day.
/// Days 2-6 ahead use "next Friday"; days 2-6 behind use "last Friday".
/// Distant dates use ISO YYYY-MM-DD when possible, or Chrono's expanded date.
pub fn format_relative_calendar_date_en(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => "today".to_owned(),
        1 => "tomorrow".to_owned(),
        -1 => "yesterday".to_owned(),
        2..=6 => {
            let name = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            format!("next {name}")
        }
        -6..=-2 => {
            let name = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            format!("last {name}")
        }
        _ => crate::format_iso_date(target).unwrap_or_else(|| target.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_days_across_week_and_year_boundaries() {
        let monday = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let friday = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        let previous = NaiveDate::from_ymd_opt(2024, 12, 27).unwrap();
        assert_eq!(format_relative_calendar_date_en(monday, monday), "today");
        assert_eq!(
            format_relative_calendar_date_en(tuesday, monday),
            "tomorrow"
        );
        assert_eq!(
            format_relative_calendar_date_en(monday, tuesday),
            "yesterday"
        );
        assert_eq!(
            format_relative_calendar_date_en(friday, monday),
            "next Friday"
        );
        assert_eq!(
            format_relative_calendar_date_en(previous, monday),
            "last Friday"
        );
        assert_eq!(
            format_relative_calendar_date_en(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(), monday),
            "2025-01-20"
        );
    }
}
