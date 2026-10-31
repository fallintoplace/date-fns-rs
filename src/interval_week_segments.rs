//! Lazy, clipped calendar-week segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::{NaiveDate, Weekday};

use crate::weeks::end_of_week;

/// Lazy iterator over inclusive week segments.
#[derive(Clone, Debug)]
pub struct WeekSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
    week_starts_on: Weekday,
}

impl Iterator for WeekSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        // If the week end exceeds Chrono's range, the segment must end
        // at the caller's representable interval boundary.
        let end = end_of_week(start, self.week_starts_on)
            .unwrap_or(self.end)
            .min(self.end);
        self.next = if end < self.end { end.succ_opt() } else { None };
        Some((start, end))
    }
}

impl FusedIterator for WeekSegmentsInInterval {}

/// Split an inclusive date interval into clipped week-aligned ranges.
///
/// Weeks start on `week_starts_on`; the first and last segments can be
/// shorter than seven days. Reversed intervals return None.
pub fn each_week_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    week_starts_on: Weekday,
) -> Option<WeekSegmentsInInterval> {
    (start <= end).then_some(WeekSegmentsInInterval {
        next: Some(start),
        end,
        week_starts_on,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn splits_monday_first_weeks_without_losing_dates() {
        let segments =
            each_week_segment_of_interval(date(2024, 3, 1), date(2024, 3, 12), Weekday::Mon)
                .unwrap()
                .collect::<Vec<_>>();
        assert_eq!(
            segments,
            vec![
                (date(2024, 3, 1), date(2024, 3, 3)),
                (date(2024, 3, 4), date(2024, 3, 10)),
                (date(2024, 3, 11), date(2024, 3, 12)),
            ]
        );
    }

    #[test]
    fn supports_sunday_first_weeks() {
        let segments =
            each_week_segment_of_interval(date(2024, 3, 1), date(2024, 3, 12), Weekday::Sun)
                .unwrap()
                .collect::<Vec<_>>();
        assert_eq!(
            segments,
            vec![
                (date(2024, 3, 1), date(2024, 3, 2)),
                (date(2024, 3, 3), date(2024, 3, 9)),
                (date(2024, 3, 10), date(2024, 3, 12)),
            ]
        );
    }

    #[test]
    fn handles_reverse_order_and_chrono_limits() {
        let day = date(2024, 3, 1);
        assert!(each_week_segment_of_interval(day, date(2024, 2, 29), Weekday::Mon).is_none());
        let mut last =
            each_week_segment_of_interval(NaiveDate::MAX, NaiveDate::MAX, NaiveDate::MAX.weekday())
                .unwrap();
        assert_eq!(last.next(), Some((NaiveDate::MAX, NaiveDate::MAX)));
        assert_eq!(last.next(), None);
        assert_eq!(last.next(), None);
    }
}
