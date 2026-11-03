//! German calendar-relative labels with an explicit reference date.

use chrono::{Datelike, NaiveDate};

const WEEKDAYS: [&str; 7] = [
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];

/// Format a date relative to the provided reference date in German.
///
/// Adjacent days use heute, morgen and gestern. Days two to six ahead
/// use nächsten Freitag; days two to six behind use letzten Freitag.
/// More distant dates use ISO calendar dates when representable.
pub fn format_relative_calendar_date_de(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => "heute".to_owned(),
        1 => "morgen".to_owned(),
        -1 => "gestern".to_owned(),
        2..=6 => {
            let day = WEEKDAYS[target.weekday().num_days_from_monday() as usize];
            format!("nächsten {day}")
        }
        -6..=-2 => {
            let day = WEEKDAYS[target.weekday().num_days_from_monday() as usize];
            format!("letzten {day}")
        }
        _ => crate::format_iso_date(target).unwrap_or_else(|| target.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn formats_adjacent_days_and_weekdays_across_years() {
        let monday = day(2024, 12, 30);
        assert_eq!(format_relative_calendar_date_de(monday, monday), "heute");
        assert_eq!(
            format_relative_calendar_date_de(day(2024, 12, 31), monday),
            "morgen"
        );
        assert_eq!(
            format_relative_calendar_date_de(day(2024, 12, 29), monday),
            "gestern"
        );
        assert_eq!(
            format_relative_calendar_date_de(day(2025, 1, 3), monday),
            "nächsten Freitag"
        );
        assert_eq!(
            format_relative_calendar_date_de(day(2024, 12, 27), monday),
            "letzten Freitag"
        );
    }

    #[test]
    fn formats_distant_dates_without_changing_date_ranges() {
        let reference = day(2024, 1, 1);
        assert_eq!(
            format_relative_calendar_date_de(day(2024, 1, 20), reference),
            "2024-01-20"
        );
        assert_eq!(
            format_relative_calendar_date_de(NaiveDate::MAX, reference),
            NaiveDate::MAX.to_string()
        );
    }
}
