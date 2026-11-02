//! Strict SQL-style ISO 9075 local timestamps without a timezone assumption.

use chrono::NaiveDateTime;

/// Format YYYY-MM-DD HH:MM:SS with optional fractional nanoseconds.
pub fn format_iso9075_datetime(value: NaiveDateTime) -> Option<String> {
    let mut text = crate::format_iso_local_datetime(value)?;
    text.replace_range(10..11, " ");
    Some(text)
}

/// Parse a SQL-style timestamp with a required space, not a T separator.
///
/// Rejects offsets, whitespace, invalid dates and noncanonical field widths.
pub fn parse_iso9075_datetime(input: &str) -> Option<NaiveDateTime> {
    if input.len() < 19 || input.as_bytes().get(10) != Some(&b' ') {
        return None;
    }
    let mut text = input.to_owned();
    text.replace_range(10..11, "T");
    crate::parse_iso_local_datetime(&text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_leap_days_and_nanoseconds() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_456_789)
            .unwrap();
        let text = "2024-02-29 23:59:58.123456789";
        assert_eq!(format_iso9075_datetime(value).as_deref(), Some(text));
        assert_eq!(parse_iso9075_datetime(text), Some(value));
        assert_eq!(
            parse_iso9075_datetime("2024-02-29 23:59:58"),
            Some(value.date().and_hms_opt(23, 59, 58).unwrap())
        );
    }

    #[test]
    fn rejects_offsets_invalid_dates_and_extra_whitespace() {
        for value in [
            "2024-02-29T23:59:58",
            "2024-02-29 23:59:58Z",
            "2024-02-29 23:59:58+01:00",
            "2024-02-30 23:59:58",
            "2024-02-29 24:59:58",
            "2024-02-29 23:59:58 ",
            "2024-02-29 23:59:58.1234567890",
        ] {
            assert_eq!(parse_iso9075_datetime(value), None);
        }
        assert_eq!(format_iso9075_datetime(NaiveDateTime::MAX), None);
    }
}
