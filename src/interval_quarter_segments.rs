//! Lazy quarter-aligned segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::quarters::end_of_quarter;

/// Lazy iterator over inclusive calendar-quarter segments.
#[derive(Clone, Debug)]
pub struct QuarterSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for QuarterSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let end = end_of_quarter(start).min(self.end);
        self.next = if end < self.end { end.succ_opt() } else { None };
        Some((start, end))
    }
}

impl FusedIterator for QuarterSegmentsInInterval {}

/// Split an inclusive interval into clipped calendar-quarter ranges.
///
/// Reversed endpoints return None. The iterator allocates no date collection
/// and never advances beyond Chrono's supported range.
pub fn each_quarter_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<QuarterSegmentsInInterval> {
    (start <= end).then_some(QuarterSegmentsInInterval {
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
    fn clips_quarters_across_calendar_years_and_leap_days() {
        let segments = each_quarter_segment_of_interval(date(2023, 12, 29), date(2024, 4, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            segments,
            vec![
                (date(2023, 12, 29), date(2023, 12, 31)),
                (date(2024, 1, 1), date(2024, 3, 31)),
                (date(2024, 4, 1), date(2024, 4, 2)),
            ]
        );
    }

    #[test]
    fn preserves_single_dates_and_rejects_reversed_inputs() {
        let day = date(2024, 2, 29);
        let mut quarter = each_quarter_segment_of_interval(day, day).unwrap();
        assert_eq!(quarter.next(), Some((day, day)));
        assert_eq!(quarter.next(), None);
        assert_eq!(quarter.next(), None);
        assert!(each_quarter_segment_of_interval(day, date(2024, 1, 1)).is_none());
    }

    #[test]
    fn is_fused_at_both_supported_extremes() {
        let mut first = each_quarter_segment_of_interval(NaiveDate::MIN, NaiveDate::MIN).unwrap();
        assert_eq!(first.next(), Some((NaiveDate::MIN, NaiveDate::MIN)));
        assert_eq!(first.next(), None);

        let mut last = each_quarter_segment_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(last.next(), Some((NaiveDate::MAX, NaiveDate::MAX)));
        assert_eq!(last.next(), None);
        assert_eq!(last.next(), None);
    }
}
