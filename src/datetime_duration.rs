//! Calendar-aware decomposition of ordered naive datetime intervals.

use chrono::{NaiveDateTime, TimeDelta};

/// Nonnegative fields describing an ordered datetime interval.
///
/// Years and months form one combined month shift from the source date.
/// Remaining fields are fixed elapsed units. Apply the combined month shift
/// before the remaining duration to round-trip clamped month ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDateTimeDuration {
    pub years: i32,
    pub months: u32,
    pub days: i64,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub nanoseconds: u32,
}

/// Decompose an ordered interval using calendar months before elapsed units.
///
/// Returns None for reversed intervals or dates outside the supported range.
pub fn interval_to_calendar_duration(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<CalendarDateTimeDuration> {
    if end < start {
        return None;
    }
    let mut months = i32::try_from(
        crate::difference_in_calendar_months_datetime(end, start)
    ).ok()?;
    let mut anchor = crate::add_months_datetime(start, months)?;
    if anchor > end {
        months = months.checked_sub(1)?;
        anchor = crate::add_months_datetime(start, months)?;
    }

    let remaining = end.signed_duration_since(anchor);
    let days = remaining.num_days();
    let clock = remaining - TimeDelta::days(days);
    let seconds = clock.num_seconds();
    let nanos = (clock - TimeDelta::seconds(seconds)).num_nanoseconds()?;

    Some(CalendarDateTimeDuration {
        years: months / 12,
        months: (months % 12) as u32,
        days,
        hours: (seconds / 3600) as u32,
        minutes: ((seconds % 3600) / 60) as u32,
        seconds: (seconds % 60) as u32,
        nanoseconds: nanos as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn decomposes_leap_months_and_fractional_clocks() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 31)
            .unwrap()
            .and_hms_nano_opt(8, 15, 30, 123_000_000)
            .unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_nano_opt(10, 16, 32, 456_000_000)
            .unwrap();
        assert_eq!(
            interval_to_calendar_duration(start, end),
            Some(CalendarDateTimeDuration {
                years: 0,
                months: 1,
                days: 1,
                hours: 2,
                minutes: 1,
                seconds: 2,
                nanoseconds: 333_000_000,
            })
        );
    }

    #[test]
    fn handles_short_of_month_end_and_exact_years() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 31)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap();
        let duration = interval_to_calendar_duration(start, end).unwrap();
        assert_eq!(duration.months, 0);
        assert_eq!(duration.days, 28);
        assert_eq!(duration.hours, 23);

        let year_end = NaiveDate::from_ymd_opt(2025, 1, 31)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let year = interval_to_calendar_duration(start, year_end).unwrap();
        assert_eq!((year.years, year.months, year.days), (1, 0, 0));
    }

    #[test]
    fn handles_empty_reversed_and_maximum_ranges() {
        let start = NaiveDateTime::MIN;
        assert_eq!(
            interval_to_calendar_duration(start, start),
            Some(CalendarDateTimeDuration {
                years: 0,
                months: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                nanoseconds: 0,
            })
        );
        assert_eq!(interval_to_calendar_duration(NaiveDateTime::MAX, start), None);
        assert!(interval_to_calendar_duration(start, NaiveDateTime::MAX).is_some());
    }
}
