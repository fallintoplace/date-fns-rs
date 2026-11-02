//! ISO week-date local timestamps without implicit time zones.

use chrono::NaiveDateTime;

/// Parse YYYY-Www-dTHH:MM:SS with optional 1-9 digit fractional seconds.
pub fn parse_iso_week_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (date, time) = input.split_once('T')?;
    Some(crate::parse_iso_week_date(date)?.and_time(crate::parse_iso_local_time(time)?))
}

/// Format a local datetime using its ISO week-numbering year.
///
/// Year and ISO week may differ from the Gregorian calendar year.
pub fn format_iso_week_local_datetime(value: NaiveDateTime) -> Option<String> {
    Some(format!(
        "{}T{}",
        crate::format_iso_week_date(value.date())?,
        crate::format_iso_local_time(value.time())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_iso_week_year_boundary_and_fractional_seconds() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let value = date.and_hms_nano_opt(23, 59, 58, 123_456_789).unwrap();
        let input = "2020-W53-5T23:59:58.123456789";
        assert_eq!(format_iso_week_local_datetime(value).as_deref(), Some(input));
        assert_eq!(parse_iso_week_local_datetime(input), Some(value));
        assert_eq!(
            parse_iso_week_local_datetime("2019-W01-1T00:00:00"),
            NaiveDate::from_ymd_opt(2018, 12, 31)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_invalid_week_dates_zones_and_noncanonical_clocks() {
        for value in [
            "2021-W53-5T00:00:00",
            "2020-W53-5T24:00:00",
            "2020-W53-5T00:00:00Z",
            "2020-W53-5T00:00:00+01:00",
            "2020-W53-5T00:00:00.1234567890",
            "2020-W53-5 00:00:00",
        ] {
            assert_eq!(parse_iso_week_local_datetime(value), None, "{value}");
        }
        assert_eq!(format_iso_week_local_datetime(NaiveDateTime::MAX), None);
    }
}
