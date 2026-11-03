//! ISO 8601 formatting of calendar-aware datetime durations.
use crate::CalendarDateTimeDuration;

/// Format a nonnegative duration, including zero fields, as an ISO 8601 string.
///
/// Matches date-fns formatISODuration for whole units, and permits fractional
/// seconds at nanosecond precision. Invalid negative fields return None.
pub fn format_iso_duration(value: CalendarDateTimeDuration) -> Option<String> {
    if value.years < 0 || value.days < 0 || value.nanoseconds >= 1_000_000_000 {
        return None;
    }
    let seconds = if value.nanoseconds == 0 {
        value.seconds.to_string()
    } else {
        let fraction = format!("{:09}", value.nanoseconds);
        format!("{}.{}", value.seconds, fraction.trim_end_matches('0'))
    };
    Some(format!(
        "P{}Y{}M{}DT{}H{}M{}S",
        value.years, value.months, value.days, value.hours, value.minutes, seconds
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty() -> CalendarDateTimeDuration {
        CalendarDateTimeDuration {
            years: 0, months: 0, days: 0, hours: 0, minutes: 0,
            seconds: 0, nanoseconds: 0,
        }
    }

    #[test]
    fn matches_date_fns_whole_duration_formatting() {
        assert_eq!(format_iso_duration(empty()).as_deref(), Some("P0Y0M0DT0H0M0S"));
        let duration = CalendarDateTimeDuration {
            years: 39, months: 2, days: 20, hours: 7, minutes: 5,
            seconds: 0, nanoseconds: 0,
        };
        assert_eq!(
            format_iso_duration(duration).as_deref(),
            Some("P39Y2M20DT7H5M0S")
        );
    }

    #[test]
    fn preserves_fractional_seconds_without_trailing_zeros() {
        let mut duration = empty();
        duration.seconds = 7;
        duration.nanoseconds = 123_000_000;
        assert_eq!(
            format_iso_duration(duration).as_deref(),
            Some("P0Y0M0DT0H0M7.123S")
        );
        duration.nanoseconds = 1;
        assert_eq!(
            format_iso_duration(duration).as_deref(),
            Some("P0Y0M0DT0H0M7.000000001S")
        );
    }

    #[test]
    fn rejects_invalid_negative_and_subsecond_fields() {
        let mut duration = empty();
        duration.years = -1;
        assert_eq!(format_iso_duration(duration), None);
        duration.years = 0;
        duration.days = -1;
        assert_eq!(format_iso_duration(duration), None);
        duration.days = 0;
        duration.nanoseconds = 1_000_000_000;
        assert_eq!(format_iso_duration(duration), None);
    }
}
