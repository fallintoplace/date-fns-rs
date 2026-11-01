//! Lazy calendar half-year starts over inclusive date intervals.

use std::iter::FusedIterator;

use chrono::{Datelike, Months, NaiveDate};

/// Lazy half-year starts for half-years overlapping an inclusive interval.
#[derive(Clone, Debug)]
pub struct HalfYearsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for HalfYearsInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(6))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for HalfYearsInInterval {}

/// Iterate half-year starts for half-years that intersect [start, end].
///
/// The first returned start may precede start. Reversed intervals return None.
pub fn each_half_year_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<HalfYearsInInterval> {
    if start > end {
        return None;
    }
    let month = if start.month() <= 6 { 1 } else { 7 };
    let first = NaiveDate::from_ymd_opt(start.year(), month, 1)?;
    Some(HalfYearsInInterval {
        next: Some(first),
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
    fn crosses_year_and_leap_boundaries() {
        let starts = each_half_year_of_interval(date(2023, 10, 15), date(2025, 2, 1))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            starts,
            vec![
                date(2023, 7, 1),
                date(2024, 1, 1),
                date(2024, 7, 1),
                date(2025, 1, 1)
            ]
        );
    }

    #[test]
    fn handles_single_date_reversed_and_extreme_ranges() {
        let mut starts = each_half_year_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(starts.next(), Some(date(NaiveDate::MAX.year(), 7, 1)));
        assert_eq!(starts.next(), None);
        assert_eq!(starts.next(), None);
        assert!(each_half_year_of_interval(date(2024, 7, 1), date(2024, 6, 30)).is_none());
    }
}
