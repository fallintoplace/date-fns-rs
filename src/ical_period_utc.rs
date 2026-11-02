//! RFC 5545 PERIOD values using explicit UTC start and end instants.

use chrono::{DateTime, Utc};

/// Parse a UTC start/end PERIOD of the form START/END.
///
/// A valid PERIOD must have a strictly later end. Duration-style PERIOD
/// values are deliberately not accepted by this start/end-specific helper.
pub fn parse_ical_period_utc(input: &str) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let (start, end) = input.split_once('/')?;
    let start = crate::parse_ical_datetime_utc(start)?;
    let end = crate::parse_ical_datetime_utc(end)?;
    (start < end).then_some((start, end))
}

/// Format an explicit UTC start/end PERIOD without precision loss.
pub fn format_ical_period_utc(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Option<String> {
    if start >= end {
        return None;
    }
    Some(format!(
        "{}/{}",
        crate::format_ical_datetime_utc(start)?,
        crate::format_ical_datetime_utc(end)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_valid_utc_periods() {
        let text = "20240229T120000Z/20240301T090000Z";
        let (start, end) = parse_ical_period_utc(text).unwrap();
        assert!(start < end);
        assert_eq!(format_ical_period_utc(start, end).as_deref(), Some(text));
    }

    #[test]
    fn rejects_reversed_local_and_ambiguous_periods() {
        for text in [
            "20240301T090000Z/20240229T120000Z",
            "20240229T120000Z/20240229T120000Z",
            "20240229T120000/20240301T090000",
            "20240229T120000Z/P1D",
            "20240229T120000Z/20240301T090000+0100",
            "20240229T120000Z/20240301T090000Z/extra",
        ] {
            assert_eq!(parse_ical_period_utc(text), None, "{text}");
        }
        let start = crate::parse_ical_datetime_utc("20240229T120000Z").unwrap();
        assert_eq!(format_ical_period_utc(start, start), None);
    }
}
