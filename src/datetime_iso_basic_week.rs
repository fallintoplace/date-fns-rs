//! ISO basic week-date local date-times with no implicit time zone.

use chrono::NaiveDateTime;

/// Parse a basic week-date datetime such as 2020W535T235958.123456789.
///
/// An optional fractional part may contain 1-9 digits. Offsets are rejected.
pub fn parse_iso_basic_week_local_datetime(input: &str) -> Option<NaiveDateTime> {
    let (day, time) = input.split_once('T')?;
    Some(crate::parse_iso_basic_week_date(day)?.and_time(crate::parse_iso_basic_local_time(time)?))
}

/// Format a local datetime as YYYYWwwDTHHMMSS with optional nanoseconds.
pub fn format_iso_basic_week_local_datetime(value: NaiveDateTime) -> Option<String> {
    Some(format!(
        "{}T{}",
        crate::format_iso_basic_week_date(value.date())?,
        crate::format_iso_basic_local_time(value.time())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_week_year_changes_and_subseconds() {
        let day = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let value = day.and_hms_nano_opt(23, 59, 58, 123_456_789).unwrap();
        let text = "2020W535T235958.123456789";
        assert_eq!(format_iso_basic_week_local_datetime(value).as_deref(), Some(text));
        assert_eq!(parse_iso_basic_week_local_datetime(text), Some(value));
        assert_eq!(
            parse_iso_basic_week_local_datetime("2019W011T000000"),
            NaiveDate::from_ymd_opt(2018, 12, 31)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_impossible_weeks_and_offsets() {
        for invalid in [
            "2021W535T000000",
            "2020W535T240000",
            "2020W535T000000Z",
            "2020W535T000000.1234567890",
            "2020W535 000000",
        ] {
            assert_eq!(parse_iso_basic_week_local_datetime(invalid), None, "{invalid}");
        }
        assert_eq!(format_iso_basic_week_local_datetime(NaiveDateTime::MAX), None);
    }
}
