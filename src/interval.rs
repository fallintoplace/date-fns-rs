//! Inclusive calendar intervals with a lazy day iterator.

use std::iter::FusedIterator;

use chrono::NaiveDate;

/// A lazy iterator over each calendar day in an inclusive interval.
#[derive(Clone, Debug)]
pub struct DaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for DaysInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = if current < self.end {
            current.succ_opt()
        } else {
            None
        };
        Some(current)
    }
}

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
    fn rejects_reversed_intervals() {
        let first = date(2024, 1, 1);
        let second = date(2024, 1, 2);
        assert!(each_day_of_interval(second, first).is_none());
        assert_eq!(is_within_interval(first, second, first), None);
        assert_eq!(are_intervals_overlapping(second, first, first, second), None);
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
}
