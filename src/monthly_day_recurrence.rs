//! Month-based recurrence with explicit handling of short months.

use std::iter::FusedIterator;

use chrono::{Datelike, Months, NaiveDate};

/// How to handle months without the requested day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonthDayPolicy {
    /// Use the last date in the month when its length is too short.
    Clamp,
    /// Omit months without the requested date.
    Skip,
}

/// Lazy monthly dates generated from a day-of-month rule.
#[derive(Clone, Debug)]
pub struct MonthlyDayRecurrences {
    next_month: Option<NaiveDate>,
    start: NaiveDate,
    end: NaiveDate,
    day: u32,
    step_months: u32,
    policy: MonthDayPolicy,
}

impl Iterator for MonthlyDayRecurrences {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(first) = self.next_month {
            self.next_month = first
                .checked_add_months(Months::new(self.step_months))
                .filter(|&next| next <= self.end);
            let last_day = crate::end_of_month(first).day();
            if self.day > last_day && self.policy == MonthDayPolicy::Skip {
                continue;
            }
            let day = self.day.min(last_day);
            let date = NaiveDate::from_ymd_opt(first.year(), first.month(), day)?;
            if date >= self.start && date <= self.end {
                return Some(date);
            }
        }
        None
    }
}

impl FusedIterator for MonthlyDayRecurrences {}

/// Iterate matching day-of-month dates in every Nth month, inclusive.
///
/// The month containing `start` is period zero. Invalid days, zero steps,
/// and reversed ranges return None. The iterator is allocation-free.
pub fn each_day_of_month_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    day: u32,
    step_months: u32,
    policy: MonthDayPolicy,
) -> Option<MonthlyDayRecurrences> {
    if start > end || !(1..=31).contains(&day) || step_months == 0 {
        return None;
    }
    Some(MonthlyDayRecurrences {
        next_month: Some(crate::start_of_month(start)),
        start,
        end,
        day,
        step_months,
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
    fn clamps_month_ends_across_leap_february() {
        let actual: Vec<_> = each_day_of_month_in_interval(
            date(2024, 1, 30),
            date(2024, 5, 31),
            31,
            1,
            MonthDayPolicy::Clamp,
        )
        .unwrap()
        .collect();
        assert_eq!(
            actual,
            vec![
                date(2024, 1, 31),
                date(2024, 2, 29),
                date(2024, 3, 31),
                date(2024, 4, 30),
                date(2024, 5, 31)
            ]
        );
    }

    #[test]
    fn skips_short_months_and_supports_month_steps() {
        let actual: Vec<_> = each_day_of_month_in_interval(
            date(2023, 1, 1),
            date(2023, 8, 31),
            31,
            2,
            MonthDayPolicy::Skip,
        )
        .unwrap()
        .collect();
        assert_eq!(
            actual,
            vec![
                date(2023, 1, 31),
                date(2023, 3, 31),
                date(2023, 5, 31),
                date(2023, 7, 31)
            ]
        );
        let skip_february: Vec<_> = each_day_of_month_in_interval(
            date(2024, 1, 1),
            date(2024, 3, 31),
            30,
            1,
            MonthDayPolicy::Skip,
        )
        .unwrap()
        .collect();
        assert_eq!(skip_february, vec![date(2024, 1, 30), date(2024, 3, 30)]);
    }

    #[test]
    fn rejects_bad_parameters_and_stays_fused_at_max() {
        let today = date(2024, 3, 1);
        assert!(each_day_of_month_in_interval(today, today, 0, 1, MonthDayPolicy::Clamp).is_none());
        assert!(each_day_of_month_in_interval(today, today, 32, 1, MonthDayPolicy::Skip).is_none());
        assert!(each_day_of_month_in_interval(today, today, 1, 0, MonthDayPolicy::Skip).is_none());
        assert!(
            each_day_of_month_in_interval(today, date(2024, 2, 1), 1, 1, MonthDayPolicy::Skip)
                .is_none()
        );
        let mut last = each_day_of_month_in_interval(
            NaiveDate::MAX,
            NaiveDate::MAX,
            NaiveDate::MAX.day(),
            1,
            MonthDayPolicy::Clamp,
        )
        .unwrap();
        assert_eq!(last.next(), Some(NaiveDate::MAX));
        assert_eq!(last.next(), None);
        assert_eq!(last.next(), None);
    }
}
