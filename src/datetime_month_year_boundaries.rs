//! Full-day month and year boundaries for naive calendar date-times.

use chrono::NaiveDateTime;

/// Return midnight at the beginning of the calendar month.
pub fn start_of_month_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::start_of_month(value.date()).and_hms_opt(0, 0, 0)
}

/// Return the final nanosecond of the calendar month.
pub fn end_of_month_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::end_of_month(value.date()).and_hms_nano_opt(23, 59, 59, 999_999_999)
}

/// Return midnight on January 1 of the input year.
pub fn start_of_year_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::start_of_year(value.date()).and_hms_opt(0, 0, 0)
}

/// Return the final nanosecond of December 31 of the input year.
pub fn end_of_year_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::end_of_year(value.date()).and_hms_nano_opt(23, 59, 59, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn handles_leap_february_and_year_end() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let value = date.and_hms_milli_opt(12, 34, 56, 789).unwrap();
        assert_eq!(
            start_of_month_datetime(value),
            NaiveDate::from_ymd_opt(2024, 2, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_month_datetime(value),
            date.and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(
            start_of_year_datetime(value),
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_year_datetime(value),
            NaiveDate::from_ymd_opt(2024, 12, 31)
                .unwrap()
                .and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
    }

    #[test]
    fn non_leap_february_ends_on_day_28() {
        let value = NaiveDate::from_ymd_opt(2023, 2, 14)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        assert_eq!(
            end_of_month_datetime(value),
            NaiveDate::from_ymd_opt(2023, 2, 28)
                .unwrap()
                .and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
    }
}
