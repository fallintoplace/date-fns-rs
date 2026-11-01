//! Strict basic ISO 8601 local date-times without offsets.

use chrono::{NaiveDateTime, Timelike};

/// Parse YYYYMMDDTHHMMSS with an optional 1-9 digit fractional part.
/// Dates, times, offsets and trailing input are strictly validated.
pub fn parse_iso_basic_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (date, time) = input.split_once('T')?;
    let date = crate::parse_iso_basic_date(date)?;
    let (clock, fractional) = match time.split_once('.') {
        Some((clock, fractional)) => (clock, Some(fractional)),
        None => (time, None),
    };
    let bytes = clock.as_bytes();
    if bytes.len() != 6 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let nanos = match fractional {
        Some(digits)
            if !digits.is_empty()
                && digits.len() <= 9
                && digits.as_bytes().iter().all(u8::is_ascii_digit) =>
        {
            let mut value = digits.parse::<u32>().ok()?;
            for _ in digits.len()..9 {
                value *= 10;
            }
            value
        }
        Some(_) => return None,
        None => 0,
    };
    date.and_hms_nano_opt(
        clock[..2].parse().ok()?,
        clock[2..4].parse().ok()?,
        clock[4..6].parse().ok()?,
        nanos,
    )
}

/// Format YYYYMMDDTHHMMSS, preserving nonzero fractional nanoseconds.
///
/// Returns None for expanded years and leap-second representations.
pub fn format_iso_basic_local_datetime(value: NaiveDateTime) -> Option<String> {
    let date = crate::format_iso_basic_date(value.date())?;
    let nanos = value.nanosecond();
    if nanos >= 1_000_000_000 {
        return None;
    }
    let suffix = if nanos == 0 {
        String::new()
    } else {
        let mut digits = format!("{nanos:09}");
        while digits.ends_with('0') {
            digits.pop();
        }
        format!(".{digits}")
    };
    Some(format!(
        "{date}T{:02}{:02}{:02}{suffix}",
        value.hour(),
        value.minute(),
        value.second()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_leap_days_and_precise_times() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_000_009)
            .unwrap();
        let text = "20240229T235958.123000009";
        assert_eq!(parse_iso_basic_local_datetime(text), Some(value));
        assert_eq!(
            format_iso_basic_local_datetime(value).as_deref(),
            Some(text)
        );
        assert_eq!(
            parse_iso_basic_local_datetime("20240229T000000"),
            Some(value.date().and_hms_opt(0, 0, 0).unwrap())
        );
    }

    #[test]
    fn rejects_bad_dates_offsets_and_noncanonical_clocks() {
        for text in [
            "20230229T000000",
            "20240229T240000",
            "20240229T235960",
            "2024-02-29T235959",
            "20240229T23:59:59",
            "20240229t235959",
            "20240229T235959Z",
            "20240229T235959.",
            "20240229T235959.1234567890",
            "20240229T235959 ",
        ] {
            assert_eq!(parse_iso_basic_local_datetime(text), None, "{text}");
        }
        assert_eq!(format_iso_basic_local_datetime(NaiveDateTime::MAX), None);
    }

    #[test]
    fn round_trips_leap_days_and_fractional_seconds() {
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_450_000)
            .unwrap();
        assert_eq!(
            format_iso_basic_local_datetime(leap).as_deref(),
            Some("20240229T235958.12345")
        );
        assert_eq!(
            parse_iso_basic_local_datetime("20240229T235958.12345"),
            Some(leap)
        );
        let midnight = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            format_iso_basic_local_datetime(midnight).as_deref(),
            Some("20240301T000000")
        );
        assert_eq!(
            parse_iso_basic_local_datetime("20240301T000000"),
            Some(midnight)
        );
    }

    #[test]
    fn rejects_invalid_and_noncanonical_values() {
        for input in [
            "20230229T235958",
            "20240229T240000",
            "20240229T235960",
            "20240229T2359",
            "20240229T235958.",
            "20240229T235958.1234567890",
            "20240229T235958Z",
            "2024-02-29T235958",
            "20240229t235958",
            "20240229T235958 ",
        ] {
            assert_eq!(parse_iso_basic_local_datetime(input), None, "{input}");
        }
        assert_eq!(format_iso_basic_local_datetime(NaiveDateTime::MAX), None);
    }
}
