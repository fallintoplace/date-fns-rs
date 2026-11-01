//! Business day ordinals and totals for calendar months.

use chrono::NaiveDate;

use crate::boundaries::{end_of_month, start_of_month};
use crate::business_days::is_business_day;
use crate::business_difference::difference_in_business_days;

/// Count Monday-Friday dates in the date's month, excluding holidays only if
/// they fall on weekends (no regional holiday calendar is applied).
pub fn get_business_days_in_month(date: NaiveDate) -> u32 {
    let first = start_of_month(date);
    let last = end_of_month(date);
    let first_day = if is_business_day(first) { 1 } else { 0 };
    (difference_in_business_days(last, first) + first_day) as u32
}

/// Return the 1-based business day number, or None on a weekend.
pub fn get_business_day_of_month(date: NaiveDate) -> Option<u32> {
    if !is_business_day(date) {
        return None;
    }
    let first = start_of_month(date);
    let first_day = if is_business_day(first) { 1 } else { 0 };
    Some((difference_in_business_days(date, first) + first_day) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_march_weekdays_and_rejects_weekends() {
        assert_eq!(get_business_days_in_month(date(2024, 3, 1)), 21);
        assert_eq!(get_business_day_of_month(date(2024, 3, 1)), Some(1));
        assert_eq!(get_business_day_of_month(date(2024, 3, 4)), Some(2));
        assert_eq!(get_business_day_of_month(date(2024, 3, 29)), Some(21));
        assert_eq!(get_business_day_of_month(date(2024, 3, 2)), None);
        assert_eq!(get_business_day_of_month(date(2024, 3, 31)), None);
    }

    #[test]
    fn handles_leap_month_and_chrono_limits() {
        assert_eq!(get_business_days_in_month(date(2024, 2, 29)), 21);
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let total = get_business_days_in_month(edge);
            assert!((20..=23).contains(&total));
            if let Some(ordinal) = get_business_day_of_month(edge) {
                assert!((1..=total).contains(&ordinal));
            }
        }
    }
}
