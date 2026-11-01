//! Lazy calendar-year segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::boundaries::end_of_year;

/// Lazy iterator over clipped, inclusive calendar-year ranges.
#[derive(Clone, Debug)]
pub struct YearSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for YearSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let end = end_of_year(start).min(self.end);
        self.next = if end < self.end { end.succ_opt() } else { None };
        Some((start, end))
    }
}

impl FusedIterator for YearSegmentsInInterval {}

/// Split an inclusive interval into calendar-year segments.
///
/// Returns None for reversed endpoints. Each segment is clipped to the
/// supplied interval, and no intermediate collection is allocated.
pub fn each_year_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<YearSegmentsInInterval> {
    (start <= end).then_some(YearSegmentsInInterval {
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
    fn clips_multi_year_ranges_including_a_leap_year() {
        let segments = each_year_segment_of_interval(date(2023, 12, 30), date(2025, 1, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            segments,
            vec![
                (date(2023, 12, 30), date(2023, 12, 31)),
                (date(2024, 1, 1), date(2024, 12, 31)),
                (date(2025, 1, 1), date(2025, 1, 2)),
            ]
        );
    }

    #[test]
    fn handles_one_day_and_invalid_order() {
        let day = date(2024, 2, 29);
        let mut segments = each_year_segment_of_interval(day, day).unwrap();
        assert_eq!(segments.next(), Some((day, day)));
        assert_eq!(segments.next(), None);
        assert_eq!(segments.next(), None);
        assert!(each_year_segment_of_interval(day, date(2024, 1, 1)).is_none());
    }

    #[test]
    fn stops_at_supported_minimum_and_maximum() {
        let mut first = each_year_segment_of_interval(NaiveDate::MIN, NaiveDate::MIN).unwrap();
        assert_eq!(first.next(), Some((NaiveDate::MIN, NaiveDate::MIN)));
        assert_eq!(first.next(), None);

        let mut last = each_year_segment_of_interval(NaiveDate::MAX, NaiveDate::MAX).unwrap();
        assert_eq!(last.next(), Some((NaiveDate::MAX, NaiveDate::MAX)));
        assert_eq!(last.next(), None);
    }
}
