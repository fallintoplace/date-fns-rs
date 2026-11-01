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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.map_or(0, |next| {
            (crate::difference::difference_in_calendar_months(self.end, next) + 1) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        if n > 0 {
            let first = crate::boundaries::start_of_month(self.next?);
            self.next = first.checked_add_months(chrono::Months::new(n as u32));
        }
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let end = end_of_month(start).min(self.end);
        self.next = if end < self.end { end.succ_opt() } else { None };
        Some((start, end))
    }
}

impl ExactSizeIterator for MonthSegmentsInInterval {}

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

    #[test]
    fn counts_and_skips_clipped_month_segments_directly() {
        let mut segments =
            each_month_segment_of_interval(date(2024, 1, 30), date(2024, 5, 2)).unwrap();
        assert_eq!(segments.len(), 5);
        assert_eq!(segments.nth(2), Some((date(2024, 3, 1), date(2024, 3, 31))));
        assert_eq!(segments.len(), 2);
        assert_eq!(segments.next(), Some((date(2024, 4, 1), date(2024, 4, 30))));
        assert_eq!(segments.next(), Some((date(2024, 5, 1), date(2024, 5, 2))));
        assert_eq!(segments.len(), 0);

        let mut extreme = each_month_segment_of_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(extreme.nth(usize::MAX), None);
        assert_eq!(extreme.next(), None);
    }
}
