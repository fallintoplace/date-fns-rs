//! Checked calendar-week boundaries for naive date-times.

use chrono::{NaiveDateTime, Weekday};

/// Midnight at the start of the week containing the datetime.
pub fn start_of_week_datetime(value: NaiveDateTime, week_starts_on: Weekday) -> Option<NaiveDateTime> {
    crate::start_of_week(value.date(), week_starts_on)?.and_hms_opt(0, 0, 0)
}

/// Last representable nanosecond of the week containing the datetime.
pub fn end_of_week_datetime(value: NaiveDateTime, week_starts_on: Weekday) -> Option<NaiveDateTime> {
    crate::end_of_week(value.date(), week_starts_on)?
        .and_hms_nano_opt(23, 59, 59, 999_999_999)
}

/// Midnight of Monday in the current ISO week.
pub fn start_of_iso_week_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    start_of_week_datetime(value, Weekday::Mon)
}

/// Last nanosecond of Sunday in the current ISO week.
pub fn end_of_iso_week_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    end_of_week_datetime(value, Weekday::Mon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn week_conventions_cross_year_boundaries() {
        let value = date(2024, 1, 3).and_hms_opt(12, 34, 56).unwrap();
        let monday = date(2024, 1, 1).and_hms_opt(0, 0, 0).unwrap();
        let sunday = date(2023, 12, 31).and_hms_opt(0, 0, 0).unwrap();
        assert_eq!(start_of_week_datetime(value, Weekday::Mon), Some(monday));
        assert_eq!(start_of_week_datetime(value, Weekday::Sun), Some(sunday));
        assert_eq!(start_of_iso_week_datetime(value), Some(monday));
        assert_eq!(
            end_of_week_datetime(value, Weekday::Mon),
            date(2024, 1, 7).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(
            end_of_week_datetime(value, Weekday::Sun),
            date(2024, 1, 6).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(
            end_of_iso_week_datetime(value),
            end_of_week_datetime(value, Weekday::Mon)
        );
    }

    #[test]
    fn week_boundaries_check_chrono_limits() {
        let first = NaiveDateTime::MIN;
        let last = NaiveDateTime::MAX;
        assert_eq!(start_of_week_datetime(first, first.date().weekday()), Some(first));
        assert_eq!(
            start_of_week_datetime(first, first.date().weekday().succ()),
            None
        );
        assert_eq!(end_of_week_datetime(last, last.date().weekday()), None);
        assert_eq!(
            end_of_week_datetime(last, last.date().weekday().succ()),
            Some(last)
        );
    }
}
