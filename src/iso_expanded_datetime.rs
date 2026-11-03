//! ISO 8601 local date-times with optional signed six-digit expanded years.

use chrono::NaiveDateTime;

/// Parse a four-digit or signed expanded-year ISO local date-time.
///
/// The date is proleptic Gregorian and the time has no implicit offset.
/// Fractional seconds can use one through nine decimal digits.
pub fn parse_iso_local_datetime_extended(input: &str) -> Option<NaiveDateTime> {
    let (date, time) = input.split_once('T')?;
    Some(NaiveDateTime::new(
        crate::parse_iso_date_extended(date)?,
        crate::parse_iso_local_time(time)?,
    ))
}

/// Format an ISO local date-time using signed years outside 0000..=9999.
pub fn format_iso_local_datetime_extended(value: NaiveDateTime) -> Option<String> {
    Some(format!(
        "{}T{}",
        crate::format_iso_date_extended(value.date()),
        crate::format_iso_local_time(value.time())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_negative_and_expanded_years() {
        for text in [
            "-000001-03-01T05:06:07.12",
            "+010000-02-29T00:00:00",
            "2024-02-29T23:59:59.123456789",
        ] {
            let value = parse_iso_local_datetime_extended(text).unwrap();
            assert_eq!(
                format_iso_local_datetime_extended(value).as_deref(),
                Some(text)
            );
        }
    }

    #[test]
    fn round_trips_chrono_datetime_limits() {
        for value in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            let text = format_iso_local_datetime_extended(value).unwrap();
            assert_eq!(parse_iso_local_datetime_extended(&text), Some(value));
        }
    }

    #[test]
    fn rejects_noncanonical_years_offsets_and_impossible_times() {
        for text in [
            "10000-01-01T00:00:00",
            "+10000-01-01T00:00:00",
            "-000001-03-01T24:00:00",
            "+010000-02-30T12:00:00",
            "2024-02-29T12:00:00Z",
            "2024-02-29 12:00:00",
        ] {
            assert_eq!(parse_iso_local_datetime_extended(text), None, "{text}");
        }
    }
}
