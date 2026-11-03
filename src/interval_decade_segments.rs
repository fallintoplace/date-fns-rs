//! Lazy Gregorian decade segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

/// Lazy iterator over clipped, inclusive calendar decades.
#[derive(Clone, Debug)]
pub struct DecadeSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for DecadeSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        // A period boundary outside Chrono's range cannot cut the interval.
        let last = crate::end_of_decade(start)
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

impl FusedIterator for DecadeSegmentsInInterval {}

/// Split an inclusive date interval into clipped calendar decades.
///
/// Decades start in years divisible by ten, including year zero.
/// Returns None for reversed endpoints and never allocates a collection.
pub fn each_decade_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<DecadeSegmentsInInterval> {
    (start <= end).then_some(DecadeSegmentsInInterval {
        next: Some(start),
        end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn clips_decades_and_preserves_order() {
        let ranges = each_decade_segment_of_interval(day(2019, 12, 30), day(2030, 1, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2019, 12, 30), day(2019, 12, 31)),
                (day(2020, 1, 1), day(2029, 12, 31)),
                (day(2030, 1, 1), day(2030, 1, 2)),
            ]
        );
    }

    #[test]
    fn splits_across_year_zero() {
        let ranges = each_decade_segment_of_interval(day(-1, 12, 30), day(0, 1, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(-1, 12, 30), day(-1, 12, 31)),
                (day(0, 1, 1), day(0, 1, 2)),
            ]
        );
    }

    #[test]
    fn handles_reversed_and_boundary_dates() {
        assert!(each_decade_segment_of_interval(day(2025, 1, 1), day(2024, 1, 1)).is_none());
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut iter = each_decade_segment_of_interval(edge, edge).unwrap();
            assert_eq!(iter.next(), Some((edge, edge)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
    }
}
