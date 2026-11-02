//! Comma-separated UTC DATE-TIME values for iCalendar RDATE and EXDATE.

use chrono::{DateTime, Utc};

/// Parse a non-empty list of UTC basic DATE-TIME values.
///
/// Every value must have the mandatory Z suffix, with seconds precision.
pub fn parse_ical_utc_datetime_list(input: &str) -> Option<Vec<DateTime<Utc>>> {
    input
        .split(',')
        .map(crate::parse_ical_datetime_utc)
        .collect()
}

/// Format a non-empty list without silently discarding fractional seconds.
///
/// Original value order and duplicates are preserved.
pub fn format_ical_utc_datetime_list(values: &[DateTime<Utc>]) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    let parts: Vec<_> = values
        .iter()
        .map(|value| crate::format_ical_datetime_utc(value.to_owned()))
        .collect::<Option<Vec<_>>>()?;
    Some(parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_multiple_utc_occurrences() {
        let input = "20240229T235958Z,20240301T000000Z";
        let values = parse_ical_utc_datetime_list(input).unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(format_ical_utc_datetime_list(&values).as_deref(), Some(input));
    }

    #[test]
    fn rejects_floating_datetimes_bad_delimiters_and_fractional_output() {
        for input in [
            "", "20240229T235958Z,", "20240229T235958Z,,20240301T000000Z",
            "20240229T235958", "20240230T235958Z", "20240229T235958+0100",
        ] {
            assert!(parse_ical_utc_datetime_list(input).is_none(), "{input}");
        }
        assert_eq!(format_ical_utc_datetime_list(&[]), None);
        let subsecond = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 1)
            .unwrap()
            .and_utc();
        assert_eq!(format_ical_utc_datetime_list(&[subsecond]), None);
    }
}
