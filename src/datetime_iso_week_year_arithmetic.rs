//! Checked ISO week-numbering year arithmetic on naive datetimes.

use chrono::NaiveDateTime;

/// Shift an ISO week-numbering year without changing week, weekday or clock.
///
/// Returns None if the target year lacks the source ISO week, including
/// week 53, or if the resulting date lies outside Chrono's range.
pub fn add_iso_week_years_datetime(value: NaiveDateTime, years: i32) -> Option<NaiveDateTime> {
    Some(crate::add_iso_week_years(value.date(), years)?.and_time(value.time()))
}

/// Subtract ISO week-numbering years while preserving nanosecond precision.
pub fn sub_iso_week_years_datetime(value: NaiveDateTime, years: i32) -> Option<NaiveDateTime> {
    Some(crate::sub_iso_week_years(value.date(), years)?.and_time(value.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(23, 45, 56, 123_456_789)
            .unwrap()
    }

    #[test]
    fn preserves_clock_and_weekday_across_iso_year_boundaries() {
        assert_eq!(
            add_iso_week_years_datetime(at(2024, 1, 1), 1),
            Some(at(2024, 12, 30))
        );
        assert_eq!(
            sub_iso_week_years_datetime(at(2024, 1, 1), 1),
            Some(at(2023, 1, 2))
        );
        assert_eq!(
            add_iso_week_years_datetime(at(2024, 1, 1), 0),
            Some(at(2024, 1, 1))
        );
    }

    #[test]
    fn rejects_missing_week_53_and_overflow() {
        assert_eq!(add_iso_week_years_datetime(at(2021, 1, 1), 1), None);
        assert_eq!(
            sub_iso_week_years_datetime(at(2021, 1, 1), 5),
            Some(at(2016, 1, 1))
        );
        assert_eq!(add_iso_week_years_datetime(at(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_iso_week_years_datetime(at(2024, 1, 1), i32::MIN), None);
    }
}
