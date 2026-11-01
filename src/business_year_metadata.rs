//! Business day ordinals and totals for calendar years.

use chrono::NaiveDate;

use crate::boundaries::{end_of_year, start_of_year};
use crate::business_days::is_business_day;
use crate::business_difference::difference_in_business_days;

/// Count Monday-Friday dates in the calendar year, without regional holidays.
pub fn get_business_days_in_year(date: NaiveDate) -> u32 {
    let first = start_of_year(date);
    let last = end_of_year(date);
    let first_day = if is_business_day(first) { 1 } else { 0 };
    (difference_in_business_days(last, first) + first_day) as u32
}

/// Return the 1-based business-day ordinal in the year, or None on a weekend.
pub fn get_business_day_of_year(date: NaiveDate) -> Option<u32> {
    if !is_business_day(date) {
        return None;
    }
    let first = start_of_year(date);
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
    fn counts_common_and_leap_years() {
        assert_eq!(get_business_days_in_year(date(2024, 1, 1)), 262);
        assert_eq!(get_business_days_in_year(date(2023, 1, 1)), 260);
        assert_eq!(get_business_day_of_year(date(2024, 1, 1)), Some(1));
        assert_eq!(get_business_day_of_year(date(2024, 12, 31)), Some(262));
        assert_eq!(get_business_day_of_year(date(2024, 12, 29)), None);
    }

    #[test]
    fn covers_extreme_calendar_years() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let total = get_business_days_in_year(edge);
            assert!((260..=262).contains(&total));
            if let Some(day) = get_business_day_of_year(edge) {
                assert!((1..=total).contains(&day));
            }
        }
    }
}
