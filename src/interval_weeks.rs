//! Lazy calendar week starts in an inclusive date interval.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate, Weekday};

use crate::arithmetic::add_days;
use crate::weeks::start_of_week;

/// An iterator over the first day of each week intersecting an interval.
#[derive(Clone, Debug)]
pub struct WeeksInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
}

impl Iterator for WeeksInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = add_days(current, 7).filter(|next| *next <= self.end);
        Some(current)
    }
}

impl FusedIterator for WeeksInInterval {}

/// Iterate through week starts for every week touching [start, end].
///
/// The first result may be before start, because its week includes start.
/// Returns None for reversed ranges and unrepresentable first boundaries.
pub fn each_week_of_interval(
    start: NaiveDate,
    end: NaiveDate,
    week_starts_on: Weekday,
) -> Option<WeeksInInterval> {
    if start > end {
        return None;
    }
    let first = start_of_week(start, week_starts_on)?;
    Some(WeeksInInterval {
        next: Some(first),
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
    fn includes_the_week_containing_the_first_day() {
        let start = date(2024, 3, 6);
        let end = date(2024, 3, 12);
        let monday = each_week_of_interval(start, end, Weekday::Mon).unwrap();
        assert_eq!(
            monday.collect::<Vec<_>>(),
            vec![date(2024, 3, 4), date(2024, 3, 11)]
        );
        let sunday = each_week_of_interval(start, end, Weekday::Sun).unwrap();
        assert_eq!(
            sunday.collect::<Vec<_>>(),
            vec![date(2024, 3, 3), date(2024, 3, 10)]
        );
    }

    #[test]
    fn single_day_and_reversed_intervals() {
        let day = date(2024, 3, 10);
        let weeks = each_week_of_interval(day, day, Weekday::Mon).unwrap();
        assert_eq!(weeks.collect::<Vec<_>>(), vec![date(2024, 3, 4)]);
        assert!(each_week_of_interval(day, date(2024, 3, 1), Weekday::Mon).is_none());
    }

    #[test]
    fn iterator_is_fused_at_maximum_date() {
        let mut weeks =
            each_week_of_interval(NaiveDate::MAX, NaiveDate::MAX, NaiveDate::MAX.weekday())
                .unwrap();
        assert_eq!(weeks.next(), Some(NaiveDate::MAX));
        assert_eq!(weeks.next(), None);
        assert_eq!(weeks.next(), None);
    }
}
