//! Lazy month-aligned segments of inclusive calendar intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::boundaries::end_of_month;

/// Lazy, non-overlapping inclusive month segments.
#[derive(Clone, Debug)]
pub struct MonthSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for MonthSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let end = end_of_month(start).min(self.end);
        self.next = if end < self.end { end.succ_opt() } else { None };
        Some((start, end))
    }
}

impl FusedIterator for MonthSegmentsInInterval {}

/// Split an inclusive interval into clipped calendar-month ranges.
///
/// Returns `None` for reversed endpoints. The iterator does not allocate
/// an intermediate date collection and handles the maximum supported date.
pub fn each_month_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<MonthSegmentsInInterval> {
    (start <= end).then_some(MonthSegmentsInInterval {
        next: Some(start),
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
    fn clips_partial_months_and_preserves_leap_day() {
        let segments = each_month_segment_of_interval(date(2024, 1, 30), date(2024, 3, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            segments,
            vec![
                (date(2024, 1, 30), date(2024, 1, 31)),
                (date(2024, 2, 1), date(2024, 2, 29)),
                (date(2024, 3, 1), date(2024, 3, 2)),
            ]
        );
    }

    #[test]
    fn handles_one_day_and_reversed_ranges() {
        let day = date(2024, 2, 29);
        let mut segments = each_month_segment_of_interval(day, day).unwrap();
        assert_eq!(segments.next(), Some((day, day)));
        assert_eq!(segments.next(), None);
        assert_eq!(segments.next(), None);
        assert!(each_month_segment_of_interval(day, date(2024, 2, 1)).is_none());
    }

    #[test]
    fn remains_fused_at_supported_limits() {
        let mut last = each_month_segment_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(last.next(), Some((NaiveDate::MAX, NaiveDate::MAX)));
        assert_eq!(last.next(), None);
        assert_eq!(last.next(), None);
    }
}
