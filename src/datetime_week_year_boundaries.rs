//! Full-day datetime boundaries for configurable and ISO week-numbering years.

use chrono::{NaiveDateTime, Weekday};

/// Midnight at the beginning of the week-numbering year containing the value.
pub fn start_of_week_year_datetime(
    value: NaiveDateTime,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDateTime> {
    crate::start_of_week_year(value.date(), week_starts_on, first_week_contains_date)?
        .and_hms_opt(0, 0, 0)
}

/// Last nanosecond of the week-numbering year containing the value.
pub fn end_of_week_year_datetime(
    value: NaiveDateTime,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<NaiveDateTime> {
    crate::end_of_week_year(value.date(), week_starts_on, first_week_contains_date)?
        .and_hms_nano_opt(23, 59, 59, 999_999_999)
}

/// Midnight on the Monday beginning the current ISO week-numbering year.
pub fn start_of_iso_week_year_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::start_of_iso_week_year(value.date())?.and_hms_opt(0, 0, 0)
}

/// Last nanosecond of the Sunday ending the current ISO week-numbering year.
pub fn end_of_iso_week_year_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::end_of_iso_week_year(value.date())?.and_hms_nano_opt(23, 59, 59, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn preserves_iso_53_week_year_boundaries_across_calendar_years() {
        let january = date(2021, 1, 1).and_hms_opt(18, 0, 0).unwrap();
        let start = date(2019, 12, 30).and_hms_opt(0, 0, 0);
        let end = date(2021, 1, 3).and_hms_nano_opt(23, 59, 59, 999_999_999);
        assert_eq!(start_of_iso_week_year_datetime(january), start);
        assert_eq!(end_of_iso_week_year_datetime(january), end);
        assert_eq!(start_of_week_year_datetime(january, Weekday::Mon, 4), start);
        assert_eq!(end_of_week_year_datetime(january, Weekday::Mon, 4), end);
    }

    #[test]
    fn supports_custom_sunday_first_week_years() {
        let january = date(2021, 1, 1).and_hms_opt(0, 0, 0).unwrap();
        assert_eq!(
            start_of_week_year_datetime(january, Weekday::Sun, 1),
            date(2020, 12, 27).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_week_year_datetime(january, Weekday::Sun, 1),
            date(2021, 12, 25).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(start_of_week_year_datetime(january, Weekday::Mon, 0), None);
        assert_eq!(end_of_week_year_datetime(january, Weekday::Mon, 8), None);
        assert_eq!(end_of_iso_week_year_datetime(NaiveDateTime::MAX), None);
    }
}
