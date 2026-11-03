//! Lazy Gregorian century segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

/// Lazy iterator over clipped, inclusive Gregorian centuries.
#[derive(Clone, Debug)]
pub struct CenturySegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for CenturySegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        // At the upper supported limit, the century may have no representable end.
        let last = crate::end_of_century(start)
            .unwrap_or(self.end)
            .min(self.end);
        self.next = if last < self.end {
            last.succ_opt()
        } else {
            None
        };
        Some((start, last))
    }
}

impl FusedIterator for CenturySegmentsInInterval {}

/// Split an inclusive interval into clipped, one-based Gregorian centuries.
///
/// Years 2001 through 2100 form a century; year zero belongs with -99
/// through zero. Reversed intervals return None.
pub fn each_century_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<CenturySegmentsInInterval> {
    (start <= end).then_some(CenturySegmentsInInterval {
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
    fn clips_one_based_centuries() {
        let ranges = each_century_segment_of_interval(day(2000, 12, 30), day(2101, 1, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2000, 12, 30), day(2000, 12, 31)),
                (day(2001, 1, 1), day(2100, 12, 31)),
                (day(2101, 1, 1), day(2101, 1, 2)),
            ]
        );
    }

    #[test]
    fn year_zero_and_one_are_different_centuries() {
        let ranges = each_century_segment_of_interval(day(0, 12, 31), day(1, 1, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(0, 12, 31), day(0, 12, 31)),
                (day(1, 1, 1), day(1, 1, 2)),
            ]
        );
    }

    #[test]
    fn validates_order_and_fuses_at_calendar_limits() {
        assert!(each_century_segment_of_interval(day(2025, 1, 1), day(2024, 1, 1)).is_none());
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut iter = each_century_segment_of_interval(edge, edge).unwrap();
            assert_eq!(iter.next(), Some((edge, edge)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
    }
}
