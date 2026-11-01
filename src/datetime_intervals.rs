//! Checked inclusive intervals of naive date-times.

use chrono::NaiveDateTime;

/// Test membership in an inclusive datetime interval.
pub fn is_within_datetime_interval(
    value: NaiveDateTime,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<bool> {
    (start <= end).then_some(start <= value && value <= end)
}

/// Test inclusive overlap, including intervals that share an endpoint.
pub fn are_datetime_intervals_overlapping(
    a_start: NaiveDateTime,
    a_end: NaiveDateTime,
    b_start: NaiveDateTime,
    b_end: NaiveDateTime,
) -> Option<bool> {
    (a_start <= a_end && b_start <= b_end).then_some(a_start <= b_end && b_start <= a_end)
}

/// Clamp a datetime to an inclusive interval.
pub fn clamp_datetime_to_interval(
    value: NaiveDateTime,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<NaiveDateTime> {
    (start <= end).then(|| value.clamp(start, end))
}

/// Find the inclusive intersection, distinguishing invalid and disjoint ranges.
pub fn intersection_of_datetime_intervals(
    a_start: NaiveDateTime,
    a_end: NaiveDateTime,
    b_start: NaiveDateTime,
    b_end: NaiveDateTime,
) -> Option<Option<(NaiveDateTime, NaiveDateTime)>> {
    if a_start > a_end || b_start > b_end {
        return None;
    }
    let start = a_start.max(b_start);
    let end = a_end.min(b_end);
    Some((start <= end).then_some((start, end)))
}

/// Duration in whole milliseconds between ordered endpoints.
/// Unlike date-only interval lengths, both endpoints do not add a millisecond.
pub fn datetime_interval_duration_millis(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<i64> {
    (start <= end).then(|| end.signed_duration_since(start).num_milliseconds())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    #[test]
    fn handles_touching_reversed_and_subsecond_ranges() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap().and_hms_opt(23, 59, 59).unwrap();
        let middle = start.checked_add_signed(TimeDelta::milliseconds(500)).unwrap();
        let end = start.checked_add_signed(TimeDelta::seconds(1)).unwrap();
        assert_eq!(datetime_interval_duration_millis(start, end), Some(1_000));
        assert_eq!(datetime_interval_duration_millis(end, start), None);
        assert_eq!(are_datetime_intervals_overlapping(start, middle, middle, end), Some(true));
        assert_eq!(intersection_of_datetime_intervals(start, middle, middle, end), Some(Some((middle, middle))));
        assert_eq!(clamp_datetime_to_interval(end, start, middle), Some(middle));
        assert_eq!(is_within_datetime_interval(end, start, middle), Some(false));
        assert_eq!(is_within_datetime_interval(start, end, middle), None);
    }

    #[test]
    fn includes_datetime_limits() {
        assert_eq!(datetime_interval_duration_millis(NaiveDateTime::MAX, NaiveDateTime::MAX), Some(0));
        assert_eq!(clamp_datetime_to_interval(NaiveDateTime::MIN, NaiveDateTime::MAX, NaiveDateTime::MAX), Some(NaiveDateTime::MAX));
    }
}