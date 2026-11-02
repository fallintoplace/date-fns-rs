//! ISO ordinal-date local timestamps without implicit time zones.

use chrono::NaiveDateTime;

/// Parse YYYY-DDDTHH:MM:SS with optional 1-9 digit fractional seconds.
pub fn parse_iso_ordinal_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (date, time) = input.split_once('T')?;
    Some(crate::parse_iso_ordinal_date(date)?.and_time(crate::parse_iso_local_time(time)?))
}

/// Format a local datetime as an ISO ordinal date and clock.
pub fn format_iso_ordinal_local_datetime(value: NaiveDateTime) -> Option<String> {
    Some(format!(
        "{}T{}",
        crate::format_iso_ordinal_date(value.date())?,
        crate::format_iso_local_time(value.time())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_leap_day_and_nanosecond_precision() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let value = date.and_hms_nano_opt(12, 34, 56, 100_000_001).unwrap();
        let text = "2024-060T12:34:56.100000001";
        assert_eq!(parse_iso_ordinal_local_datetime(text), Some(value));
        assert_eq!(
            format_iso_ordinal_local_datetime(value).as_deref(),
            Some(text)
        );
        assert_eq!(
            parse_iso_ordinal_local_datetime("2023-060T00:00:00"),
            NaiveDate::from_ymd_opt(2023, 3, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_invalid_ordinals_offsets_and_precision_loss() {
        for value in [
            "2023-366T12:00:00",
            "2024-000T12:00:00",
            "2024-060T24:00:00",
            "2024-060T12:00:00Z",
            "2024-060T12:00:00+00:00",
            "2024-060T12:00:00.1234567890",
            "2024-060 12:00:00",
        ] {
            assert_eq!(parse_iso_ordinal_local_datetime(value), None, "{value}");
        }
        assert_eq!(format_iso_ordinal_local_datetime(NaiveDateTime::MIN), None);
    }
}
