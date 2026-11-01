//! Lazy iteration over inclusive naive datetime ranges in whole-hour steps.

use std::iter::FusedIterator;

use chrono::{NaiveDateTime, TimeDelta};

/// A lazy forward iterator retaining the starting timestamp's clock precision.
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

/// Iterate an inclusive datetime interval at a positive whole-hour step.
///
/// The first value is exactly start. Returns None for reversed endpoints or
/// a zero step, and ends cleanly when stepping exceeds Chrono's range.
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
        step: TimeDelta::hours(i64::from(step_hours)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn steps_across_midnight_with_original_subseconds() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_milli_opt(23, 30, 15, 123)
            .unwrap();
        let end = start.checked_add_signed(TimeDelta::hours(5)).unwrap();
        let values: Vec<_> = each_nth_hour_of_datetime_interval(start, end, 2)
            .unwrap()
            .collect();
        assert_eq!(
            values,
            vec![
                start,
                start.checked_add_signed(TimeDelta::hours(2)).unwrap(),
                start.checked_add_signed(TimeDelta::hours(4)).unwrap()
            ]
        );
    }

    #[test]
    fn rejects_invalid_ranges_and_fuses_at_datetime_max() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(each_nth_hour_of_datetime_interval(start, start, 0).is_none());
        assert!(each_nth_hour_of_datetime_interval(start, NaiveDateTime::MIN, 1).is_none());
        let mut it =
            each_nth_hour_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, 1)
                .unwrap();
        assert_eq!(it.next(), Some(NaiveDateTime::MAX));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
        assert_eq!(
            each_nth_hour_of_datetime_interval(start, start, u32::MAX)
                .unwrap()
                .count(),
            1
        );
    }
}
