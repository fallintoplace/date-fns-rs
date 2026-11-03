//! Typed RFC 5545 DATE and UTC DATE-TIME values.

use chrono::{DateTime, NaiveDate, Utc};

/// A calendar day or an explicit UTC instant from iCalendar data.
///
/// Floating local DATE-TIME values and time-zone identifiers are not inferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IcalTemporalValue {
    Date(NaiveDate),
    UtcDateTime(DateTime<Utc>),
}

/// Parse an iCalendar DATE or UTC DATE-TIME, preserving the value kind.
pub fn parse_ical_temporal_value(input: &str) -> Option<IcalTemporalValue> {
    if input.ends_with('Z') {
        Some(IcalTemporalValue::UtcDateTime(
            crate::parse_ical_datetime_utc(input)?,
        ))
    } else {
        Some(IcalTemporalValue::Date(crate::parse_ical_date(input)?))
    }
}

/// Format a typed iCalendar value without losing timestamp precision.
pub fn format_ical_temporal_value(value: &IcalTemporalValue) -> Option<String> {
    match value {
        IcalTemporalValue::Date(date) => crate::format_ical_date(*date),
        IcalTemporalValue::UtcDateTime(value) => crate::format_ical_datetime_utc(*value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn preserves_date_and_utc_datetime_distinctions() {
        for text in ["20240229", "20240229T230000Z"] {
            let parsed = parse_ical_temporal_value(text).unwrap();
            assert_eq!(format_ical_temporal_value(&parsed).as_deref(), Some(text));
        }
        assert!(matches!(
            parse_ical_temporal_value("20240229"),
            Some(IcalTemporalValue::Date(_))
        ));
        assert!(matches!(
            parse_ical_temporal_value("20240229T230000Z"),
            Some(IcalTemporalValue::UtcDateTime(_))
        ));
    }

    #[test]
    fn rejects_invalid_local_offsets_and_fractional_values() {
        for input in [
            "20240230",
            "20240229T230000",
            "20240229T230000+0100",
            "20240229T230000.1Z",
            "2024-02-29",
        ] {
            assert_eq!(parse_ical_temporal_value(input), None, "{input}");
        }
        let fractional = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_milli_opt(23, 0, 0, 1)
            .unwrap()
            .and_utc();
        assert_eq!(
            format_ical_temporal_value(&IcalTemporalValue::UtcDateTime(fractional)),
            None
        );
    }
}
