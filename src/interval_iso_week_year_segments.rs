//! Lazy ISO week-year segments of inclusive date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

/// Lazy iterator over clipped, inclusive ISO week-numbering years.
#[derive(Clone, Debug)]
pub struct IsoWeekYearSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for IsoWeekYearSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        // An unrepresentable ISO year-end extends beyond Chrono's limit.
        let last = crate::end_of_iso_week_year(start)
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

impl FusedIterator for IsoWeekYearSegmentsInInterval {}

/// Split an inclusive date interval at ISO week-year boundaries.
///
/// ISO years start on Monday, and their first week contains January 4.
/// Reversed endpoints return None. Each range is clipped to the input.
pub fn each_iso_week_year_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
) -> Option<IsoWeekYearSegmentsInInterval> {
    (start <= end).then_some(IsoWeekYearSegmentsInInterval {
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
    fn splits_a_53_week_year_after_january_third() {
        let ranges = each_iso_week_year_segment_of_interval(day(2020, 12, 31), day(2021, 1, 5))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2020, 12, 31), day(2021, 1, 3)),
                (day(2021, 1, 4), day(2021, 1, 5)),
            ]
        );
    }

    #[test]
    fn does_not_split_at_december_31_when_iso_year_is_unchanged() {
        let ranges = each_iso_week_year_segment_of_interval(day(2020, 12, 31), day(2021, 1, 1))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(ranges, vec![(day(2020, 12, 31), day(2021, 1, 1))]);
    }

    #[test]
    fn rejects_reversed_ranges_and_fuses_at_limits() {
        assert!(each_iso_week_year_segment_of_interval(day(2024, 2, 1), day(2024, 1, 1)).is_none());
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut iter = each_iso_week_year_segment_of_interval(edge, edge).unwrap();
            assert_eq!(iter.next(), Some((edge, edge)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
    }
}
