//! Business-day arithmetic with caller-defined holiday exclusions.

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included};

use chrono::NaiveDate;

use crate::business_days::{add_business_days, is_business_day};

/// Add a signed number of business days, excluding supplied holidays.
///
/// The starting day is not counted. A zero offset preserves the start even
/// if it is a weekend or holiday. Duplicates and weekend holidays are ignored.
/// Dates are advanced in blocks rather than scanning every day of long spans.
/// Returns None on date overflow.
pub fn add_business_days_with_holidays(
    date: NaiveDate,
    days: i64,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if days == 0 {
        return Some(date);
    }

    let blocked: BTreeSet<_> = holidays
        .iter()
        .copied()
        .filter(|day| is_business_day(*day))
        .collect();
    let forward = days > 0;
    let mut current = date;
    let mut remaining = days;

    loop {
        let candidate = add_business_days(current, remaining)?;
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

/// Subtract a signed number of business days with holiday exclusions.
pub fn sub_business_days_with_holidays(
    date: NaiveDate,
    days: i64,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    add_business_days_with_holidays(date, days.checked_neg()?, holidays)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn advances_past_multiple_holidays_in_both_directions() {
        let friday = date(2024, 3, 1);
        let tuesday = date(2024, 3, 12);
        let holidays = [date(2024, 3, 4), date(2024, 3, 7)];
        assert_eq!(
            add_business_days_with_holidays(friday, 5, &holidays),
            Some(tuesday)
        );
        assert_eq!(
            add_business_days_with_holidays(tuesday, -5, &holidays),
            Some(friday)
        );
        assert_eq!(
            sub_business_days_with_holidays(tuesday, 5, &holidays),
            Some(friday)
        );
    }

    #[test]
    fn ignores_duplicate_and_weekend_holidays() {
        let friday = date(2024, 3, 1);
        let holidays = [
            date(2024, 3, 4),
            date(2024, 3, 4),
            date(2024, 3, 2),
            date(2024, 4, 1),
        ];
        assert_eq!(
            add_business_days_with_holidays(friday, 1, &holidays),
            Some(date(2024, 3, 5))
        );
        assert_eq!(
            add_business_days_with_holidays(date(2024, 3, 2), 1, &holidays),
            Some(date(2024, 3, 5))
        );
        assert_eq!(
            add_business_days_with_holidays(friday, 0, &holidays),
            Some(friday)
        );
    }

    #[test]
    fn handles_no_holidays_and_chrono_limits() {
        let friday = date(2024, 3, 1);
        assert_eq!(
            add_business_days_with_holidays(friday, 5, &[]),
            Some(date(2024, 3, 8))
        );
        assert_eq!(
            add_business_days_with_holidays(NaiveDate::MAX, 1, &[]),
            None
        );
        assert_eq!(
            sub_business_days_with_holidays(friday, i64::MIN, &[]),
            None
        );
    }
}
