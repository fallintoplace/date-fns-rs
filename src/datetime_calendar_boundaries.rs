//! Inclusive calendar-period boundaries for naive date-times.

use chrono::{NaiveDate, NaiveDateTime};

fn first_in_day(date: NaiveDate) -> NaiveDateTime {
    date.and_hms_opt(0, 0, 0).unwrap()
}

fn last_in_day(date: NaiveDate) -> NaiveDateTime {
    date.and_hms_nano_opt(23, 59, 59, 999_999_999).unwrap()
}

/// Start of the calendar month at midnight.
pub fn start_of_month_datetime(value: NaiveDateTime) -> NaiveDateTime {
    first_in_day(crate::start_of_month(value.date()))
}

/// Last nanosecond of the calendar month.
pub fn end_of_month_datetime(value: NaiveDateTime) -> NaiveDateTime {
    last_in_day(crate::end_of_month(value.date()))
}

/// Start of the calendar quarter at midnight.
pub fn start_of_quarter_datetime(value: NaiveDateTime) -> NaiveDateTime {
    first_in_day(crate::start_of_quarter(value.date()))
}

/// Last nanosecond of the calendar quarter.
pub fn end_of_quarter_datetime(value: NaiveDateTime) -> NaiveDateTime {
    last_in_day(crate::end_of_quarter(value.date()))
}

/// Start of the calendar year at midnight.
pub fn start_of_year_datetime(value: NaiveDateTime) -> NaiveDateTime {
    first_in_day(crate::start_of_year(value.date()))
}

/// Last nanosecond of the calendar year.
pub fn end_of_year_datetime(value: NaiveDateTime) -> NaiveDateTime {
    last_in_day(crate::end_of_year(value.date()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dt(year: i32, month: u32, day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_nano_opt(14, 25, 39, 123_456_789)
            .unwrap()
    }

    #[test]
    fn handles_leap_month_quarter_and_year_boundaries() {
        let value = dt(2024, 2, 29);
        assert_eq!(
            start_of_month_datetime(value),
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
        );
        assert_eq!(
            end_of_month_datetime(value),
            NaiveDate::from_ymd_opt(2024, 2, 29)
                .unwrap()
                .and_hms_nano_opt(23, 59, 59, 999_999_999)
                .unwrap()
        );
        assert_eq!(start_of_quarter_datetime(value), start_of_year_datetime(value));
        assert_eq!(
            end_of_quarter_datetime(value).date(),
            NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()
        );
        assert_eq!(
            end_of_year_datetime(value).date(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );
        assert_eq!(
            start_of_quarter_datetime(dt(2024, 11, 30)).date(),
            NaiveDate::from_ymd_opt(2024, 10, 1).unwrap()
        );
    }

    #[test]
    fn supports_supported_datetime_limits_without_overflow() {
        assert_eq!(start_of_year_datetime(NaiveDateTime::MIN), NaiveDateTime::MIN);
        assert_eq!(end_of_year_datetime(NaiveDateTime::MAX), NaiveDateTime::MAX);
        assert_eq!(end_of_month_datetime(NaiveDateTime::MAX), NaiveDateTime::MAX);
    }
}
