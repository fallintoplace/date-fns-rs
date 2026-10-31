//! Lazy iteration over every nth day of an inclusive date interval.

use std::iter::FusedIterator;

use chrono::{Days, NaiveDate};

/// An iterator over fixed-step calendar days, starting at the interval start.
#[derive(Clone, Debug)]
pub struct SteppedDaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
    step: u64,
}

impl Iterator for SteppedDaysInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_days(Days::new(self.step))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for SteppedDaysInInterval {}

/// Iterate through every step-th day in [start, end], inclusive.
///
/// The first element is always start. Returns None for a reversed interval
/// or a zero step. No dates are allocated in advance.
pub fn each_nth_day_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    step: u64,
) -> Option<SteppedDaysInInterval> {
    (start <= end && step != 0).then_some(SteppedDaysInInterval {
        next: Some(start),
        end,
        step,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn iterates_over_leap_days_and_includes_aligned_endpoint() {
        let days = each_nth_day_of_interval(date(2024, 2, 27), date(2024, 3, 2), 2)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(days, vec![date(2024, 2, 27), date(2024, 2, 29), date(2024, 3, 2)]);
    }

    #[test]
    fn rejects_zero_steps_and_reversed_intervals() {
        let first = date(2024, 3, 1);
        let last = date(2024, 3, 3);
        assert!(each_nth_day_of_interval(first, last, 0).is_none());
        assert!(each_nth_day_of_interval(last, first, 1).is_none());
        assert_eq!(
            each_nth_day_of_interval(first, last, 99)
                .unwrap()
                .collect::<Vec<_>>(),
            vec![first]
        );
    }

    #[test]
    fn stops_without_overflow_and_is_fused() {
        let mut iter = each_nth_day_of_interval(NaiveDate::MAX, NaiveDate::MAX, 1)
            .unwrap();
        assert_eq!(iter.next(), Some(NaiveDate::MAX));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
