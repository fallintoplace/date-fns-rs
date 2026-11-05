//! Apply signed ISO calendar durations while preserving naive clock time.

use chrono::NaiveDateTime;

use crate::IsoCalendarDuration;

/// Apply a signed calendar duration to a naive datetime.
///
/// Months clamp to the final month's last day. The clock and nanosecond
/// component are preserved. Invalid durations and range overflow return None.
pub fn apply_iso_calendar_duration_datetime(
    start: NaiveDateTime,
    duration: IsoCalendarDuration,
) -> Option<NaiveDateTime> {
    let date = crate::apply_iso_calendar_duration(start.date(), duration)?;
    Some(date.and_time(start.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};

    #[test]
    fn retains_subseconds_when_clamping_leap_months() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 31)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_456_789)
            .unwrap();
        let duration = crate::parse_iso_calendar_duration("P1M").unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_456_789)
            .unwrap();
        assert_eq!(
            apply_iso_calendar_duration_datetime(start, duration),
            Some(expected)
        );
    }

    #[test]
    fn supports_negative_periods_and_range_limits() {
        let start = NaiveDate::from_ymd_opt(2024, 3, 31)
            .unwrap()
            .and_hms_opt(9, 45, 0)
            .unwrap();
        let duration = crate::parse_iso_calendar_duration("-P1M").unwrap();
        assert_eq!(
            apply_iso_calendar_duration_datetime(start, duration),
            Some(
                NaiveDate::from_ymd_opt(2024, 2, 29)
                    .unwrap()
                    .and_hms_opt(9, 45, 0)
                    .unwrap()
            )
        );
        let one_day = crate::parse_iso_calendar_duration("P1D").unwrap();
        assert_eq!(
            apply_iso_calendar_duration_datetime(NaiveDateTime::MAX, one_day),
            None
        );
    }

    #[test]
    fn rejects_mixed_week_and_month_fields() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let invalid = IsoCalendarDuration {
            negative: false,
            years: 0,
            months: 1,
            weeks: 1,
            days: 0,
        };
        assert_eq!(apply_iso_calendar_duration_datetime(start, invalid), None);
    }
}
