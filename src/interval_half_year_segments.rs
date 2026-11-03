//! Lazy calendar half-year segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::half_years::end_of_half_year;

/// Lazy iterator over clipped, inclusive calendar half-year ranges.
#[derive(Clone, Debug)]
pub struct HalfYearSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for HalfYearSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let last = end_of_half_year(start).min(self.end);
        self.next = if last < self.end { last.succ_opt() } else { None };
        Some((start, last))
    }
}

impl FusedIterator for HalfYearSegmentsInInterval {}

/// Split an inclusive interval into clipped calendar half-year ranges.
///
/// Returns None for reversed endpoints. The iterator does not allocate
/// intermediate dates and handles the supported calendar limits.
pub fn each_half_year_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<HalfYearSegmentsInInterval> {
    (start <= end).then_some(HalfYearSegmentsInInterval {
        next: Some(start),
        end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn clips_halves_across_new_year() {
        let ranges = each_half_year_segment_of_interval(
            day(2024, 6, 28),
            day(2025, 1, 2),
        )
        .unwrap()
        .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2024, 6, 28), day(2024, 6, 30)),
                (day(2024, 7, 1), day(2024, 12, 31)),
                (day(2025, 1, 1), day(2025, 1, 2)),
            ]
        );
    }

    #[test]
    fn is_fused_at_bounds_and_rejects_reversed_intervals() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut ranges = each_half_year_segment_of_interval(date, date).unwrap();
            assert_eq!(ranges.next(), Some((date, date)));
            assert_eq!(ranges.next(), None);
            assert_eq!(ranges.next(), None);
        }
        assert!(each_half_year_segment_of_interval(
            day(2024, 7, 1),
            day(2024, 6, 30)
        )
        .is_none());
    }
}
