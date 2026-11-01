//! Lazy hourly stepping over inclusive naive datetime intervals.

use std::iter::FusedIterator;

use chrono::{NaiveDateTime, TimeDelta};

/// A lazy iterator preserving the exact starting clock minute and second.
#[derive(Clone, Debug)]
pub struct HoursInDatetimeInterval {
    next: Option<NaiveDateTime>,
    end: NaiveDateTime,
    step: TimeDelta,
}

impl Iterator for HoursInDatetimeInterval {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_signed(self.step)
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for HoursInDatetimeInterval {}

/// Step by a positive number of whole hours, including the first timestamp.
/// Reversed intervals and zero steps return None; stepping beyond the date
/// range stops iteration without wrapping.
pub fn each_nth_hour_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
    step_hours: u32,
) -> Option<HoursInDatetimeInterval> {
    if start > end || step_hours == 0 {
        return None;
    }
    Some(HoursInDatetimeInterval {
        next: Some(start),
        end,
        step: TimeDelta::try_hours(i64::from(step_hours))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn steps_across_midnight_and_a_leap_day() {
        let first = NaiveDate::from_ymd_opt(2024, 2, 28)
            .unwrap()
            .and_hms_opt(23, 30, 0)
            .unwrap();
        let second = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 30, 0)
            .unwrap();
        let third = second.checked_add_signed(TimeDelta::hours(1)).unwrap();
        let end = second.checked_add_signed(TimeDelta::minutes(90)).unwrap();
        let hours: Vec<_> = each_nth_hour_of_datetime_interval(first, end, 1)
            .unwrap()
            .collect();
        assert_eq!(hours, vec![first, second, third]);
    }

    #[test]
    fn rejects_invalid_steps_and_exhausts_at_supported_limits() {
        let start = NaiveDateTime::MAX;
        assert!(each_nth_hour_of_datetime_interval(start, start, 0).is_none());
        assert!(each_nth_hour_of_datetime_interval(start, NaiveDateTime::MIN, 1).is_none());
        let mut iter = each_nth_hour_of_datetime_interval(start, start, 1).unwrap();
        assert_eq!(iter.next(), Some(start));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        let mut large =
            each_nth_hour_of_datetime_interval(NaiveDateTime::MIN, NaiveDateTime::MAX, u32::MAX)
                .unwrap();
        assert_eq!(large.next(), Some(NaiveDateTime::MIN));
    }
}
