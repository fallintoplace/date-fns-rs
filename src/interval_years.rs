//! Lazy first-of-year dates for inclusive calendar intervals.

use std::iter::FusedIterator;

use chrono::{Months, NaiveDate};

use crate::boundaries::start_of_year;

/// A lazy iterator over year starts in an inclusive interval.
#[derive(Clone, Debug)]
pub struct YearsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for YearsInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(12))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for YearsInInterval {}

/// Iterate through January 1 dates for years touching [start, end].
///
/// The first result can precede start. Reversed intervals return None.
pub fn each_year_of_interval(start: NaiveDate, end: NaiveDate) -> Option<YearsInInterval> {
    (start <= end).then_some(YearsInInterval {
        next: Some(start_of_year(start)),
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
    fn includes_years_overlapping_both_endpoints() {
        let years = each_year_of_interval(date(2023, 12, 31), date(2025, 1, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            years,
            vec![date(2023, 1, 1), date(2024, 1, 1), date(2025, 1, 1)]
        );
    }

    #[test]
    fn handles_zero_length_and_reversed_intervals() {
        let day = date(2024, 6, 1);
        let years = each_year_of_interval(day, day).unwrap();
        assert_eq!(years.collect::<Vec<_>>(), vec![date(2024, 1, 1)]);
        assert!(each_year_of_interval(day, date(2023, 1, 1)).is_none());
    }

    #[test]
    fn terminates_without_overflow_at_date_limits() {
        let mut years = each_year_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(years.next(), Some(start_of_year(NaiveDate::MAX)));
        assert_eq!(years.next(), None);
        assert_eq!(years.next(), None);
    }
}
