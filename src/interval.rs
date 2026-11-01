//! Inclusive calendar intervals with a lazy day iterator.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate};

/// A lazy iterator over each calendar day in an inclusive interval.
#[derive(Clone, Debug)]
pub struct DaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for DaysInInterval {
    type Item = NaiveDate;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.map_or(0, |next| {
            (i64::from(self.end.num_days_from_ce()) - i64::from(next.num_days_from_ce()) + 1)
                as usize
        });
        (remaining, Some(remaining))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = if current < self.end {
            current.succ_opt()
        } else {
            None
        };
        Some(current)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let current = self.next?;
        self.next = current.checked_add_days(chrono::Days::new(n as u64));
        self.next()
    }
}

impl DoubleEndedIterator for DaysInInterval {
    fn next_back(&mut self) -> Option<Self::Item> {
        let front = self.next?;
        let back = self.end;
        if front == back {
            self.next = None;
        } else {
            self.end = back.pred_opt()?;
        }
        Some(back)
    }
}

impl ExactSizeIterator for DaysInInterval {}

impl FusedIterator for DaysInInterval {}

/// Iterate over all days between `start` and `end`, inclusive.
///
/// Returns `None` for a reversed interval. No dates are allocated upfront.
pub fn each_day_of_interval(start: NaiveDate, end: NaiveDate) -> Option<DaysInInterval> {
    (start <= end).then_some(DaysInInterval {
        next: Some(start),
        end,
    })
}

/// Check whether `date` lies inside an inclusive interval.
///
/// Returns `None` for a reversed interval.
pub fn is_within_interval(date: NaiveDate, start: NaiveDate, end: NaiveDate) -> Option<bool> {
    (start <= end).then_some(start <= date && date <= end)
}

/// Check whether two inclusive intervals overlap, including at endpoints.
///
/// Returns `None` if either interval is reversed.
pub fn are_intervals_overlapping(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> Option<bool> {
    (a_start <= a_end && b_start <= b_end).then_some(a_start <= b_end && b_start <= a_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn iterates_across_leap_day_without_skipping_endpoints() {
        let days: Vec<_> = each_day_of_interval(date(2024, 2, 28), date(2024, 3, 1))
            .unwrap()
            .collect();
        assert_eq!(
            days,
            vec![date(2024, 2, 28), date(2024, 2, 29), date(2024, 3, 1)]
        );
    }

    #[test]
    fn iterator_is_fused_and_handles_the_maximum_date() {
        let mut days = each_day_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(days.next(), Some(NaiveDate::MAX));
        assert_eq!(days.next(), None);
        assert_eq!(days.next(), None);
    }

    #[test]
    fn supports_interleaved_forward_and_backward_iteration() {
        let mut days = each_day_of_interval(date(2024, 1, 1), date(2024, 1, 5)).unwrap();
        assert_eq!(days.len(), 5);
        assert_eq!(days.size_hint(), (5, Some(5)));
        assert_eq!(days.next_back(), Some(date(2024, 1, 5)));
        assert_eq!(days.next(), Some(date(2024, 1, 1)));
        assert_eq!(days.len(), 3);
        assert_eq!(days.next_back(), Some(date(2024, 1, 4)));
        assert_eq!(days.next_back(), Some(date(2024, 1, 3)));
        assert_eq!(days.next(), Some(date(2024, 1, 2)));
        assert_eq!(days.len(), 0);
        assert_eq!(days.next_back(), None);
        assert_eq!(days.next(), None);
    }

    #[test]
    fn handles_backwards_iteration_at_supported_limits() {
        let previous = NaiveDate::MAX.pred_opt().unwrap();
        let mut days = each_day_of_interval(previous, NaiveDate::MAX).unwrap();
        assert_eq!(days.next_back(), Some(NaiveDate::MAX));
        assert_eq!(days.next_back(), Some(previous));
        assert_eq!(days.next_back(), None);
        assert_eq!(days.size_hint(), (0, Some(0)));

        let mut single = each_day_of_interval(NaiveDate::MIN, NaiveDate::MIN).unwrap();
        assert_eq!(single.next_back(), Some(NaiveDate::MIN));
        assert_eq!(single.next(), None);
    }

    #[test]
    fn rejects_reversed_intervals() {
        let first = date(2024, 1, 1);
        let second = date(2024, 1, 2);
        assert!(each_day_of_interval(second, first).is_none());
        assert_eq!(is_within_interval(first, second, first), None);
        assert_eq!(
            are_intervals_overlapping(second, first, first, second),
            None
        );
    }

    #[test]
    fn tests_containment_and_inclusive_overlap() {
        let first = date(2024, 1, 1);
        let middle = date(2024, 1, 2);
        let last = date(2024, 1, 3);
        assert_eq!(is_within_interval(middle, first, last), Some(true));
        assert_eq!(is_within_interval(last, first, middle), Some(false));
        let touching = are_intervals_overlapping(first, middle, middle, last);
        assert_eq!(touching, Some(true));
        let separate = are_intervals_overlapping(first, first, middle, last);
        assert_eq!(separate, Some(false));
    }
    #[test]
    fn nth_skips_without_repeated_iteration() {
        let mut days = each_day_of_interval(date(2024, 1, 1), date(2024, 12, 31)).unwrap();
        assert_eq!(days.nth(199), Some(date(2024, 7, 18)));
        assert_eq!(days.len(), 166);
        assert_eq!(days.next_back(), Some(date(2024, 12, 31)));
        assert_eq!(days.len(), 165);
        assert_eq!(days.nth(usize::MAX), None);
        assert_eq!(days.next(), None);
    }

    #[test]
    fn nth_exhausts_at_supported_date_limits() {
        let mut days = each_day_of_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(days.nth(usize::MAX), None);
        assert_eq!(days.size_hint(), (0, Some(0)));
    }
}
