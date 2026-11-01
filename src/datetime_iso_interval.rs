//! ISO 8601 local date-time intervals with inclusive endpoints.

use chrono::NaiveDateTime;

/// Parse a start/end pair of ISO local date-times, separated by one slash.
///
/// The pair must be ordered. Neither offset-aware nor abbreviated inputs
/// are accepted, and no time zone is inferred.
pub fn parse_iso_local_datetime_interval(
    input: &str,
) -> Option<(NaiveDateTime, NaiveDateTime)> {
    let (left, right) = input.split_once('/')?;
    let start = crate::parse_iso_local_datetime(left)?;
    let end = crate::parse_iso_local_datetime(right)?;
    (start <= end).then_some((start, end))
}

/// Format an ordered inclusive ISO local date-time interval.
///
/// Returns None for reversed endpoints or dates outside ISO four-digit years.
pub fn format_iso_local_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<String> {
    if start > end {
        return None;
    }
    Some(format!(
        "{}/{}",
        crate::format_iso_local_datetime(start)?,
        crate::format_iso_local_datetime(end)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn round_trips_precise_interval_crossing_leap_day() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, 123_456_789)
            .unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let text = "2024-02-29T23:59:59.123456789/2024-03-01T00:00:00";
        assert_eq!(parse_iso_local_datetime_interval(text), Some((start, end)));
        assert_eq!(
            format_iso_local_datetime_interval(start, end).as_deref(),
            Some(text)
        );
        assert_eq!(
            parse_iso_local_datetime_interval(
                "2024-02-29T00:00:00/2024-02-29T00:00:00"
            ),
            Some((
                start.date().and_hms_opt(0, 0, 0).unwrap(),
                start.date().and_hms_opt(0, 0, 0).unwrap()
            ))
        );
    }

    #[test]
    fn rejects_reversed_missing_and_offset_endpoints() {
        let later = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let earlier = later.checked_sub_signed(chrono::TimeDelta::seconds(1)).unwrap();
        assert_eq!(format_iso_local_datetime_interval(later, earlier), None);
        for text in [
            "2024-03-01T00:00:00/2024-02-29T23:59:59",
            "2024-03-01T00:00:00",
            "2024-03-01T00:00:00/",
            "/2024-03-01T00:00:00",
            "2024-03-01T00:00:00Z/2024-03-02T00:00:00Z",
            "2024-03-01T00:00:00/2024-03-02T00:00:00/extra",
        ] {
            assert_eq!(parse_iso_local_datetime_interval(text), None, "{text}");
        }
    }
}
