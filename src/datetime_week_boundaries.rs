//! Calendar week boundaries for naive date-times with a chosen first weekday.

use chrono::{NaiveDateTime, Weekday};

/// Return midnight on the first day of the week containing value.
/// The beginning may be out of Chrono's supported date range.
pub fn start_of_week_datetime(
    value: NaiveDateTime,
    week_starts_on: Weekday,
) -> Option<NaiveDateTime> {
    crate::start_of_week(value.date(), week_starts_on)?.and_hms_opt(0, 0, 0)
}

/// Return the last representable nanosecond of the last day of the week.
/// The ending may be out of Chrono's supported date range.
pub fn end_of_week_datetime(
    value: NaiveDateTime,
    week_starts_on: Weekday,
) -> Option<NaiveDateTime> {
    crate::end_of_week(value.date(), week_starts_on)?
        .and_hms_nano_opt(23, 59, 59, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn crosses_calendar_months_with_explicit_first_weekday() {
        let value = NaiveDate::from_ymd_opt(2024, 1, 3)
            .unwrap()
            .and_hms_opt(12, 34, 56)
            .unwrap();
        let monday = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        assert_eq!(
            start_of_week_datetime(value, Weekday::Mon),
            monday.and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_week_datetime(value, Weekday::Mon),
            sunday.and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(
            start_of_week_datetime(value, Weekday::Sun),
            NaiveDate::from_ymd_opt(2023, 12, 31)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn ignores_original_time_and_preserves_week_end() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let morning = date.and_hms_opt(0, 0, 0).unwrap();
        let evening = date.and_hms_opt(23, 59, 59).unwrap();
        assert_eq!(
            start_of_week_datetime(morning, Weekday::Mon),
            start_of_week_datetime(evening, Weekday::Mon)
        );
        assert_eq!(
            end_of_week_datetime(morning, Weekday::Mon),
            end_of_week_datetime(evening, Weekday::Mon)
        );
    }
}
