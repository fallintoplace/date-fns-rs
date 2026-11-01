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

    fn size_hint(&self) -> (usize, Option<usize>) {
        use chrono::Datelike;

        let remaining = self.next.map_or(0, |next| {
            let days = i64::from(self.end.num_days_from_ce()) - i64::from(next.num_days_from_ce());
            (days as u64 / self.step + 1) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let current = self.next?;
        let offset = (n as u64).checked_mul(self.step)?;
        self.next = current.checked_add_days(Days::new(offset));
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_days(Days::new(self.step))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl DoubleEndedIterator for SteppedDaysInInterval {
    fn next_back(&mut self) -> Option<Self::Item> {
        let front = self.next?;
        let remaining_days = (self.end - front).num_days() as u64;
        let steps = remaining_days / self.step;
        let last = front.checked_add_days(Days::new(steps * self.step))?;
        if last == front {
            self.next = None;
        } else {
            self.end = last.pred_opt()?;
        }
        Some(last)
    }
}

impl ExactSizeIterator for SteppedDaysInInterval {}

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
        assert_eq!(
            days,
            vec![date(2024, 2, 27), date(2024, 2, 29), date(2024, 3, 2)]
        );
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
        let mut iter = each_nth_day_of_interval(NaiveDate::MAX, NaiveDate::MAX, 1).unwrap();
        assert_eq!(iter.next(), Some(NaiveDate::MAX));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn exposes_exact_lengths_and_skips_directly() {
        let mut dates = each_nth_day_of_interval(date(2024, 2, 27), date(2024, 3, 4), 2).unwrap();
        assert_eq!(dates.len(), 4);
        assert_eq!(dates.nth(2), Some(date(2024, 3, 2)));
        assert_eq!(dates.len(), 1);
        assert_eq!(dates.next(), Some(date(2024, 3, 4)));
        assert_eq!(dates.size_hint(), (0, Some(0)));
    }

    #[test]
    fn skips_excessively_large_positions_without_overflow() {
        let mut dates = each_nth_day_of_interval(NaiveDate::MIN, NaiveDate::MAX, 7).unwrap();
        assert_eq!(dates.nth(usize::MAX), None);
        assert_eq!(dates.next(), None);
    }

    #[test]
    fn reverse_iteration_preserves_the_step_alignment() {
        let mut dates = each_nth_day_of_interval(date(2024, 3, 1), date(2024, 3, 15), 3).unwrap();
        assert_eq!(dates.len(), 5);
        assert_eq!(dates.next_back(), Some(date(2024, 3, 13)));
        assert_eq!(dates.len(), 4);
        assert_eq!(dates.next(), Some(date(2024, 3, 1)));
        assert_eq!(dates.next_back(), Some(date(2024, 3, 10)));
        assert_eq!(dates.nth(1), Some(date(2024, 3, 7)));
        assert_eq!(dates.next_back(), None);
        assert_eq!(dates.len(), 0);

        let mut extreme =
            each_nth_day_of_interval(NaiveDate::MAX, NaiveDate::MAX, u64::MAX).unwrap();
        assert_eq!(extreme.next_back(), Some(NaiveDate::MAX));
        assert_eq!(extreme.next_back(), None);
    }
}
