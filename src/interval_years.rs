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

    fn size_hint(&self) -> (usize, Option<usize>) {
        use chrono::Datelike;

        let remaining = self.next.map_or(0, |next| {
            (i64::from(self.end.year()) - i64::from(next.year()) + 1) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let months = u32::try_from(n).ok()?.checked_mul(12)?;
        self.next = self.next?.checked_add_months(Months::new(months));
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .checked_add_months(Months::new(12))
            .filter(|next| *next <= self.end);
        Some(current)
    }
}

impl DoubleEndedIterator for YearsInInterval {
    fn next_back(&mut self) -> Option<Self::Item> {
        let front = self.next?;
        let last = start_of_year(self.end);
        if last == front {
            self.next = None;
        } else {
            self.end = last.pred_opt()?;
        }
        Some(last)
    }
}

impl ExactSizeIterator for YearsInInterval {}

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
    #[test]
    fn skips_year_starts_and_reports_exact_remaining_count() {
        let mut years = each_year_of_interval(date(2022, 12, 31), date(2025, 1, 2)).unwrap();
        assert_eq!(years.len(), 4);
        assert_eq!(years.nth(2), Some(date(2024, 1, 1)));
        assert_eq!(years.len(), 1);
        assert_eq!(years.next(), Some(date(2025, 1, 1)));
        assert_eq!(years.len(), 0);
    }

    #[test]
    fn exhausts_impossible_nth_calls_at_range_limits() {
        let mut years = each_year_of_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(years.nth(usize::MAX), None);
        assert_eq!(years.next(), None);
    }

    #[test]
    fn supports_reverse_and_interleaved_year_traversal() {
        let mut years = each_year_of_interval(date(2021, 12, 31), date(2025, 1, 2)).unwrap();
        assert_eq!(years.len(), 5);
        assert_eq!(years.next_back(), Some(date(2025, 1, 1)));
        assert_eq!(years.next(), Some(date(2021, 1, 1)));
        assert_eq!(years.next_back(), Some(date(2024, 1, 1)));
        assert_eq!(years.next(), Some(date(2022, 1, 1)));
        assert_eq!(years.next_back(), Some(date(2023, 1, 1)));
        assert_eq!(years.next(), None);
        assert_eq!(years.len(), 0);

        let mut edge = each_year_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(edge.next_back(), Some(start_of_year(NaiveDate::MAX)));
        assert_eq!(edge.next_back(), None);
    }
}
