//! Comma-separated RFC 5545 UTC start/end PERIOD values.

use chrono::{DateTime, Utc};

/// Parse a nonempty comma-separated list of UTC start/end PERIOD values.
///
/// Periods preserve their original order. An empty or malformed item rejects
/// the entire list. Duration-form PERIODs are deliberately unsupported here.
pub fn parse_ical_period_list_utc(
    input: &str,
) -> Option<Vec<(DateTime<Utc>, DateTime<Utc>)>> {
    if input.is_empty() {
        return None;
    }
    input.split(',').map(crate::parse_ical_period_utc).collect()
}

/// Format a nonempty list of UTC start/end PERIODs without precision loss.
pub fn format_ical_period_list_utc(
    periods: &[(DateTime<Utc>, DateTime<Utc>)],
) -> Option<String> {
    if periods.is_empty() {
        return None;
    }
    periods
        .iter()
        .map(|(start, end)| crate::format_ical_period_utc(*start, *end))
        .collect::<Option<Vec<_>>>()
        .map(|parts| parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_multiple_periods_and_preserves_order() {
        let input = "20240301T080000Z/20240301T090000Z,20240229T120000Z/20240229T130000Z";
        let values = parse_ical_period_list_utc(input).unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(format_ical_period_list_utc(&values).as_deref(), Some(input));
    }

    #[test]
    fn rejects_empty_reversed_and_duration_period_entries() {
        for input in [
            "",
            ",20240301T080000Z/20240301T090000Z",
            "20240301T080000Z/20240301T090000Z,",
            "20240301T090000Z/20240301T080000Z",
            "20240301T080000Z/P1D",
            "20240301T080000Z/20240301T090000Z,INVALID",
        ] {
            assert_eq!(parse_ical_period_list_utc(input), None, "{input}");
        }
        assert_eq!(format_ical_period_list_utc(&[]), None);
    }
}
