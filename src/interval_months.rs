//! Lazy first-of-month dates for inclusive calendar intervals.

use std::iter::FusedIterator;

use chrono::{Months, NaiveDate};

use crate::boundaries::start_of_month;

/// A lazy iterator over month starts in an inclusive interval.
#[derive(Clone, Debug)]
pub struct MonthsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for MonthsInInterval {
    type Item = NaiveDate;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.map_or(0, |next| {
            (crate::difference::difference_in_calendar_months(self.end, next) + 1) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        self.next = self.next?.checked_add_months(Months::new(n as u32));
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(1))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl ExactSizeIterator for MonthsInInterval {}

impl FusedIterator for MonthsInInterval {}

/// Iterate through month starts for months overlapping [start, end].
///
/// The first month start may be before start. Reversed intervals return None.
pub fn each_month_of_interval(start: NaiveDate, end: NaiveDate) -> Option<MonthsInInterval> {
    (start <= end).then_some(MonthsInInterval {
        next: Some(start_of_month(start)),
        end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn crosses_year_and_leap_month_boundaries() {
        let months = each_month_of_interval(date(2023, 12, 31), date(2024, 2, 29))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            months,
            vec![date(2023, 12, 1), date(2024, 1, 1), date(2024, 2, 1)]
        );
    }

    #[test]
    fn handles_single_date_and_reversed_intervals() {
        let day = date(2024, 2, 29);
        let months = each_month_of_interval(day, day).unwrap();
        assert_eq!(months.collect::<Vec<_>>(), vec![date(2024, 2, 1)]);
        assert!(each_month_of_interval(day, date(2024, 2, 1)).is_none());
    }

    #[test]
    fn stops_at_maximum_without_wrapping() {
        let mut months = each_month_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(months.next(), Some(start_of_month(NaiveDate::MAX)));
        assert_eq!(months.next(), None);
        assert_eq!(months.next(), None);
    }
    #[test]
    fn skips_month_starts_and_tracks_exact_remaining_count() {
        let mut months =
            each_month_of_interval(date(2023, 12, 31), date(2024, 4, 2)).unwrap();
        assert_eq!(months.len(), 5);
        assert_eq!(months.nth(2), Some(date(2024, 2, 1)));
        assert_eq!(months.len(), 2);
        assert_eq!(months.next(), Some(date(2024, 3, 1)));
        assert_eq!(months.len(), 1);
    }

    #[test]
    fn exhausts_impossible_skips_at_date_limits() {
        let mut months = each_month_of_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(months.nth(usize::MAX), None);
        assert_eq!(months.size_hint(), (0, Some(0)));
    }
}
