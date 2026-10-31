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

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(1))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

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
}
