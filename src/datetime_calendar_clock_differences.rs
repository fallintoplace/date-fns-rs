//! Calendar clock-unit boundaries of naive datetimes.

use chrono::NaiveDateTime;

/// Count signed hour boundaries between two naive datetimes.
///
/// Unlike whole elapsed hours, 12:59 -> 13:00 crosses one hour boundary.
pub fn difference_in_calendar_hours_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> i64 {
    left.and_utc().timestamp().div_euclid(3_600)
        - right.and_utc().timestamp().div_euclid(3_600)
}

/// Count signed minute boundaries between two naive datetimes.
pub fn difference_in_calendar_minutes_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> i64 {
    left.and_utc().timestamp().div_euclid(60)
        - right.and_utc().timestamp().div_euclid(60)
}

/// Count signed second boundaries between two naive datetimes.
pub fn difference_in_calendar_seconds_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> i64 {
    left.and_utc().timestamp() - right.and_utc().timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn counts_calendar_boundaries_without_elapsed_duration_truncation() {
        let before = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_milli_opt(23, 59, 59, 900)
            .unwrap();
        let after = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 0, 100)
            .unwrap();
        assert_eq!(difference_in_calendar_hours_datetime(after, before), 1);
        assert_eq!(difference_in_calendar_minutes_datetime(after, before), 1);
        assert_eq!(difference_in_calendar_seconds_datetime(after, before), 1);
        assert_eq!(difference_in_calendar_hours_datetime(before, after), -1);
        assert_eq!(difference_in_calendar_minutes_datetime(before, after), -1);
    }

    #[test]
    fn uses_euclidean_clock_buckets_before_unix_epoch() {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let before = NaiveDate::from_ymd_opt(1969, 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap();
        assert_eq!(difference_in_calendar_hours_datetime(epoch, before), 1);
        assert_eq!(difference_in_calendar_minutes_datetime(epoch, before), 1);
        assert_eq!(difference_in_calendar_seconds_datetime(epoch, before), 1);
    }

    #[test]
    fn handles_equal_and_extreme_naive_datetimes() {
        assert_eq!(
            difference_in_calendar_seconds_datetime(NaiveDateTime::MAX, NaiveDateTime::MAX),
            0
        );
        assert!(difference_in_calendar_hours_datetime(
            NaiveDateTime::MAX,
            NaiveDateTime::MIN
        ) > 0);
    }
}
