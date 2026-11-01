//! Lazy monthly recurrence for positive or negative weekday occurrences.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate, Weekday};

use crate::boundaries::start_of_month;
use crate::months::add_months;
use crate::nth_weekday::nth_weekday_of_month;

/// Calendar dates matching an nth-weekday rule across successive months.
#[derive(Clone, Debug)]
pub struct MonthlyWeekdayOccurrences {
    next_month: Option<NaiveDate>,
    start: NaiveDate,
    end: NaiveDate,
    weekday: Weekday,
    occurrence: i8,
}

impl Iterator for MonthlyWeekdayOccurrences {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(month) = self.next_month {
            self.next_month = add_months(month, 1).filter(|next| *next <= self.end);
            if let Some(candidate) =
                nth_weekday_of_month(month.year(), month.month(), self.weekday, self.occurrence)
            {
                if candidate >= self.start && candidate <= self.end {
                    return Some(candidate);
                }
            }
        }
        None
    }
}

impl FusedIterator for MonthlyWeekdayOccurrences {}

/// Iterate monthly nth-weekday occurrences in an inclusive date interval.
///
/// Positive occurrences count from the start of each month; negative ones
/// count from the end. Missing fifth weekdays are skipped. The occurrence
/// must be from -5 through -1 or 1 through 5.
pub fn each_nth_weekday_of_month_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    weekday: Weekday,
    occurrence: i8,
) -> Option<MonthlyWeekdayOccurrences> {
    if start > end || occurrence == 0 || !(-5..=5).contains(&occurrence) {
        return None;
    }
    Some(MonthlyWeekdayOccurrences {
        next_month: Some(start_of_month(start)),
        start,
        end,
        weekday,
        occurrence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn yields_last_friday_across_month_boundaries() {
        let dates: Vec<_> = each_nth_weekday_of_month_in_interval(
            date(2024, 1, 20),
            date(2024, 4, 5),
            Weekday::Fri,
            -1,
        )
        .unwrap()
        .collect();
        assert_eq!(
            dates,
            vec![date(2024, 1, 26), date(2024, 2, 23), date(2024, 3, 29)]
        );
    }

    #[test]
    fn skips_months_without_a_fifth_weekday() {
        let dates: Vec<_> = each_nth_weekday_of_month_in_interval(
            date(2024, 1, 1),
            date(2024, 5, 31),
            Weekday::Thu,
            5,
        )
        .unwrap()
        .collect();
        assert_eq!(dates, vec![date(2024, 2, 29), date(2024, 5, 30)]);
    }

    #[test]
    fn rejects_invalid_ranges_and_fuses_at_calendar_limit() {
        let day = date(2024, 3, 1);
        assert!(each_nth_weekday_of_month_in_interval(day, day, Weekday::Fri, 0).is_none());
        assert!(each_nth_weekday_of_month_in_interval(day, day, Weekday::Fri, 6).is_none());
        assert!(
            each_nth_weekday_of_month_in_interval(day, date(2024, 2, 1), Weekday::Fri, 1).is_none()
        );
        let mut items = each_nth_weekday_of_month_in_interval(
            NaiveDate::MAX,
            NaiveDate::MAX,
            NaiveDate::MAX.weekday(),
            -1,
        )
        .unwrap();
        assert_eq!(items.next(), Some(NaiveDate::MAX));
        assert_eq!(items.next(), None);
        assert_eq!(items.next(), None);
    }
}
