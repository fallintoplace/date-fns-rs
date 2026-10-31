//! Lazy Saturday and Sunday iteration across inclusive intervals.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate, Weekday};

use crate::arithmetic::add_days;

/// A lazy iterator over weekend dates in chronological order.
#[derive(Clone, Debug)]
pub struct WeekendsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for WeekendsInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        let days = if current.weekday() == Weekday::Sat {
            1
        } else {
            6
        };
        self.next = add_days(current, days).filter(|date| *date <= self.end);
        Some(current)
    }
}

impl FusedIterator for WeekendsInInterval {}

/// Iterate over Saturdays and Sundays in [start, end] without allocation.
///
/// Reversed intervals return None. Intervals without weekend dates return
/// an empty iterator. Holidays and locale-specific weekends are not included.
pub fn each_weekend_of_interval(start: NaiveDate, end: NaiveDate) -> Option<WeekendsInInterval> {
    if start > end {
        return None;
    }

    let weekday = start.weekday().num_days_from_monday();
    let first = if weekday >= 5 {
        Some(start)
    } else {
        add_days(start, i64::from(5 - weekday))
    }
    .filter(|date| *date <= end);

    Some(WeekendsInInterval { next: first, end })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn yields_weekend_dates_across_multiple_weeks() {
        let days = each_weekend_of_interval(date(2024, 3, 1), date(2024, 3, 11))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            days,
            vec![
                date(2024, 3, 2),
                date(2024, 3, 3),
                date(2024, 3, 9),
                date(2024, 3, 10),
            ]
        );
    }

    #[test]
    fn handles_single_days_empty_ranges_and_reversed_ranges() {
        let saturday = date(2024, 3, 2);
        let monday = date(2024, 3, 4);
        assert_eq!(
            each_weekend_of_interval(saturday, saturday)
                .unwrap()
                .collect::<Vec<_>>(),
            vec![saturday]
        );
        assert_eq!(each_weekend_of_interval(monday, monday).unwrap().count(), 0);
        assert!(each_weekend_of_interval(monday, saturday).is_none());
    }

    #[test]
    fn is_fused_at_maximum_date() {
        let mut days = each_weekend_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        let expected = match NaiveDate::MAX.weekday() {
            Weekday::Sat | Weekday::Sun => Some(NaiveDate::MAX),
            _ => None,
        };
        assert_eq!(days.next(), expected);
        assert_eq!(days.next(), None);
        assert_eq!(days.next(), None);
    }
}
