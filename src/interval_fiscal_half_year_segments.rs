//! Lazy, inclusive date-interval segments aligned to fiscal half-years.

use std::iter::FusedIterator;

use chrono::{Datelike, Months, NaiveDate};

/// Lazy iterator over clipped, inclusive fiscal half-years.
#[derive(Clone, Debug)]
pub struct FiscalHalfYearSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
    first_month: u32,
}

impl Iterator for FiscalHalfYearSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let offset = (start.month() + 12 - self.first_month) % 12;
        let months = 6 - offset % 6;
        let next_period = NaiveDate::from_ymd_opt(start.year(), start.month(), 1)
            .and_then(|first| first.checked_add_months(Months::new(months)));
        let last = next_period
            .and_then(|next| next.pred_opt())
            .unwrap_or(NaiveDate::MAX)
            .min(self.end);
        self.next = if last < self.end {
            last.succ_opt()
        } else {
            None
        };
        Some((start, last))
    }
}

impl FusedIterator for FiscalHalfYearSegmentsInInterval {}

/// Split an inclusive interval into fiscal half-years.
///
/// `first_month` is the fiscal year start (1 through 12). Returns None
/// for invalid months or reversed endpoints. The iterator allocates no
/// intermediate collection and clips periods to Chrono's supported range.
pub fn each_fiscal_half_year_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    first_month: u32,
) -> Option<FiscalHalfYearSegmentsInInterval> {
    if start > end || !(1..=12).contains(&first_month) {
        return None;
    }
    Some(FiscalHalfYearSegmentsInInterval {
        next: Some(start),
        end,
        first_month,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn clips_april_based_halves_across_new_year() {
        let ranges =
            each_fiscal_half_year_segment_of_interval(day(2023, 9, 28), day(2024, 4, 2), 4)
                .unwrap()
                .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2023, 9, 28), day(2023, 9, 30)),
                (day(2023, 10, 1), day(2024, 3, 31)),
                (day(2024, 4, 1), day(2024, 4, 2)),
            ]
        );
    }

    #[test]
    fn rejects_invalid_months_and_reversed_ranges() {
        let first = day(2024, 1, 1);
        let last = day(2024, 2, 29);
        assert!(each_fiscal_half_year_segment_of_interval(last, first, 4).is_none());
        assert!(each_fiscal_half_year_segment_of_interval(first, last, 0).is_none());
        assert!(each_fiscal_half_year_segment_of_interval(first, last, 13).is_none());
    }

    #[test]
    fn is_fused_at_both_chrono_limits() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut ranges = each_fiscal_half_year_segment_of_interval(date, date, 2).unwrap();
            assert_eq!(ranges.next(), Some((date, date)));
            assert_eq!(ranges.next(), None);
            assert_eq!(ranges.next(), None);
        }
    }
}
