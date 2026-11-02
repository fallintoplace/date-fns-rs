//! Comma-separated floating iCalendar DATE-TIME values for RDATE and EXDATE.

use chrono::NaiveDateTime;

/// Parse a non-empty list of floating iCalendar local DATE-TIME values.
///
/// No timezone is assumed. Input order and duplicate occurrences are kept.
pub fn parse_ical_floating_datetime_list(input: &str) -> Option<Vec<NaiveDateTime>> {
    if input.is_empty() {
        return None;
    }
    input
        .split(',')
        .map(crate::parse_ical_datetime_floating)
        .collect()
}

/// Format floating iCalendar DATE-TIME values without precision loss.
///
/// Rejects empty lists and any subsecond timestamps.
pub fn format_ical_floating_datetime_list(values: &[NaiveDateTime]) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    let parts: Vec<_> = values
        .iter()
        .copied()
        .map(crate::format_ical_datetime_floating)
        .collect::<Option<Vec<_>>>()?;
    Some(parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_multiple_floating_times_and_duplicates() {
        let input = "20240229T235958,20240301T000000,20240229T235958";
        let values = parse_ical_floating_datetime_list(input).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], values[2]);
        assert_eq!(
            format_ical_floating_datetime_list(&values).as_deref(),
            Some(input)
        );
    }

    #[test]
    fn rejects_invalid_lists_offsets_and_fractional_output() {
        for text in [
            "",
            "20240229T235958,",
            ",20240229T235958",
            "20240229T235958,,20240301T000000",
            "20240229T235958Z",
            "20240229T235958+0100",
        ] {
            assert!(parse_ical_floating_datetime_list(text).is_none());
        }
        assert_eq!(format_ical_floating_datetime_list(&[]), None);
        let fraction = chrono::NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 1)
            .unwrap();
        assert_eq!(format_ical_floating_datetime_list(&[fraction]), None);
    }
}
