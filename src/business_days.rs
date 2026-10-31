//! Weekday-only arithmetic that skips Saturdays and Sundays.

use chrono::NaiveDate;

use crate::arithmetic::{add_days, sub_days};
use crate::compare::is_weekend;

/// Return whether the date is Monday through Friday.
pub fn is_business_day(date: NaiveDate) -> bool {
    !is_weekend(date)
}

/// Move by a signed number of business days, skipping weekends.
///
/// The start date is not counted. A zero offset preserves the start date,
/// even on a weekend. Nonzero offsets from a weekend start at the next
/// weekday in the requested direction. Work scales with at most four
/// remaining weekdays after skipping complete five-day workweeks.
/// Returns None on calendar overflow.
pub fn add_business_days(date: NaiveDate, days: i64) -> Option<NaiveDate> {
    if days == 0 {
        return Some(date);
    }

    let forward = days > 0;
    let mut remaining = days.unsigned_abs();
    let mut current = date;

    if is_weekend(current) {
        loop {
            current = if forward {
                current.succ_opt()?
            } else {
                current.pred_opt()?
            };
            if is_business_day(current) {
                break;
            }
        }
        remaining -= 1;
    }

    let calendar_days = i64::try_from((remaining / 5).checked_mul(7)?).ok()?;
    current = if forward {
        add_days(current, calendar_days)?
    } else {
        sub_days(current, calendar_days)?
    };

    for _ in 0..(remaining % 5) {
        loop {
            current = if forward {
                current.succ_opt()?
            } else {
                current.pred_opt()?
            };
            if is_business_day(current) {
                break;
            }
        }
    }

    Some(current)
}

/// Subtract a signed number of business days without silently wrapping.
pub fn sub_business_days(date: NaiveDate, days: i64) -> Option<NaiveDate> {
    add_business_days(date, days.checked_neg()?)
}

/// Find the next weekday strictly after the date.
pub fn next_business_day(date: NaiveDate) -> Option<NaiveDate> {
    add_business_days(date, 1)
}

/// Find the previous weekday strictly before the date.
pub fn previous_business_day(date: NaiveDate) -> Option<NaiveDate> {
    add_business_days(date, -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn skips_weekends_in_both_directions() {
        let friday = date(2024, 3, 1);
        let monday = date(2024, 3, 4);
        assert!(is_business_day(friday));
        assert!(!is_business_day(date(2024, 3, 2)));
        assert_eq!(next_business_day(friday), Some(monday));
        assert_eq!(previous_business_day(monday), Some(friday));
        assert_eq!(add_business_days(friday, 5), Some(date(2024, 3, 8)));
        assert_eq!(sub_business_days(monday, 5), Some(date(2024, 2, 26)));
    }

    #[test]
    fn weekend_start_does_not_count_start_day() {
        let saturday = date(2024, 3, 2);
        let sunday = date(2024, 3, 3);
        assert_eq!(add_business_days(saturday, 0), Some(saturday));
        assert_eq!(add_business_days(saturday, 1), Some(date(2024, 3, 4)));
        assert_eq!(add_business_days(saturday, 5), Some(date(2024, 3, 8)));
        assert_eq!(add_business_days(sunday, -1), Some(date(2024, 3, 1)));
        assert_eq!(add_business_days(sunday, -5), Some(date(2024, 2, 26)));
    }

    #[test]
    fn offsets_are_checked_at_date_limits() {
        assert_eq!(add_business_days(NaiveDate::MAX, 1), None);
        assert_eq!(add_business_days(NaiveDate::MIN, -1), None);
        assert_eq!(add_business_days(date(2024, 3, 1), i64::MIN), None);
        assert_eq!(sub_business_days(date(2024, 3, 1), i64::MIN), None);
    }
}
