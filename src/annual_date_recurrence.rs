//! Annual month/day recurrence with explicit leap-day handling.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate};

/// Behavior for a February 29 rule in years without a leap day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeapDayPolicy {
    Skip,
    February28,
    March1,
}

/// Lazy annual occurrences, anchored at the year containing the start date.
#[derive(Clone, Debug)]
pub struct AnnualDateRecurrences {
    next_year: Option<i64>,
    start: NaiveDate,
    end: NaiveDate,
    month: u32,
    day: u32,
    year_step: i64,
    policy: LeapDayPolicy,
}

impl Iterator for AnnualDateRecurrences {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(year) = self.next_year {
            self.next_year = year
                .checked_add(self.year_step)
                .filter(|&next| next <= i64::from(self.end.year()));
            let year = i32::try_from(year).ok()?;
            let candidate = match NaiveDate::from_ymd_opt(year, self.month, self.day) {
                Some(date) => Some(date),
                None if self.month == 2 && self.day == 29 => match self.policy {
                    LeapDayPolicy::Skip => None,
                    LeapDayPolicy::February28 => NaiveDate::from_ymd_opt(year, 2, 28),
                    LeapDayPolicy::March1 => NaiveDate::from_ymd_opt(year, 3, 1),
                },
                None => None,
            };
            if let Some(date) = candidate {
                if date >= self.start && date <= self.end {
                    return Some(date);
                }
            }
        }
        None
    }
}

impl FusedIterator for AnnualDateRecurrences {}

/// Iterate a month/day rule in every Nth year of an inclusive date interval.
///
/// Year zero and BCE dates use Chrono's proleptic Gregorian calendar. The
/// first scheduled year is the year containing `start`. A day must exist in
/// at least one leap year. Invalid rules and reversed ranges return None.
pub fn each_annual_date_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    month: u32,
    day: u32,
    year_step: u32,
    policy: LeapDayPolicy,
) -> Option<AnnualDateRecurrences> {
    if start > end || year_step == 0 {
        return None;
    }
    NaiveDate::from_ymd_opt(2000, month, day)?;
    Some(AnnualDateRecurrences {
        next_year: Some(i64::from(start.year())),
        start,
        end,
        month,
        day,
        year_step: i64::from(year_step),
        policy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn chooses_each_leap_day_policy_predictably() {
        let start = date(2020, 1, 1);
        let end = date(2025, 12, 31);
        let collect = |policy| {
            each_annual_date_in_interval(start, end, 2, 29, 1, policy)
                .unwrap()
                .collect::<Vec<_>>()
        };
        assert_eq!(
            collect(LeapDayPolicy::Skip),
            vec![date(2020, 2, 29), date(2024, 2, 29)]
        );
        assert_eq!(
            collect(LeapDayPolicy::February28),
            vec![
                date(2020, 2, 29),
                date(2021, 2, 28),
                date(2022, 2, 28),
                date(2023, 2, 28),
                date(2024, 2, 29),
                date(2025, 2, 28)
            ]
        );
        assert_eq!(
            collect(LeapDayPolicy::March1),
            vec![
                date(2020, 2, 29),
                date(2021, 3, 1),
                date(2022, 3, 1),
                date(2023, 3, 1),
                date(2024, 2, 29),
                date(2025, 3, 1)
            ]
        );
    }

    #[test]
    fn supports_year_steps_and_year_zero() {
        let actual: Vec<_> = each_annual_date_in_interval(
            date(2020, 1, 1),
            date(2025, 12, 31),
            2,
            29,
            2,
            LeapDayPolicy::February28,
        )
        .unwrap()
        .collect();
        assert_eq!(
            actual,
            vec![date(2020, 2, 29), date(2022, 2, 28), date(2024, 2, 29)]
        );
        assert_eq!(
            each_annual_date_in_interval(
                date(-1, 1, 1),
                date(1, 12, 31),
                2,
                29,
                1,
                LeapDayPolicy::Skip
            )
            .unwrap()
            .collect::<Vec<_>>(),
            vec![date(0, 2, 29)]
        );
    }

    #[test]
    fn validates_rules_and_handles_maximum_date() {
        let start = date(2024, 1, 1);
        assert!(
            each_annual_date_in_interval(start, start, 4, 31, 1, LeapDayPolicy::Skip).is_none()
        );
        assert!(
            each_annual_date_in_interval(start, start, 2, 29, 0, LeapDayPolicy::Skip).is_none()
        );
        assert!(each_annual_date_in_interval(
            start,
            date(2023, 1, 1),
            1,
            1,
            1,
            LeapDayPolicy::Skip
        )
        .is_none());
        let mut last = each_annual_date_in_interval(
            NaiveDate::MAX,
            NaiveDate::MAX,
            NaiveDate::MAX.month(),
            NaiveDate::MAX.day(),
            u32::MAX,
            LeapDayPolicy::Skip,
        )
        .unwrap();
        assert_eq!(last.next(), Some(NaiveDate::MAX));
        assert_eq!(last.next(), None);
        assert_eq!(last.next(), None);
    }
}
