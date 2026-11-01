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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.map_or(0, |next| {
            (crate::difference::difference_in_calendar_months(self.end, next) / 6 + 1) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let months = u32::try_from(n).ok()?.checked_mul(6)?;
        self.next = self.next?.checked_add_months(Months::new(months));
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(6))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl DoubleEndedIterator for HalfYearsInInterval {
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let months = u32::try_from(n).ok()?.checked_mul(6)?;
        let last_month = if self.end.month() <= 6 { 1 } else { 7 };
        let last = NaiveDate::from_ymd_opt(self.end.year(), last_month, 1)?;
        self.end = last.checked_sub_months(Months::new(months))?;
        self.next_back()
    }

    fn next_back(&mut self) -> Option<Self::Item> {
        let first = self.next?;
        let month = if self.end.month() <= 6 { 1 } else { 7 };
        let last = NaiveDate::from_ymd_opt(self.end.year(), month, 1)?;
        if last == first {
            self.next = None;
        } else {
            self.end = last.pred_opt()?;
        }
        Some(last)
    }
}

impl ExactSizeIterator for HalfYearsInInterval {}

impl FusedIterator for HalfYearsInInterval {}

/// Iterate half-year starts for half-years that intersect [start, end].
///
/// The first returned start may precede start. Reversed intervals return None.
pub fn each_half_year_of_interval(start: NaiveDate, end: NaiveDate) -> Option<HalfYearsInInterval> {
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
    #[test]
    fn supports_exact_size_and_bidirectional_skips() {
        let mut halves = each_half_year_of_interval(date(2023, 10, 15), date(2025, 2, 1)).unwrap();
        assert_eq!(halves.len(), 4);
        assert_eq!(halves.nth_back(1), Some(date(2024, 7, 1)));
        assert_eq!(halves.len(), 2);
        assert_eq!(halves.next(), Some(date(2023, 7, 1)));
        assert_eq!(halves.next_back(), Some(date(2024, 1, 1)));
        assert_eq!(halves.next(), None);

        let mut forwards =
            each_half_year_of_interval(date(2023, 10, 15), date(2025, 2, 1)).unwrap();
        assert_eq!(forwards.nth(2), Some(date(2024, 7, 1)));
        assert_eq!(forwards.len(), 1);
        assert_eq!(forwards.next_back(), Some(date(2025, 1, 1)));

        let mut huge = each_half_year_of_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(huge.nth_back(usize::MAX), None);
        assert_eq!(huge.len(), 0);
    }

}
