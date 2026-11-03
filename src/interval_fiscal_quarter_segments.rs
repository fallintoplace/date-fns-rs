//! Lazy, inclusive date-interval segments aligned to fiscal quarters.

use std::iter::FusedIterator;

use chrono::{Datelike, Months, NaiveDate};

/// Lazy iterator over clipped, inclusive fiscal quarters.
#[derive(Clone, Debug)]
pub struct FiscalQuarterSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
    first_month: u32,
}

impl Iterator for FiscalQuarterSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let offset = (start.month() + 12 - self.first_month) % 12;
        let months = 3 - offset % 3;
        let next_period = NaiveDate::from_ymd_opt(start.year(), start.month(), 1)
            .and_then(|first| first.checked_add_months(Months::new(months)));
        let last = next_period
            .and_then(|next| next.pred_opt())
            .unwrap_or(NaiveDate::MAX)
            .min(self.end);
        self.next = if last < self.end { last.succ_opt() } else { None };
        Some((start, last))
    }
}

impl FusedIterator for FiscalQuarterSegmentsInInterval {}

/// Split an inclusive interval into fiscal quarters.
///
/// `first_month` is the fiscal year start (1 through 12). Returns None
/// for invalid months or reversed endpoints. The iterator allocates no
/// intermediate collection and clips periods to Chrono's supported range.
pub fn each_fiscal_quarter_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    first_month: u32,
) -> Option<FiscalQuarterSegmentsInInterval> {
    if start > end || !(1..=12).contains(&first_month) {
        return None;
    }
    Some(FiscalQuarterSegmentsInInterval {
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
    fn clips_february_based_quarters_across_new_year() {
        let ranges = each_fiscal_quarter_segment_of_interval(
            day(2024, 1, 30),
            day(2024, 5, 2),
            2,
        )
        .unwrap()
        .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2024, 1, 30), day(2024, 1, 31)),
                (day(2024, 2, 1), day(2024, 4, 30)),
                (day(2024, 5, 1), day(2024, 5, 2)),
            ]
        );
    }

    #[test]
    fn rejects_invalid_months_and_reversed_ranges() {
        let first = day(2024, 1, 1);
        let last = day(2024, 2, 29);
        assert!(each_fiscal_quarter_segment_of_interval(last, first, 4).is_none());
        assert!(each_fiscal_quarter_segment_of_interval(first, last, 0).is_none());
        assert!(each_fiscal_quarter_segment_of_interval(first, last, 13).is_none());
    }

    #[test]
    fn is_fused_at_both_chrono_limits() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut ranges = each_fiscal_quarter_segment_of_interval(date, date, 2).unwrap();
            assert_eq!(ranges.next(), Some((date, date)));
            assert_eq!(ranges.next(), None);
            assert_eq!(ranges.next(), None);
        }
    }
}
