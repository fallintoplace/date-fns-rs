//! Lazy inclusive week-year segments with configurable calendar conventions.

use std::iter::FusedIterator;

use chrono::{NaiveDate, Weekday};

/// Lazy iterator over clipped, inclusive configurable week-years.
#[derive(Clone, Debug)]
pub struct WeekYearSegmentsInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
}

impl Iterator for WeekYearSegmentsInInterval {
    type Item = (NaiveDate, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next?;
        let last = crate::end_of_week_year(
            start,
            self.week_starts_on,
            self.first_week_contains_date,
        )
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

impl FusedIterator for WeekYearSegmentsInInterval {}

/// Split an inclusive date interval at configurable week-year boundaries.
///
/// Use Monday and 4 for ISO years, or Sunday and 1 for US-style years.
/// The first-week anchor must be in 1..=7. Returns None for invalid rules
/// or reversed endpoints; period ends are clipped to Chrono's date limits.
pub fn each_week_year_segment_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<WeekYearSegmentsInInterval> {
    if start > end || !(1..=7).contains(&first_week_contains_date) {
        return None;
    }
    crate::get_week_year(start, week_starts_on, first_week_contains_date)?;
    crate::get_week_year(end, week_starts_on, first_week_contains_date)?;
    Some(WeekYearSegmentsInInterval {
        next: Some(start),
        end,
        week_starts_on,
        first_week_contains_date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn splits_iso_week_years_after_january_third() {
        let ranges = each_week_year_segment_of_interval(
            day(2020, 12, 31),
            day(2021, 1, 5),
            Weekday::Mon,
            4,
        )
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
    fn splits_us_style_years_in_december() {
        let ranges = each_week_year_segment_of_interval(
            day(2020, 12, 26),
            day(2021, 1, 2),
            Weekday::Sun,
            1,
        )
        .unwrap()
        .collect::<Vec<_>>();
        assert_eq!(
            ranges,
            vec![
                (day(2020, 12, 26), day(2020, 12, 26)),
                (day(2020, 12, 27), day(2021, 1, 2)),
            ]
        );
    }

    #[test]
    fn validates_rules_and_fuses_at_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 2, 1);
        assert!(each_week_year_segment_of_interval(last, first, Weekday::Mon, 4).is_none());
        assert!(each_week_year_segment_of_interval(first, last, Weekday::Mon, 0).is_none());
        assert!(each_week_year_segment_of_interval(first, last, Weekday::Mon, 8).is_none());
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let mut iter =
                each_week_year_segment_of_interval(edge, edge, Weekday::Mon, 4).unwrap();
            assert_eq!(iter.next(), Some((edge, edge)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
    }
}
