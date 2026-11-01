//! Lazy weekday-only iteration over inclusive calendar intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::business_days::{add_business_days, is_business_day, next_business_day};
use crate::interval_day_counts::count_business_days_in_interval;

/// A lazy iterator over Monday-Friday dates in an inclusive interval.
#[derive(Clone, Debug)]
pub struct BusinessDaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for BusinessDaysInInterval {
    type Item = NaiveDate;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.map_or(0, |next| {
            count_business_days_in_interval(next, self.end).unwrap_or(0) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        self.next = add_business_days(self.next?, i64::try_from(n).ok()?);
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = next_business_day(current).filter(|day| *day <= self.end);
        Some(current)
    }
}

impl ExactSizeIterator for BusinessDaysInInterval {}

impl FusedIterator for BusinessDaysInInterval {}

/// Iterate over weekdays in [start, end] without allocating a collection.
///
/// The interval is inclusive. Reversed ranges return None; valid ranges
/// containing no weekdays return an empty iterator. Holidays are not excluded.
pub fn each_business_day_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<BusinessDaysInInterval> {
    if start > end {
        return None;
    }
    let first = if is_business_day(start) {
        Some(start)
    } else {
        next_business_day(start)
    }
    .filter(|day| *day <= end);
    Some(BusinessDaysInInterval { next: first, end })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn yields_weekdays_across_a_weekend() {
        let days = each_business_day_of_interval(date(2024, 3, 1), date(2024, 3, 5))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            days,
            vec![date(2024, 3, 1), date(2024, 3, 4), date(2024, 3, 5)]
        );
    }

    #[test]
    fn supports_empty_weekends_and_reversed_ranges() {
        let saturday = date(2024, 3, 2);
        let sunday = date(2024, 3, 3);
        assert_eq!(
            each_business_day_of_interval(saturday, sunday)
                .unwrap()
                .count(),
            0
        );
        assert!(each_business_day_of_interval(sunday, saturday).is_none());
        assert_eq!(
            each_business_day_of_interval(saturday, saturday)
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn is_fused_at_maximum_date() {
        let mut days = each_business_day_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(
            days.next(),
            is_business_day(NaiveDate::MAX).then_some(NaiveDate::MAX)
        );
        assert_eq!(days.next(), None);
        assert_eq!(days.next(), None);
    }

    #[test]
    fn counts_and_skips_business_days_without_traversal() {
        let mut days = each_business_day_of_interval(date(2024, 3, 1), date(2024, 3, 11)).unwrap();
        assert_eq!(days.len(), 7);
        assert_eq!(days.nth(2), Some(date(2024, 3, 5)));
        assert_eq!(days.len(), 4);
        assert_eq!(days.nth(2), Some(date(2024, 3, 8)));
        assert_eq!(days.len(), 1);
        assert_eq!(days.next(), Some(date(2024, 3, 11)));
        assert_eq!(days.len(), 0);

        let mut weekend =
            each_business_day_of_interval(date(2024, 3, 2), date(2024, 3, 3)).unwrap();
        assert_eq!(weekend.len(), 0);
        assert_eq!(weekend.nth(0), None);

        let mut extreme = each_business_day_of_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(extreme.nth(usize::MAX), None);
        assert_eq!(extreme.next(), None);
    }
}
