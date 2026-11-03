//! Same-day local time ranges without dates or UTC offsets.

use chrono::NaiveTime;

/// Parse an inclusive HH:MM:SS[/fraction] time span separated by a slash.
///
/// The range may be a single time, but overnight ranges are rejected.
pub fn parse_iso_local_time_interval(input: &str) -> Option<(NaiveTime, NaiveTime)> {
    let (left, right) = input.split_once('/')?;
    let start = crate::parse_iso_local_time(left)?;
    let end = crate::parse_iso_local_time(right)?;
    (start <= end).then_some((start, end))
}

/// Format a time-only range, preserving nanoseconds without guessing a zone.
///
/// Leap seconds and reversed intervals return None.
pub fn format_iso_local_time_interval(start: NaiveTime, end: NaiveTime) -> Option<String> {
    if start > end {
        return None;
    }
    let left = crate::format_iso_local_time(start)?;
    let right = crate::format_iso_local_time(end)?;
    Some(format!("{left}/{right}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_fractional_and_equal_time_ranges() {
        let input = "09:30:00.1/17:45:00.25";
        let start = NaiveTime::from_hms_nano_opt(9, 30, 0, 100_000_000).unwrap();
        let end = NaiveTime::from_hms_nano_opt(17, 45, 0, 250_000_000).unwrap();
        assert_eq!(parse_iso_local_time_interval(input), Some((start, end)));
        assert_eq!(
            format_iso_local_time_interval(start, end).as_deref(),
            Some(input)
        );
        assert_eq!(
            parse_iso_local_time_interval("00:00:00/00:00:00"),
            Some((NaiveTime::MIN, NaiveTime::MIN))
        );
    }

    #[test]
    fn rejects_overnight_offset_and_noncanonical_ranges() {
        for invalid in [
            "17:00:00/09:00:00",
            "09:00:00/08:59:59",
            "09:00:00Z/10:00:00Z",
            "09:00/10:00",
            "09:00:00/10:00:00/11:00:00",
            "09:00:00 /10:00:00",
            "23:59:60/23:59:60",
        ] {
            assert_eq!(parse_iso_local_time_interval(invalid), None, "{invalid}");
        }
        assert_eq!(
            format_iso_local_time_interval(
                NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(9, 0, 0).unwrap()
            ),
            None
        );
    }
}
