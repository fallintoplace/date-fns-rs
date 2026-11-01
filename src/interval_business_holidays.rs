//! Lazy business-day iteration with caller-defined holiday exclusions.

use std::collections::BTreeSet;
use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::business_days::{is_business_day, next_business_day};

/// Lazy iterator over non-holiday Mondays through Fridays.
#[derive(Clone, Debug)]
pub struct HolidayBusinessDaysInInterval {
    next: Option<NaiveDate>,
    end: NaiveDate,
    holidays: BTreeSet<NaiveDate>,
}

impl Iterator for HolidayBusinessDaysInInterval {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        let mut next = next_business_day(current);
        while let Some(candidate) = next {
            if candidate > self.end {
                next = None;
                break;
            }
            if !self.holidays.contains(&candidate) {
                break;
            }
            next = next_business_day(candidate);
        }
        self.next = next;
        Some(current)
    }
}

impl FusedIterator for HolidayBusinessDaysInInterval {}

/// Iterate inclusive business days, excluding duplicate and weekend holidays.
/// A reversed interval returns None, and a valid range with no matching days
/// yields an empty iterator. Holiday dates are copied once into a sorted set.
pub fn each_business_day_of_interval_excluding_holidays(
    start: NaiveDate,
    end: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<HolidayBusinessDaysInInterval> {
    if start > end {
        return None;
    }
    let holidays: BTreeSet<_> = holidays.iter().copied().collect();
    let mut next = if is_business_day(start) {
        Some(start)
    } else {
        next_business_day(start)
    };
    while let Some(candidate) = next {
        if candidate > end {
            next = None;
            break;
        }
        if !holidays.contains(&candidate) {
            break;
        }
        next = next_business_day(candidate);
    }
    Some(HolidayBusinessDaysInInterval {
        next,
        end,
        holidays,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn excludes_weekends_duplicate_holidays_and_consecutive_dates() {
        let holidays = [date(4), date(6), date(2), date(4)];
        let days: Vec<_> =
            each_business_day_of_interval_excluding_holidays(date(1), date(8), &holidays)
                .unwrap()
                .collect();
        assert_eq!(days, vec![date(1), date(5), date(7), date(8)]);
    }

    #[test]
    fn handles_blocked_starts_empty_ranges_and_chrono_limits() {
        let days: Vec<_> =
            each_business_day_of_interval_excluding_holidays(date(2), date(4), &[date(4)])
                .unwrap()
                .collect();
        assert!(days.is_empty());
        let days: Vec<_> =
            each_business_day_of_interval_excluding_holidays(date(1), date(1), &[date(1)])
                .unwrap()
                .collect();
        assert!(days.is_empty());
        assert!(each_business_day_of_interval_excluding_holidays(date(2), date(1), &[]).is_none());
        let mut max = each_business_day_of_interval_excluding_holidays(
            NaiveDate::MAX,
            NaiveDate::MAX,
            &[NaiveDate::MAX],
        )
        .unwrap();
        assert_eq!(max.next(), None);
        assert_eq!(max.next(), None);
    }
}
