//! Week comparisons for naive local date-times.

use chrono::{NaiveDateTime, Weekday};

/// Compare week membership with a chosen first weekday.
///
/// Time of day is ignored. This works at Chrono's supported date limits.
pub fn is_same_week_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
    week_starts_on: Weekday,
) -> bool {
    crate::is_same_week(left.date(), right.date(), week_starts_on)
}

/// Compare Monday-first ISO week membership, ignoring clock time.
pub fn is_same_iso_week_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    crate::is_same_iso_week(left.date(), right.date())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn datetime(y: i32, m: u32, d: u32, h: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, 30, 0)
            .unwrap()
    }

    #[test]
    fn compares_monday_and_sunday_first_weeks() {
        let sunday = datetime(2023, 12, 31, 23);
        let monday = datetime(2024, 1, 1, 1);
        assert!(!is_same_week_datetime(sunday, monday, Weekday::Mon));
        assert!(is_same_week_datetime(sunday, monday, Weekday::Sun));
        assert!(!is_same_iso_week_datetime(sunday, monday));
    }

    #[test]
    fn iso_week_can_cross_calendar_years() {
        assert!(is_same_iso_week_datetime(
            datetime(2019, 12, 30, 0),
            datetime(2020, 1, 5, 23)
        ));
        assert!(!is_same_iso_week_datetime(
            datetime(2020, 1, 5, 23),
            datetime(2020, 1, 6, 0)
        ));
    }

    #[test]
    fn compares_supported_datetime_limits() {
        assert!(is_same_iso_week_datetime(
            NaiveDateTime::MIN,
            NaiveDateTime::MIN
        ));
        assert!(is_same_week_datetime(
            NaiveDateTime::MAX,
            NaiveDateTime::MAX,
            Weekday::Sun
        ));
    }
}
