//! Inclusive date-interval utilities that validate endpoint order.

use chrono::NaiveDate;

use crate::difference::difference_in_calendar_days;

/// Clamp a date to an inclusive interval.
///
/// Returns None when the interval's start is after its end.
pub fn clamp_to_interval(date: NaiveDate, start: NaiveDate, end: NaiveDate) -> Option<NaiveDate> {
    (start <= end).then(|| date.clamp(start, end))
}

/// Find the intersection of two inclusive intervals.
///
/// The outer None indicates an invalid (reversed) input interval.
/// Some(None) indicates valid but non-overlapping intervals.
/// Some(Some((start, end))) contains the inclusive intersection.
pub fn intersection_of_intervals(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> Option<Option<(NaiveDate, NaiveDate)>> {
    if a_start > a_end || b_start > b_end {
        return None;
    }

    let start = a_start.max(b_start);
    let end = a_end.min(b_end);
    Some((start <= end).then_some((start, end)))
}

/// Count calendar days in an inclusive interval, including both endpoints.
///
/// Returns None for a reversed interval.
pub fn interval_length_in_days(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    (start <= end).then(|| difference_in_calendar_days(end, start) + 1)
}

/// Return whether one valid inclusive interval lies within another.
///
/// Returns None if either interval has reversed endpoints.
pub fn is_interval_within_interval(
    inner_start: NaiveDate,
    inner_end: NaiveDate,
    outer_start: NaiveDate,
    outer_end: NaiveDate,
) -> Option<bool> {
    if inner_start > inner_end || outer_start > outer_end {
        return None;
    }
    Some(inner_start >= outer_start && inner_end <= outer_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn clamps_to_inclusive_endpoints() {
        let start = date(2024, 2, 28);
        let end = date(2024, 3, 1);
        assert_eq!(clamp_to_interval(date(2024, 1, 1), start, end), Some(start));
        assert_eq!(
            clamp_to_interval(date(2024, 2, 29), start, end),
            Some(date(2024, 2, 29))
        );
        assert_eq!(clamp_to_interval(date(2024, 4, 1), start, end), Some(end));
        assert_eq!(clamp_to_interval(start, end, start), None);
    }

    #[test]
    fn intersection_distinguishes_invalid_disjoint_and_touching() {
        let a = date(2024, 1, 1);
        let b = date(2024, 1, 2);
        let c = date(2024, 1, 3);
        assert_eq!(intersection_of_intervals(a, b, b, c), Some(Some((b, b))));
        assert_eq!(intersection_of_intervals(a, a, b, c), Some(None));
        assert_eq!(intersection_of_intervals(b, a, a, c), None);
    }

    #[test]
    fn interval_length_includes_leap_days_and_maximum() {
        assert_eq!(
            interval_length_in_days(date(2024, 2, 28), date(2024, 3, 1)),
            Some(3)
        );
        assert_eq!(
            interval_length_in_days(NaiveDate::MAX, NaiveDate::MAX),
            Some(1)
        );
        assert_eq!(
            interval_length_in_days(NaiveDate::MAX, NaiveDate::MIN),
            None
        );
    }
    #[test]
    fn tests_inclusive_interval_containment() {
        let a = date(2024, 1, 1);
        let b = date(2024, 1, 2);
        let c = date(2024, 1, 3);
        assert_eq!(is_interval_within_interval(b, c, a, c), Some(true));
        assert_eq!(is_interval_within_interval(a, c, b, c), Some(false));
        assert_eq!(is_interval_within_interval(a, c, a, c), Some(true));
        assert_eq!(is_interval_within_interval(c, c, a, c), Some(true));
    }

    #[test]
    fn rejects_reversed_intervals() {
        let a = date(2024, 1, 1);
        let b = date(2024, 1, 2);
        assert_eq!(is_interval_within_interval(b, a, a, b), None);
        assert_eq!(is_interval_within_interval(a, b, b, a), None);
    }
}
