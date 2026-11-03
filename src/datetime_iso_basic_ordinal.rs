//! ISO basic ordinal local date-times without implicit time zones.

use chrono::NaiveDateTime;

/// Parse a basic ordinal-date datetime such as 2024060T235958.123.
///
/// Accepts fractional seconds up to nanosecond precision, but no zone offsets.
pub fn parse_iso_basic_ordinal_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (day, time) = input.split_once('T')?;
    let date = crate::parse_iso_basic_ordinal_date(day)?;
    let clock = crate::parse_iso_basic_local_time(time)?;
    Some(date.and_time(clock))
}

/// Format a local datetime as YYYYDDDTHHMMSS with optional nanoseconds.
pub fn format_iso_basic_ordinal_local_datetime(value: NaiveDateTime) -> Option<String> {
    Some(format!(
        "{}T{}",
        crate::format_iso_basic_ordinal_date(value.date())?,
        crate::format_iso_basic_local_time(value.time())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_leap_days_and_nanoseconds() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let value = day.and_hms_nano_opt(12, 34, 56, 120_000_001).unwrap();
        let text = "2024060T123456.120000001";
        assert_eq!(
            format_iso_basic_ordinal_local_datetime(value).as_deref(),
            Some(text)
        );
        assert_eq!(parse_iso_basic_ordinal_local_datetime(text), Some(value));
        assert_eq!(
            parse_iso_basic_ordinal_local_datetime("2023365T000000"),
            NaiveDate::from_ymd_opt(2023, 12, 31)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_invalid_ordinals_and_offsets() {
        for invalid in [
            "2023366T000000",
            "2024000T000000",
            "2024060T240000",
            "2024060T000000Z",
            "2024060T000000.1234567890",
            "2024060 000000",
        ] {
            assert_eq!(
                parse_iso_basic_ordinal_local_datetime(invalid),
                None,
                "{invalid}"
            );
        }
        assert_eq!(
            format_iso_basic_ordinal_local_datetime(NaiveDateTime::MIN),
            None
        );
    }
}
