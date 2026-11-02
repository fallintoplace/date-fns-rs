//! RFC 5545 basic DATE and UTC DATE-TIME values for iCalendar files.

use chrono::{DateTime, NaiveDate, Timelike, Utc};

/// Parse an iCalendar DATE value in exactly YYYYMMDD form.
pub fn parse_ical_date(input: &str) -> Option<NaiveDate> {
    crate::parse_iso_basic_date(input)
}

/// Format a calendar date as a strict iCalendar DATE value.
pub fn format_ical_date(date: NaiveDate) -> Option<String> {
    crate::format_iso_basic_date(date)
}

/// Parse a UTC DATE-TIME as YYYYMMDDTHHMMSSZ.
///
/// The Z suffix is mandatory. Floating local times, offset strings,
/// fractional seconds, and time-zone identifiers are intentionally rejected.
pub fn parse_ical_datetime_utc(input: &str) -> Option<DateTime<Utc>> {
    let local = input.strip_suffix('Z')?;
    if local.contains('.') {
        return None;
    }
    Some(crate::parse_iso_basic_local_datetime(local)?.and_utc())
}

/// Format a UTC DATE-TIME without fractional seconds.
///
/// Returns None for timestamps with subsecond values or years outside
/// 0000-9999 so serialization cannot silently lose information.
pub fn format_ical_datetime_utc(value: DateTime<Utc>) -> Option<String> {
    if value.nanosecond() != 0 {
        return None;
    }
    Some(format!(
        "{}Z",
        crate::format_iso_basic_local_datetime(value.naive_utc())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_icalendar_leap_day_dates_and_utc_datetimes() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(parse_ical_date("20240229"), Some(day));
        assert_eq!(format_ical_date(day).as_deref(), Some("20240229"));
        let utc = day.and_hms_opt(23, 59, 58).unwrap().and_utc();
        assert_eq!(parse_ical_datetime_utc("20240229T235958Z"), Some(utc));
        assert_eq!(
            format_ical_datetime_utc(utc).as_deref(),
            Some("20240229T235958Z")
        );
        assert_eq!(
            format_ical_datetime_utc(
                day.and_hms_opt(0, 0, 0).unwrap().and_utc()
            )
            .as_deref(),
            Some("20240229T000000Z")
        );
    }

    #[test]
    fn rejects_floating_times_offsets_fractional_seconds_and_invalid_dates() {
        for value in [
            "20240229T235959",
            "20240229T235959+0100",
            "20240229T235959.1Z",
            "20240229T235960Z",
            "20240230T000000Z",
            "2024-02-29T00:00:00Z",
            "20240229T240000Z",
            "20240229T235959z",
            "20240229T235959Z ",
            "20240229T235959ZZ",
        ] {
            assert_eq!(parse_ical_datetime_utc(value), None, "{value}");
        }
        let fractional = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 1)
            .unwrap()
            .and_utc();
        assert_eq!(format_ical_datetime_utc(fractional), None);
        assert_eq!(format_ical_date(NaiveDate::MAX), None);
    }
}
