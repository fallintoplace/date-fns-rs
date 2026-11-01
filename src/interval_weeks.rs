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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.map_or(0, |next| {
            (crate::difference::difference_in_calendar_days(self.end, next) / 7 + 1) as usize
        });
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let offset = i64::try_from(n).ok()?.checked_mul(7)?;
        self.next = add_days(self.next?, offset);
        self.next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = add_days(current, 7).filter(|next| *next <= self.end);
        Some(current)
    }
}

impl ExactSizeIterator for WeeksInInterval {}

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
    #[test]
    fn reports_exact_week_count_and_skips_directly() {
        let mut weeks =
            each_week_of_interval(date(2024, 3, 1), date(2024, 4, 3), Weekday::Mon).unwrap();
        assert_eq!(weeks.len(), 6);
        assert_eq!(weeks.nth(3), Some(date(2024, 3, 18)));
        assert_eq!(weeks.len(), 2);
        assert_eq!(weeks.next(), Some(date(2024, 3, 25)));
        assert_eq!(weeks.next(), Some(date(2024, 4, 1)));
        assert_eq!(weeks.next(), None);
        assert_eq!(weeks.len(), 0);

        let mut extreme =
            each_week_of_interval(NaiveDate::MIN, NaiveDate::MAX, NaiveDate::MIN.weekday())
                .unwrap();
        assert_eq!(extreme.nth(usize::MAX), None);
        assert_eq!(extreme.next(), None);
    }

}
