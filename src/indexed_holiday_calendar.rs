//! An indexed holiday calendar for repeated business-date queries.

use chrono::{Datelike, NaiveDate};

/// Sorted, unique weekday holidays with efficient queries and jumps.
///
/// Sorting/deduplication happens once. Membership takes O(log h), inclusive
/// counts take O(log h), and advancing by an arbitrary number of business
/// days takes O(log(range) * log(h)) without scanning calendar days.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexedHolidayCalendar {
    holidays: Vec<NaiveDate>,
}

impl IndexedHolidayCalendar {
    /// Index holiday dates; ignore duplicates and Saturdays/Sundays.
    pub fn new(holidays: &[NaiveDate]) -> Self {
        let mut holidays: Vec<_> = holidays
            .iter()
            .copied()
            .filter(|date| crate::is_business_day(*date))
            .collect();
        holidays.sort_unstable();
        holidays.dedup();
        Self { holidays }
    }

    /// Return the sorted, unique weekday holidays.
    pub fn holidays(&self) -> &[NaiveDate] {
        &self.holidays
    }

    /// Check whether a date is neither a weekend nor a holiday.
    pub fn is_business_day(&self, date: NaiveDate) -> bool {
        crate::is_business_day(date) && self.holidays.binary_search(&date).is_err()
    }

    /// Count unique excluded holidays in an inclusive calendar range.
    ///
    /// Returns None for reversed endpoints.
    pub fn count_holidays(&self, start: NaiveDate, end: NaiveDate) -> Option<usize> {
        if start > end {
            return None;
        }
        let first = self.holidays.partition_point(|date| *date < start);
        let last = self.holidays.partition_point(|date| *date <= end);
        Some(last - first)
    }

    /// Count Monday-Friday workdays excluding holidays in an inclusive range.
    pub fn count_business_days(&self, start: NaiveDate, end: NaiveDate) -> Option<i64> {
        let weekdays = crate::count_business_days_in_interval(start, end)?;
        let excluded = i64::try_from(self.count_holidays(start, end)?).ok()?;
        weekdays.checked_sub(excluded)
    }

    /// Advance by signed business days, excluding the starting date.
    ///
    /// A zero offset preserves the start even on weekends or holidays.
    /// Binary search locates the first or last date reaching the requested
    /// count, with checked Chrono boundaries and no long day-by-day loop.
    pub fn add_business_days(&self, date: NaiveDate, offset: i64) -> Option<NaiveDate> {
        if offset == 0 {
            return Some(date);
        }
        let amount = offset.unsigned_abs();
        let to_date = |ordinal: i64| {
            NaiveDate::from_num_days_from_ce_opt(i32::try_from(ordinal).ok()?)
        };
        if offset > 0 {
            let first = date.succ_opt()?;
            let mut lo = i64::from(first.num_days_from_ce());
            let mut hi = i64::from(NaiveDate::MAX.num_days_from_ce());
            if u64::try_from(self.count_business_days(first, NaiveDate::MAX)?).ok()? < amount {
                return None;
            }
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if u64::try_from(self.count_business_days(first, to_date(mid)?)?).ok()? >= amount {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            to_date(lo)
        } else {
            let last = date.pred_opt()?;
            let mut lo = i64::from(NaiveDate::MIN.num_days_from_ce());
            let mut hi = i64::from(last.num_days_from_ce());
            if u64::try_from(self.count_business_days(NaiveDate::MIN, last)?).ok()? < amount {
                return None;
            }
            while lo < hi {
                let mid = lo + (hi - lo + 1) / 2;
                if u64::try_from(self.count_business_days(to_date(mid)?, last)?).ok()? >= amount {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            to_date(lo)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn de_duplicates_weekday_holidays_and_counts_half_open_equivalents() {
        let first = date(2024, 3, 1);
        let last = date(2024, 3, 12);
        let calendar = IndexedHolidayCalendar::new(&[
            date(2024, 3, 7),
            date(2024, 3, 4),
            date(2024, 3, 4),
            date(2024, 3, 2),
        ]);
        assert_eq!(
            calendar.holidays(),
            &[date(2024, 3, 4), date(2024, 3, 7)]
        );
        assert!(!calendar.is_business_day(date(2024, 3, 4)));
        assert!(!calendar.is_business_day(date(2024, 3, 2)));
        assert!(calendar.is_business_day(date(2024, 3, 5)));
        assert_eq!(calendar.count_holidays(first, last), Some(2));
        assert_eq!(calendar.count_business_days(first, last), Some(6));
        assert_eq!(calendar.add_business_days(first, 5), Some(last));
        assert_eq!(calendar.add_business_days(last, -5), Some(first));
        assert_eq!(calendar.count_business_days(last, first), None);
    }

    #[test]
    fn agrees_with_existing_arithmetic_across_weekends_and_holiday_clusters() {
        let holidays = [
            date(2024, 2, 29),
            date(2024, 3, 1),
            date(2024, 3, 1),
            date(2024, 3, 2),
            date(2024, 3, 4),
            date(2024, 3, 5),
            date(2024, 3, 14),
            date(2024, 3, 15),
        ];
        let calendar = IndexedHolidayCalendar::new(&holidays);
        let first = date(2024, 2, 25);
        for origin in crate::each_day_of_interval(first, date(2024, 3, 12)).unwrap() {
            for offset in -30..=30 {
                assert_eq!(
                    calendar.add_business_days(origin, offset),
                    crate::add_business_days_with_holidays(origin, offset, &holidays),
                    "{origin}, {offset}"
                );
            }
        }
        for start in crate::each_day_of_interval(first, date(2024, 3, 5)).unwrap() {
            for end in crate::each_day_of_interval(start, date(2024, 3, 20)).unwrap() {
                let slow = crate::each_day_of_interval(start, end)
                    .unwrap()
                    .filter(|day| calendar.is_business_day(*day))
                    .count() as i64;
                assert_eq!(calendar.count_business_days(start, end), Some(slow));
            }
        }
    }

    #[test]
    fn enforces_boundaries_and_extreme_offsets() {
        let empty = IndexedHolidayCalendar::new(&[]);
        assert_eq!(empty.add_business_days(NaiveDate::MAX, 1), None);
        assert_eq!(empty.add_business_days(NaiveDate::MIN, -1), None);
        assert_eq!(empty.add_business_days(date(2024, 3, 1), i64::MIN), None);
        assert_eq!(empty.add_business_days(date(2024, 3, 1), i64::MAX), None);
        assert_eq!(
            empty.add_business_days(date(2024, 3, 2), 0),
            Some(date(2024, 3, 2))
        );
        assert_eq!(
            empty.count_holidays(NaiveDate::MIN, NaiveDate::MAX),
            Some(0)
        );
        assert_eq!(
            empty.count_business_days(NaiveDate::MIN, NaiveDate::MAX),
            crate::count_business_days_in_interval(NaiveDate::MIN, NaiveDate::MAX)
        );
    }
}
