//! Lazy stepped iteration over inclusive naive datetime intervals.

use std::iter::FusedIterator;

use chrono::{NaiveDateTime, TimeDelta};

/// A lazy iterator starting at an exact timestamp, inclusive of both endpoints.
#[derive(Clone, Debug)]
pub struct MinutesInDatetimeInterval {
    next: Option<NaiveDateTime>,
    end: NaiveDateTime,
    step: TimeDelta,
}

impl Iterator for MinutesInDatetimeInterval {
    type Item = NaiveDateTime;

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let offset = i64::try_from(n)
            .ok()
            .and_then(|count| count.checked_mul(self.step.num_minutes()))
            .and_then(TimeDelta::try_minutes);
        self.next = self
            .next
            .and_then(|current| offset.and_then(|duration| current.checked_add_signed(duration)))
            .filter(|next| *next <= self.end);
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_signed(self.step)
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for MinutesInDatetimeInterval {}

/// Iterate an inclusive interval at a positive whole-minute step.
/// The first value is always start, even when it is not a minute boundary.
/// Returns None for reversed endpoints or a zero step.
pub fn each_nth_minute_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
    step_minutes: u32,
) -> Option<MinutesInDatetimeInterval> {
    if start > end || step_minutes == 0 {
        return None;
    }
    Some(MinutesInDatetimeInterval {
        next: Some(start),
        end,
        step: TimeDelta::minutes(i64::from(step_minutes)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn iterates_over_midnight_and_stops_at_end() {
        let first = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(23, 59, 0)
            .unwrap();
        let last = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 1, 0)
            .unwrap();
        let values: Vec<_> = each_nth_minute_of_datetime_interval(first, last, 1)
            .unwrap()
            .collect();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], first);
        assert_eq!(values[2], last);
        assert_eq!(
            each_nth_minute_of_datetime_interval(first, last, 2)
                .unwrap()
                .count(),
            2
        );
    }

    #[test]
    fn validates_intervals_and_stays_fused_at_max() {
        let first = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(each_nth_minute_of_datetime_interval(first, first, 0).is_none());
        assert!(each_nth_minute_of_datetime_interval(first, NaiveDateTime::MIN, 1).is_none());
        let mut it =
            each_nth_minute_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, 1)
                .unwrap();
        assert_eq!(it.next(), Some(NaiveDateTime::MAX));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn skips_minute_samples_in_one_step_and_preserves_subseconds() {
        let first = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 30, 123_456_789)
            .unwrap();
        let last = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_nano_opt(0, 5, 30, 123_456_789)
            .unwrap();
        let mut samples = each_nth_minute_of_datetime_interval(first, last, 2).unwrap();
        assert_eq!(samples.nth(2), first.checked_add_signed(TimeDelta::minutes(4)));
        assert_eq!(samples.next(), Some(last));
        assert_eq!(samples.next(), None);

        let mut extreme =
            each_nth_minute_of_datetime_interval(NaiveDateTime::MIN, NaiveDateTime::MAX, 1)
                .unwrap();
        assert_eq!(extreme.nth(usize::MAX), None);
        assert_eq!(extreme.next(), None);
    }
}
