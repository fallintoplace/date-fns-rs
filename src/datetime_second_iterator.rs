//! Lazy stepped iteration at second precision over inclusive naive datetime ranges.

use std::iter::FusedIterator;

use chrono::{NaiveDateTime, TimeDelta};

/// A lazy second-step iterator that preserves the starting timestamp's nanoseconds.
#[derive(Clone, Debug)]
pub struct SecondsInDatetimeInterval {
    next: Option<NaiveDateTime>,
    end: NaiveDateTime,
    step_seconds: i64,
}

impl Iterator for SecondsInDatetimeInterval {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_signed(TimeDelta::seconds(self.step_seconds))
            .filter(|next| *next <= self.end);
        Some(current)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.next = self
            .next
            .and_then(|current| {
                i64::try_from(n)
                    .ok()
                    .and_then(|count| count.checked_mul(self.step_seconds))
                    .and_then(|seconds| current.checked_add_signed(TimeDelta::seconds(seconds)))
            })
            .filter(|next| *next <= self.end);
        self.next()
    }
}

impl FusedIterator for SecondsInDatetimeInterval {}

/// Iterate an inclusive interval at a positive whole-second step.
///
/// The first value is exactly start. Returns None for a zero step or reversed
/// range. Large skips terminate safely without walking every intermediate time.
pub fn each_nth_second_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
    step_seconds: u32,
) -> Option<SecondsInDatetimeInterval> {
    if start > end || step_seconds == 0 {
        return None;
    }
    Some(SecondsInDatetimeInterval {
        next: Some(start),
        end,
        step_seconds: i64::from(step_seconds),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn iterates_across_midnight_preserving_nanoseconds() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_456_789)
            .unwrap();
        let end = start.checked_add_signed(TimeDelta::seconds(5)).unwrap();
        let samples: Vec<_> = each_nth_second_of_datetime_interval(start, end, 2)
            .unwrap()
            .collect();
        assert_eq!(
            samples,
            vec![
                start,
                start.checked_add_signed(TimeDelta::seconds(2)).unwrap(),
                start.checked_add_signed(TimeDelta::seconds(4)).unwrap()
            ]
        );
    }

    #[test]
    fn nth_skips_in_constant_time_and_exhausts_at_end() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end = start.checked_add_signed(TimeDelta::seconds(10)).unwrap();
        let mut it = each_nth_second_of_datetime_interval(start, end, 2).unwrap();
        assert_eq!(
            it.nth(3),
            start.checked_add_signed(TimeDelta::seconds(6))
        );
        assert_eq!(it.next(), start.checked_add_signed(TimeDelta::seconds(8)));
        assert_eq!(it.nth(2), None);
        assert_eq!(it.next(), None);

        let mut huge =
            each_nth_second_of_datetime_interval(NaiveDateTime::MIN, NaiveDateTime::MAX, 1)
                .unwrap();
        assert_eq!(huge.nth(usize::MAX), None);
        assert_eq!(huge.next(), None);
    }

    #[test]
    fn validates_steps_and_maximum_datetime() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(each_nth_second_of_datetime_interval(start, start, 0).is_none());
        assert!(each_nth_second_of_datetime_interval(start, NaiveDateTime::MIN, 1).is_none());
        let mut it =
            each_nth_second_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, 1)
                .unwrap();
        assert_eq!(it.next(), Some(NaiveDateTime::MAX));
        assert_eq!(it.next(), None);
    }
}
