//! Lazy weekday-only iteration over inclusive calendar intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::business_days::{is_business_day, next_business_day};

/// A lazy iterator over Monday-Friday dates in an inclusive interval.
#[derive(Clone, Debug)]
pub struct BusinessDaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for BusinessDaysInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = next_business_day(current).filter(|day| *day <= self.end);
        Some(current)
    }
}

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
        assert_eq!(days, vec![date(2024, 3, 1), date(2024, 3, 4), date(2024, 3, 5)]);
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
        assert_eq!(days.next(), is_business_day(NaiveDate::MAX).then_some(NaiveDate::MAX));
        assert_eq!(days.next(), None);
        assert_eq!(days.next(), None);
    }
}
