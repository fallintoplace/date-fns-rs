//! Lazy first-of-quarter dates for inclusive calendar intervals.

use std::iter::FusedIterator;

use chrono::{Months, NaiveDate};

use crate::quarters::start_of_quarter;

/// A lazy iterator over quarter starts in an inclusive interval.
#[derive(Clone, Debug)]
pub struct QuartersInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for QuartersInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(3))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for QuartersInInterval {}

/// Iterate through quarter starts for quarters overlapping [start, end].
///
/// The first returned quarter start may precede start. Reversed intervals
/// return None. Iteration does not allocate an intermediate collection.
pub fn each_quarter_of_interval(start: NaiveDate, end: NaiveDate) -> Option<QuartersInInterval> {
    (start <= end).then_some(QuartersInInterval {
        next: Some(start_of_quarter(start)),
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
    fn visits_quarter_starts_across_years() {
        let quarters = each_quarter_of_interval(date(2023, 12, 31), date(2024, 7, 1))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            quarters,
            vec![
                date(2023, 10, 1),
                date(2024, 1, 1),
                date(2024, 4, 1),
                date(2024, 7, 1)
            ]
        );
    }

    #[test]
    fn handles_single_date_and_reversed_intervals() {
        let leap_day = date(2024, 2, 29);
        let values = each_quarter_of_interval(leap_day, leap_day)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(values, vec![date(2024, 1, 1)]);
        assert!(each_quarter_of_interval(leap_day, date(2024, 1, 1)).is_none());
    }

    #[test]
    fn stops_at_maximum_without_wrapping() {
        let mut quarters = each_quarter_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(quarters.next(), Some(start_of_quarter(NaiveDate::MAX)));
        assert_eq!(quarters.next(), None);
        assert_eq!(quarters.next(), None);
    }
}
