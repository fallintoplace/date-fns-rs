//! Lazy partitioning of inclusive datetimes at local calendar midnights.

use std::iter::FusedIterator;

use chrono::{Days, NaiveDateTime};

/// A bidirectional iterator over clipped, inclusive calendar-day segments.
///
/// Each yielded segment covers exactly one date, with original fractional
/// endpoints preserved. Intermediate dates start at midnight and end at the
/// final nanosecond of the day. No time zone is assumed and no days are
/// allocated before iteration.
#[derive(Clone, Debug)]
pub struct DatetimeDaySegments {
    front: Option<NaiveDateTime>,
    back: NaiveDateTime,
}

impl DatetimeDaySegments {
    fn remaining(&self) -> usize {
        self.front.map_or(0, |front| {
            (self
                .back
                .date()
                .signed_duration_since(front.date())
                .num_days()
                + 1) as usize
        })
    }
}

impl Iterator for DatetimeDaySegments {
    type Item = (NaiveDateTime, NaiveDateTime);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.front?;
        let day_end = first.date().and_hms_nano_opt(23, 59, 59, 999_999_999)?;
        let last = day_end.min(self.back);
        self.front = if first.date() == self.back.date() {
            None
        } else {
            first.date().succ_opt()?.and_hms_opt(0, 0, 0)
        };
        Some((first, last))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.remaining() {
            self.front = None;
            return None;
        }
        if n != 0 {
            let date = self
                .front?
                .date()
                .checked_add_days(Days::new(u64::try_from(n).ok()?))?;
            self.front = date.and_hms_opt(0, 0, 0);
        }
        self.next()
    }
}

impl DoubleEndedIterator for DatetimeDaySegments {
    fn next_back(&mut self) -> Option<Self::Item> {
        let first = self.front?;
        let end = self.back;
        let midnight = end.date().and_hms_opt(0, 0, 0)?;
        let start = midnight.max(first);
        if first.date() == end.date() {
            self.front = None;
        } else {
            self.back = end
                .date()
                .pred_opt()?
                .and_hms_nano_opt(23, 59, 59, 999_999_999)?;
        }
        Some((start, end))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.remaining() {
            self.front = None;
            return None;
        }
        if n != 0 {
            let date = self
                .back
                .date()
                .checked_sub_days(Days::new(u64::try_from(n).ok()?))?;
            self.back = date.and_hms_nano_opt(23, 59, 59, 999_999_999)?;
        }
        self.next_back()
    }
}

impl ExactSizeIterator for DatetimeDaySegments {}

impl FusedIterator for DatetimeDaySegments {}

/// Partition an inclusive local datetime range into per-day segments.
///
/// Returns None for reversed endpoints. Interleaved forward/backward
/// iteration and nth skipping use constant-time calendar arithmetic.
pub fn each_day_segment_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<DatetimeDaySegments> {
    (start <= end).then_some(DatetimeDaySegments {
        front: Some(start),
        back: end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(month: u32, day: u32, hour: u32, nanos: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, month, day)
            .unwrap()
            .and_hms_nano_opt(hour, 0, 0, nanos)
            .unwrap()
    }

    #[test]
    fn partitions_leap_day_crossings_without_losing_nanoseconds() {
        let first = at(2, 28, 13, 123);
        let last = at(3, 1, 6, 456);
        let segments: Vec<_> = each_day_segment_of_datetime_interval(first, last)
            .unwrap()
            .collect();
        assert_eq!(
            segments,
            vec![
                (
                    first,
                    at(2, 28, 23, 0)
                        .date()
                        .and_hms_nano_opt(23, 59, 59, 999_999_999)
                        .unwrap()
                ),
                (
                    at(2, 29, 0, 0),
                    at(2, 29, 23, 0)
                        .date()
                        .and_hms_nano_opt(23, 59, 59, 999_999_999)
                        .unwrap()
                ),
                (at(3, 1, 0, 0), last),
            ]
        );
        assert_eq!(segments[0].0, first);
        assert_eq!(segments[2].1, last);
    }

    #[test]
    fn bidirectional_nth_skips_retain_endpoints() {
        let first = at(2, 27, 18, 7);
        let last = at(3, 3, 12, 9);
        let mut segments = each_day_segment_of_datetime_interval(first, last).unwrap();
        assert_eq!(segments.len(), 6);
        assert_eq!(segments.next_back().unwrap().1, last);
        assert_eq!(segments.len(), 5);
        assert_eq!(segments.nth(1).unwrap().0, at(2, 28, 0, 0));
        assert_eq!(segments.len(), 3);
        assert_eq!(segments.nth_back(1).unwrap().0, at(3, 1, 0, 0));
        assert_eq!(segments.len(), 1);
        assert_eq!(segments.next().unwrap().0, at(2, 29, 0, 0));
        assert_eq!(segments.next(), None);
        assert_eq!(segments.next_back(), None);
    }

    #[test]
    fn handles_single_instant_and_chrono_extremes() {
        let mut single =
            each_day_segment_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX).unwrap();
        assert_eq!(single.len(), 1);
        assert_eq!(
            single.next_back(),
            Some((NaiveDateTime::MAX, NaiveDateTime::MAX))
        );
        assert_eq!(single.next(), None);
        let mut huge =
            each_day_segment_of_datetime_interval(NaiveDateTime::MIN, NaiveDateTime::MAX).unwrap();
        assert_eq!(huge.nth(usize::MAX), None);
        assert_eq!(huge.next(), None);
        assert!(
            each_day_segment_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MIN).is_none()
        );
    }

    #[test]
    fn forwards_and_backwards_visit_the_same_segments() {
        let first = at(2, 26, 11, 1);
        let last = at(3, 6, 3, 2);
        let forward: Vec<_> = each_day_segment_of_datetime_interval(first, last)
            .unwrap()
            .collect();
        let backward: Vec<_> = each_day_segment_of_datetime_interval(first, last)
            .unwrap()
            .rev()
            .collect();
        assert_eq!(forward, backward.into_iter().rev().collect::<Vec<_>>());
    }
}
