//! Inclusive calendar half-year boundaries for naive date-times.

use chrono::NaiveDateTime;

/// Midnight on January 1 or July 1 of the containing half-year.
pub fn start_of_half_year_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::start_of_half_year(value.date()).and_hms_opt(0, 0, 0)
}

/// Last nanosecond of June 30 or December 31 of the containing half-year.
pub fn end_of_half_year_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::end_of_half_year(value.date()).and_hms_nano_opt(23, 59, 59, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn spans_both_calendar_halves_at_nanosecond_precision() {
        let first = date(2024, 2, 29).and_hms_opt(12, 45, 0).unwrap();
        assert_eq!(
            start_of_half_year_datetime(first),
            date(2024, 1, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_half_year_datetime(first),
            date(2024, 6, 30).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        let second = date(2024, 7, 1).and_hms_opt(12, 0, 0).unwrap();
        assert_eq!(
            start_of_half_year_datetime(second),
            date(2024, 7, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_half_year_datetime(second),
            date(2024, 12, 31).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
    }

    #[test]
    fn includes_chrono_datetime_boundaries() {
        assert_eq!(
            start_of_half_year_datetime(NaiveDateTime::MIN),
            Some(NaiveDateTime::MIN)
        );
        assert_eq!(
            end_of_half_year_datetime(NaiveDateTime::MAX),
            Some(NaiveDateTime::MAX)
        );
    }
}
