//! RFC 5545 PERIOD values using an explicit UTC start and positive duration.

use chrono::{DateTime, TimeDelta, Utc};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

fn checked_end(start: DateTime<Utc>, nanoseconds: i128) -> Option<DateTime<Utc>> {
    if nanoseconds <= 0 || nanoseconds % NANOS_PER_SECOND != 0 {
        return None;
    }
    let seconds = i64::try_from(nanoseconds / NANOS_PER_SECOND).ok()?;
    start.checked_add_signed(TimeDelta::try_seconds(seconds)?)
}

/// Parse a UTC PERIOD in START/DURATION form.
///
/// The duration must be strictly positive and fit within Chrono's date range.
/// This complements the separate start/end PERIOD parser.
pub fn parse_ical_period_duration_utc(input: &str) -> Option<(DateTime<Utc>, i128)> {
    let (start, duration) = input.split_once('/')?;
    let start = crate::parse_ical_datetime_utc(start)?;
    let duration = crate::parse_ical_duration(duration)?;
    checked_end(start, duration)?;
    Some((start, duration))
}

/// Format a UTC start/duration PERIOD without truncating fractional seconds.
pub fn format_ical_period_duration_utc(
    start: DateTime<Utc>,
    nanoseconds: i128,
) -> Option<String> {
    checked_end(start, nanoseconds)?;
    Some(format!(
        "{}/{}",
        crate::format_ical_datetime_utc(start)?,
        crate::format_ical_duration(nanoseconds)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_positive_utc_periods_across_leap_days() {
        let text = "20240229T120000Z/P1DT2H";
        let (start, duration) = parse_ical_period_duration_utc(text).unwrap();
        assert_eq!(duration, 93_600_000_000_000);
        assert_eq!(format_ical_period_duration_utc(start, duration).as_deref(), Some(text));
    }

    #[test]
    fn rejects_zero_negative_local_and_invalid_durations() {
        for input in [
            "20240229T120000Z/P0D",
            "20240229T120000Z/-PT1S",
            "20240229T120000Z/PT1.5S",
            "20240229T120000/P1D",
            "20240229T120000Z/20240301T120000Z",
            "20240229T120000Z/P1D/EXTRA",
        ] {
            assert_eq!(parse_ical_period_duration_utc(input), None, "{input}");
        }
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_utc();
        assert_eq!(format_ical_period_duration_utc(start, 0), None);
        assert_eq!(format_ical_period_duration_utc(start, 1), None);
        assert_eq!(format_ical_period_duration_utc(start, -1_000_000_000), None);
    }

    #[test]
    fn rejects_dates_past_chronos_supported_range() {
        let end = chrono::NaiveDateTime::MAX.and_utc();
        assert_eq!(format_ical_period_duration_utc(end, 1_000_000_000), None);
    }
}
