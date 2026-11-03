//! Human-readable English formatting for calendar datetime durations.
use crate::CalendarDateTimeDuration;

fn push_unit(parts: &mut Vec<String>, amount: i64, unit: &str, include_zero: bool) {
    if include_zero || amount != 0 {
        let plural = if amount == 1 { "" } else { "s" };
        parts.push(format!("{amount} {unit}{plural}"));
    }
}

/// Format a nonnegative duration as English units in descending order.
///
/// Zero units are omitted unless `include_zero` is true. The delimiter is
/// user-supplied, and fractional seconds retain nanosecond precision.
/// Returns None for negative years/days or invalid nanoseconds.
pub fn format_calendar_duration_en(
    value: CalendarDateTimeDuration,
    include_zero: bool,
    delimiter: &str,
) -> Option<String> {
    if value.years < 0 || value.days < 0 || value.nanoseconds >= 1_000_000_000 {
        return None;
    }

    let mut parts = Vec::new();
    push_unit(&mut parts, i64::from(value.years), "year", include_zero);
    push_unit(&mut parts, i64::from(value.months), "month", include_zero);
    push_unit(&mut parts, value.days, "day", include_zero);
    push_unit(&mut parts, i64::from(value.hours), "hour", include_zero);
    push_unit(&mut parts, i64::from(value.minutes), "minute", include_zero);

    if include_zero || value.seconds != 0 || value.nanoseconds != 0 {
        let seconds = if value.nanoseconds == 0 {
            value.seconds.to_string()
        } else {
            let fraction = format!("{:09}", value.nanoseconds);
            format!("{}.{}", value.seconds, fraction.trim_end_matches('0'))
        };
        let plural = if value.seconds == 1 && value.nanoseconds == 0 {
            ""
        } else {
            "s"
        };
        parts.push(format!("{seconds} second{plural}"));
    }

    Some(parts.join(delimiter))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn formats_nonzero_units_and_configurable_delimiters() {
        let mut duration = empty();
        duration.years = 1;
        duration.months = 2;
        duration.days = 3;
        assert_eq!(
            format_calendar_duration_en(duration, false, " ").as_deref(),
            Some("1 year 2 months 3 days")
        );
        assert_eq!(
            format_calendar_duration_en(duration, false, ", ").as_deref(),
            Some("1 year, 2 months, 3 days")
        );
    }

    #[test]
    fn supports_empty_zero_and_fractional_durations() {
        assert_eq!(
            format_calendar_duration_en(empty(), false, ""),
            Some(String::new())
        );
        let mut duration = empty();
        duration.seconds = 1;
        assert_eq!(
            format_calendar_duration_en(duration, false, " ").as_deref(),
            Some("1 second")
        );
        duration.nanoseconds = 250_000_000;
        assert_eq!(
            format_calendar_duration_en(duration, false, " ").as_deref(),
            Some("1.25 seconds")
        );
        assert!(format_calendar_duration_en(empty(), true, " ")
            .unwrap()
            .contains("0 years"));
    }

    #[test]
    fn rejects_invalid_durations() {
        let mut duration = empty();
        duration.days = -1;
        assert_eq!(format_calendar_duration_en(duration, false, " "), None);
        duration.days = 0;
        duration.nanoseconds = 1_000_000_000;
        assert_eq!(format_calendar_duration_en(duration, false, " "), None);
    }
}
