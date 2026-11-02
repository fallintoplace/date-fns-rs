//! Explicit weekend holiday observance for caller-supplied calendars.

use std::collections::BTreeSet;

use chrono::{Datelike, Days, NaiveDate, Weekday};

/// How weekend holidays are moved to additional observed nonworking dates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HolidayObservance {
    /// Do not add observed holidays.
    None,
    /// Observe Saturdays on Friday and Sundays on Monday.
    NearestWeekday,
    /// Observe both Saturdays and Sundays on Monday.
    FollowingMonday,
    /// Observe each weekend date on the next weekday not already occupied.
    ///
    /// Reservations include original holidays, so consecutive weekend
    /// holidays never collide with a weekday holiday or each other.
    FollowingFreeWeekday,
}

/// Add observed dates to a holiday calendar, without dropping original dates.
///
/// Input may be unsorted and contain duplicates. Returns sorted, deduplicated
/// dates, or None when a required observed date falls beyond Chrono's range.
/// NearestWeekday and FollowingMonday can collapse multiple observations
/// onto the same date; use FollowingFreeWeekday to avoid such collisions.
pub fn observed_holidays(
    holidays: &[NaiveDate],
    policy: HolidayObservance,
) -> Option<Vec<NaiveDate>> {
    let mut output: BTreeSet<NaiveDate> = holidays.iter().copied().collect();
    let actual: Vec<_> = output.iter().copied().collect();
    for original in actual {
        let weekday = original.weekday();
        let observed = match (policy, weekday) {
            (HolidayObservance::None, _) => None,
            (_, Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri) => {
                None
            }
            (HolidayObservance::NearestWeekday, Weekday::Sat) => {
                Some(original.checked_sub_days(Days::new(1))?)
            }
            (HolidayObservance::NearestWeekday, Weekday::Sun) => {
                Some(original.checked_add_days(Days::new(1))?)
            }
            (HolidayObservance::FollowingMonday, Weekday::Sat) => {
                Some(original.checked_add_days(Days::new(2))?)
            }
            (HolidayObservance::FollowingMonday, Weekday::Sun) => {
                Some(original.checked_add_days(Days::new(1))?)
            }
            (HolidayObservance::FollowingFreeWeekday, Weekday::Sat | Weekday::Sun) => {
                let offset = if weekday == Weekday::Sat { 2 } else { 1 };
                let mut candidate = original.checked_add_days(Days::new(offset))?;
                while matches!(candidate.weekday(), Weekday::Sat | Weekday::Sun)
                    || output.contains(&candidate)
                {
                    candidate = candidate.checked_add_days(Days::new(1))?;
                }
                Some(candidate)
            }
        };
        if let Some(date) = observed {
            output.insert(date);
        }
    }
    Some(output.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn nearest_weekday_and_monday_policies_are_explicit() {
        let christmas = date(2021, 12, 25);
        let boxing_day = date(2021, 12, 26);
        assert_eq!(
            observed_holidays(
                &[boxing_day, christmas, christmas],
                HolidayObservance::NearestWeekday
            ),
            Some(vec![
                date(2021, 12, 24),
                christmas,
                boxing_day,
                date(2021, 12, 27),
            ])
        );
        assert_eq!(
            observed_holidays(
                &[christmas, boxing_day],
                HolidayObservance::FollowingMonday
            ),
            Some(vec![christmas, boxing_day, date(2021, 12, 27)])
        );
        assert_eq!(
            observed_holidays(&[christmas], HolidayObservance::None),
            Some(vec![christmas])
        );
    }

    #[test]
    fn next_free_policy_avoids_observed_date_collisions() {
        let original = [
            date(2021, 12, 25),
            date(2021, 12, 26),
            date(2021, 12, 27),
        ];
        assert_eq!(
            observed_holidays(&original, HolidayObservance::FollowingFreeWeekday),
            Some(vec![
                date(2021, 12, 25),
                date(2021, 12, 26),
                date(2021, 12, 27),
                date(2021, 12, 28),
                date(2021, 12, 29),
            ])
        );
    }

    #[test]
    fn remains_sorted_across_years_and_handles_empty_input() {
        assert_eq!(
            observed_holidays(
                &[date(2023, 1, 1), date(2022, 12, 31)],
                HolidayObservance::NearestWeekday
            ),
            Some(vec![
                date(2022, 12, 30),
                date(2022, 12, 31),
                date(2023, 1, 1),
                date(2023, 1, 2),
            ])
        );
        assert_eq!(
            observed_holidays(&[], HolidayObservance::FollowingFreeWeekday),
            Some(vec![])
        );
    }
}
