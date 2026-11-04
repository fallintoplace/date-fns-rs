//! Apply a decomposed calendar and clock duration to a naive datetime.

use chrono::{NaiveDateTime, TimeDelta};

use crate::CalendarDateTimeDuration;

/// Apply a canonical nonnegative calendar duration to a naive datetime.
///
/// Years and months are combined before the month-clamping shift. Elapsed
/// days and clock components are applied afterwards. Invalid components or
/// out-of-range results return None.
pub fn apply_calendar_datetime_duration(
    start: NaiveDateTime,
    duration: CalendarDateTimeDuration,
) -> Option<NaiveDateTime> {
    if duration.years < 0
        || duration.months >= 12
        || duration.days < 0
        || duration.hours >= 24
        || duration.minutes >= 60
        || duration.seconds >= 60
        || duration.nanoseconds >= 1_000_000_000
    {
        return None;
    }

    let months = duration
        .years
        .checked_mul(12)?
        .checked_add(i32::try_from(duration.months).ok()?)?;
    let value = crate::add_months_datetime(start, months)?;
    let value = value.checked_add_signed(TimeDelta::try_days(duration.days)?)?;
    let value = value.checked_add_signed(TimeDelta::hours(i64::from(duration.hours)))?;
    let value = value.checked_add_signed(TimeDelta::minutes(i64::from(duration.minutes)))?;
    let value = value.checked_add_signed(TimeDelta::seconds(i64::from(duration.seconds)))?;
    value.checked_add_signed(TimeDelta::nanoseconds(i64::from(duration.nanoseconds)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn empty() -> CalendarDateTimeDuration {
        CalendarDateTimeDuration {
            years: 0,
            months: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            nanoseconds: 0,
        }
    }

    #[test]
    fn round_trips_decomposed_calendar_intervals() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 31)
            .unwrap()
            .and_hms_nano_opt(8, 15, 30, 123_000_000)
            .unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_nano_opt(10, 16, 32, 456_000_000)
            .unwrap();
        let duration = crate::interval_to_calendar_duration(start, end).unwrap();
        assert_eq!(apply_calendar_datetime_duration(start, duration), Some(end));

        let mut one_month = empty();
        one_month.months = 1;
        let leap_end = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(8, 15, 30, 123_000_000)
            .unwrap();
        assert_eq!(apply_calendar_datetime_duration(start, one_month), Some(leap_end));
    }

    #[test]
    fn rejects_invalid_fields_and_checked_overflow() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let mut value = empty();
        assert_eq!(apply_calendar_datetime_duration(start, value), Some(start));
        value.months = 12;
        assert_eq!(apply_calendar_datetime_duration(start, value), None);
        value.months = 0;
        value.nanoseconds = 1_000_000_000;
        assert_eq!(apply_calendar_datetime_duration(start, value), None);
        value.nanoseconds = 0;
        value.years = -1;
        assert_eq!(apply_calendar_datetime_duration(start, value), None);
        value.years = 0;
        value.days = 1;
        assert_eq!(
            apply_calendar_datetime_duration(NaiveDateTime::MAX, value),
            None
        );
    }
}
