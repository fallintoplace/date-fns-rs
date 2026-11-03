//! Lazy calendar-week segments of inclusive naive datetime intervals.

use std::iter::FusedIterator;

use chrono::{NaiveDate, NaiveDateTime, Weekday};

/// Non-overlapping inclusive datetime segments clipped at week boundaries.
#[derive(Clone, Debug)]
pub struct DatetimeWeekSegments {
    inner: crate::WeekSegmentsInInterval,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl DatetimeWeekSegments {
    fn clip(&self, dates: (NaiveDate, NaiveDate)) -> (NaiveDateTime, NaiveDateTime) {
        let first = dates
            .0
            .and_hms_opt(0, 0, 0)
            .expect("midnight of a valid date must exist")
            .max(self.start);
        let last = dates
            .1
            .and_hms_nano_opt(23, 59, 59, 999_999_999)
            .expect("end of a valid date must exist")
            .min(self.end);
        (first, last)
    }
}

impl Iterator for DatetimeWeekSegments {
    type Item = (NaiveDateTime, NaiveDateTime);

    fn next(&mut self) -> Option<Self::Item> {
        let dates = self.inner.next()?;
        Some(self.clip(dates))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let dates = self.inner.nth(n)?;
        Some(self.clip(dates))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl FusedIterator for DatetimeWeekSegments {}

/// Partition an inclusive datetime interval by week without losing timestamps.
///
/// Partial first and last segments are clipped to the exact input datetimes.
/// Returns None for reversed ranges and never allocates an intermediate list.
pub fn each_week_segment_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime, week_starts_on: Weekday,
) -> Option<DatetimeWeekSegments> {
    if start > end {
        return None;
    }
    let inner = crate::each_week_segment_of_interval(start.date(), end.date(), week_starts_on)?;
    Some(DatetimeWeekSegments { inner, start, end })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn at(y: i32, m: u32, d: u32, h: u32) -> NaiveDateTime {
        day(y, m, d)
            .and_hms_nano_opt(h, 15, 30, 123_456_789)
            .unwrap()
    }

    #[test]
    fn clips_partial_segments_preserving_nanoseconds() {
        let start = at(2024, 3, 1, 13);
        let end = at(2024, 3, 12, 6);
        let segments: Vec<_> = each_week_segment_of_datetime_interval(start, end, Weekday::Mon).unwrap().collect();

        assert_eq!(
            segments.iter().map(|(first, last)| (first.date(), last.date())).collect::<Vec<_>>(),
            vec![
                (day(2024, 3, 1), day(2024, 3, 3)),
                (day(2024, 3, 4), day(2024, 3, 10)),
                (day(2024, 3, 11), day(2024, 3, 12))
            ]
        );
        assert_eq!(segments[0].0, start);
        assert_eq!(segments[2].1, end);
        assert_eq!(segments[1].0, day(2024, 3, 4).and_hms_opt(0, 0, 0).unwrap());
        assert_eq!(
            segments[1].1,
            day(2024, 3, 10).and_hms_nano_opt(23, 59, 59, 999_999_999).unwrap()
        );
        for adjacent in segments.windows(2) {
            assert_eq!(
                adjacent[0].1.checked_add_signed(TimeDelta::nanoseconds(1)),
                Some(adjacent[1].0)
            );
        }
    }

    #[test]
    fn rejects_reversed_ranges_and_exhausts_at_supported_limits() {
        let first = at(2024, 3, 1, 13);
        let last = at(2024, 3, 12, 6);
        assert!(each_week_segment_of_datetime_interval(last, first, Weekday::Mon).is_none());
        let mut end = each_week_segment_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, Weekday::Mon).unwrap();
        assert_eq!(end.next(), Some((NaiveDateTime::MAX, NaiveDateTime::MAX)));
        assert_eq!(end.next(), None);
        assert_eq!(end.next(), None);
        let mut beginning = each_week_segment_of_datetime_interval(NaiveDateTime::MIN, NaiveDateTime::MIN, Weekday::Mon).unwrap();
        assert_eq!(beginning.next(), Some((NaiveDateTime::MIN, NaiveDateTime::MIN)));
        assert_eq!(beginning.next(), None);
    }

    #[test]
    fn supports_sunday_first_weeks() {
        let start = at(2024, 3, 1, 13);
        let end = at(2024, 3, 12, 6);
        let dates: Vec<_> = each_week_segment_of_datetime_interval(start, end, Weekday::Sun)
            .unwrap()
            .map(|(first, last)| (first.date(), last.date()))
            .collect();
        assert_eq!(
            dates,
            vec![
                (day(2024, 3, 1), day(2024, 3, 2)),
                (day(2024, 3, 3), day(2024, 3, 9)),
                (day(2024, 3, 10), day(2024, 3, 12))
            ]
        );
    }
}
