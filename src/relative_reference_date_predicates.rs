//! Relative calendar period predicates without an implicit system clock.

use chrono::{Datelike, NaiveDate, Weekday};

/// Check membership in the reference date's Gregorian calendar month.
pub fn is_this_month(date: NaiveDate, reference: NaiveDate) -> bool {
    crate::is_same_month(date, reference)
}

/// Check membership in the reference date's Gregorian calendar year.
pub fn is_this_calendar_year(date: NaiveDate, reference: NaiveDate) -> bool {
    crate::is_same_year(date, reference)
}

/// Check membership in the reference date's Gregorian calendar quarter.
pub fn is_this_calendar_quarter(date: NaiveDate, reference: NaiveDate) -> bool {
    date.year() == reference.year() && (date.month() - 1) / 3 == (reference.month() - 1) / 3
}

/// Check membership in the reference date's week using a chosen first day.
pub fn is_this_calendar_week(
    date: NaiveDate,
    reference: NaiveDate,
    week_starts_on: Weekday,
) -> bool {
    crate::is_same_week(date, reference, week_starts_on)
}

/// Check membership in the reference date's Monday-first ISO week.
pub fn is_this_iso_calendar_week(date: NaiveDate, reference: NaiveDate) -> bool {
    crate::is_same_iso_week(date, reference)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn compares_reference_months_quarters_and_years() {
        let reference = date(2024, 3, 31);
        assert!(is_this_month(date(2024, 3, 1), reference));
        assert!(!is_this_month(date(2024, 4, 1), reference));
        assert!(is_this_calendar_quarter(date(2024, 1, 1), reference));
        assert!(!is_this_calendar_quarter(date(2024, 4, 1), reference));
        assert!(is_this_calendar_year(date(2024, 12, 31), reference));
        assert!(!is_this_calendar_year(date(2025, 1, 1), reference));
    }

    #[test]
    fn supports_configured_weeks_across_year_boundaries() {
        let sunday = date(2023, 12, 31);
        let monday = date(2024, 1, 1);
        assert!(is_this_calendar_week(sunday, monday, Weekday::Sun));
        assert!(!is_this_calendar_week(sunday, monday, Weekday::Mon));
        assert!(!is_this_iso_calendar_week(sunday, monday));
        assert!(is_this_iso_calendar_week(
            date(2019, 12, 30),
            date(2020, 1, 5)
        ));
        assert!(is_this_calendar_week(
            NaiveDate::MIN,
            NaiveDate::MIN,
            Weekday::Mon
        ));
    }
}
