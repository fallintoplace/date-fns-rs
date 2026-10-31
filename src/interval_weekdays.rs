//! Lazy occurrence iteration for one weekday in an inclusive interval.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate, Weekday};

use crate::arithmetic::add_days;

/// A lazy iterator over dates matching a specified weekday.
#[derive(Clone, Debug)]
pub struct WeekdaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for WeekdaysInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = add_days(current, 7).filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for WeekdaysInInterval {}

/// Iterate dates with the requested weekday in an inclusive interval.
///
/// An interval without any matches yields an empty iterator. A reversed
/// interval returns None. Results are not collected in memory.
pub fn each_weekday_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    weekday: Weekday,
) -> Option<WeekdaysInInterval> {
    if start > end {
        return None;
    }

    let current = i64::from(start.weekday().num_days_from_monday());
    let target = i64::from(weekday.num_days_from_monday());
    let offset = (target - current).rem_euclid(7);
    let next = add_days(start, offset).filter(|date| *date <= end);
    Some(WeekdaysInInterval { next, end })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_weekly_occurrences_across_a_month_boundary() {
        let result: Vec<_> =
            each_weekday_of_interval(date(2024, 2, 27), date(2024, 3, 12), Weekday::Tue)
                .unwrap()
                .collect();
        assert_eq!(
            result,
            vec![date(2024, 2, 27), date(2024, 3, 5), date(2024, 3, 12)]
        );
    }

    #[test]
    fn handles_empty_and_reversed_ranges() {
        let day = date(2024, 3, 11);
        assert_eq!(
            each_weekday_of_interval(day, day, Weekday::Tue)
                .unwrap()
                .count(),
            0
        );
        assert!(each_weekday_of_interval(date(2024, 3, 12), day, Weekday::Mon).is_none());
    }

    #[test]
    fn is_fused_at_the_maximum_supported_date() {
        let mut days =
            each_weekday_of_interval(NaiveDate::MAX, NaiveDate::MAX, NaiveDate::MAX.weekday())
                .unwrap();
        assert_eq!(days.next(), Some(NaiveDate::MAX));
        assert_eq!(days.next(), None);
        assert_eq!(days.next(), None);
    }
}
