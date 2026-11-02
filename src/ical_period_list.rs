//! RFC 5545 comma-separated UTC PERIOD values for RDATE properties.

use chrono::{DateTime, Utc};

/// Parse a non-empty list of UTC start/end PERIOD values.
///
/// Preserves input order and duplicates. Each end must be later than its start.
pub fn parse_ical_period_list(input: &str) -> Option<Vec<(DateTime<Utc>, DateTime<Utc>)>> {
    if input.is_empty() {
        return None;
    }
    input.split(',').map(crate::parse_ical_period_utc).collect()
}

/// Format a non-empty list of UTC start/end PERIOD values.
///
/// Invalid periods or fractional timestamps return None.
pub fn format_ical_period_list(periods: &[(DateTime<Utc>, DateTime<Utc>)]) -> Option<String> {
    if periods.is_empty() {
        return None;
    }
    let values: Vec<_> = periods
        .iter()
        .map(|(start, end)| crate::format_ical_period_utc(*start, *end))
        .collect::<Option<Vec<_>>>()?;
    Some(values.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_lists_and_duplicates() {
        let first = "20240229T120000Z/20240229T130000Z";
        let second = "20240301T090000Z/20240301T100000Z";
        let input = format!("{first},{second},{first}");
        let parsed = parse_ical_period_list(&input).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0], parsed[2]);
        assert_eq!(format_ical_period_list(&parsed), Some(input));
    }

    #[test]
    fn rejects_empty_invalid_and_fractional_periods() {
        for input in [
            "",
            "20240229T120000Z/20240229T130000Z,",
            ",20240229T120000Z/20240229T130000Z",
            "20240229T130000Z/20240229T120000Z",
            "20240229T120000Z/PT1H",
        ] {
            assert_eq!(parse_ical_period_list(input), None);
        }
        assert_eq!(format_ical_period_list(&[]), None);
    }
}
