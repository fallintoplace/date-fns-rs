//! RFC 5545 DURATION values with integral seconds and no calendar months.

/// Parse an iCalendar DURATION as exact signed nanoseconds.
///
/// The RFC permits integer weeks, days, hours, minutes and seconds. A leading
/// positive sign is accepted; fractional seconds and months are rejected.
pub fn parse_ical_duration(input: &str) -> Option<i128> {
    if input.contains('.') {
        return None;
    }
    let text = input.strip_prefix('+').unwrap_or(input);
    crate::parse_iso_fixed_duration(text)
}

/// Format a whole-second duration as a canonical iCalendar DURATION.
///
/// Fractional nanoseconds cannot be represented by RFC 5545 DURATION.
pub fn format_ical_duration(nanoseconds: i128) -> Option<String> {
    if nanoseconds % 1_000_000_000 != 0 {
        return None;
    }
    Some(crate::format_iso_fixed_duration(nanoseconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_week_day_and_clock_durations() {
        assert_eq!(parse_ical_duration("P2W"), Some(1_209_600_000_000_000));
        assert_eq!(parse_ical_duration("+P1DT2H3M4S"), Some(93_784_000_000_000));
        assert_eq!(parse_ical_duration("-PT15M"), Some(-900_000_000_000));
        assert_eq!(format_ical_duration(0).as_deref(), Some("PT0S"));
        let value = parse_ical_duration("-P2DT3H4M5S").unwrap();
        assert_eq!(
            format_ical_duration(value).as_deref(),
            Some("-P2DT3H4M5S")
        );
    }

    #[test]
    fn rejects_fractional_and_calendar_dependent_fields() {
        for invalid in [
            "", "P", "P1Y", "P1M", "PT1.5S", "P1W2D", "PT", "PT1S ",
            "P-1D", "PT1M1H", "P1DT", "P1W1W",
        ] {
            assert_eq!(parse_ical_duration(invalid), None, "{invalid}");
        }
        assert_eq!(format_ical_duration(1), None);
        assert_eq!(format_ical_duration(-1), None);
    }
}
