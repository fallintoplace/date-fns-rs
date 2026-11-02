//! Configurable workweek arithmetic with explicit holiday exclusions.

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included};

use chrono::NaiveDate;

use crate::Workweek;

/// Count configured working weekdays excluding holidays in [start, end].
///
/// Duplicate holidays, non-working holidays and out-of-range holidays do
/// not change the count. Reversed dates return None.
pub fn count_workdays_with_holidays(
    week: Workweek,
    start: NaiveDate,
    end: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<i64> {
    let base = week.count_workdays(start, end)?;
    let excluded: BTreeSet<_> = holidays
        .iter()
        .copied()
        .filter(|day| *day >= start && *day <= end && week.is_workday(*day))
        .collect();
    Some(base - i64::try_from(excluded.len()).ok()?)
}

/// Move a signed number of configured workdays, skipping listed holidays.
///
/// The starting date is excluded for nonzero offsets; zero preserves the
/// input. Complete workweeks are skipped in O(1), with holiday correction
/// proportional to the number of relevant holidays.
pub fn add_workdays_with_holidays(
    week: Workweek,
    date: NaiveDate,
    offset: i64,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if offset == 0 {
        return Some(date);
    }
    let blocked: BTreeSet<_> = holidays
        .iter()
        .copied()
        .filter(|day| week.is_workday(*day))
        .collect();
    let forward = offset > 0;
    let mut current = date;
    let mut remaining = offset;
    loop {
        let candidate = week.add_workdays(current, remaining)?;
        let (lower, upper) = if forward {
            (Excluded(current), Included(candidate))
        } else {
            (Included(candidate), Excluded(current))
        };
        let skipped = i64::try_from(blocked.range((lower, upper)).count()).ok()?;
        if skipped == 0 {
            return Some(candidate);
        }
        current = candidate;
        remaining = if forward { skipped } else { -skipped };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn supports_sunday_to_thursday_workweeks_and_holidays() {
        let week = Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap();
        let holidays = [date(2024, 3, 3), date(2024, 3, 3)];
        assert_eq!(
            add_workdays_with_holidays(week, date(2024, 2, 29), 1, &holidays),
            Some(date(2024, 3, 4))
        );
        assert_eq!(
            add_workdays_with_holidays(week, date(2024, 3, 4), -1, &holidays),
            Some(date(2024, 2, 29))
        );
        assert_eq!(
            count_workdays_with_holidays(week, date(2024, 2, 29), date(2024, 3, 4), &holidays,),
            Some(2)
        );
    }

    #[test]
    fn zero_shifts_and_extreme_dates_are_checked() {
        let week = Workweek::from_mask(0x7f).unwrap();
        assert_eq!(
            add_workdays_with_holidays(week, NaiveDate::MAX, 0, &[]),
            Some(NaiveDate::MAX)
        );
        assert_eq!(
            add_workdays_with_holidays(week, NaiveDate::MAX, 1, &[]),
            None
        );
    }
}
