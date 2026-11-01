//! Business day ordinals and totals for calendar years.

use chrono::NaiveDate;

use crate::boundaries::{end_of_year, start_of_year};
use crate::business_days::is_business_day;
use crate::business_difference::difference_in_business_days;
use crate::holiday_business_difference::difference_in_business_days_excluding_holidays;

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

/// Count weekdays in the calendar year after excluding provided holidays.
///
/// Duplicate holidays and weekend holidays are ignored.
pub fn get_business_days_in_year_with_holidays(date: NaiveDate, holidays: &[NaiveDate]) -> u32 {
    let first = start_of_year(date);
    let last = end_of_year(date);
    let initial = if is_business_day(first) && !holidays.contains(&first) {
        1
    } else {
        0
    };
    let remaining = difference_in_business_days_excluding_holidays(last, first, holidays);
    (remaining + initial) as u32
}

/// Return a 1-based holiday-adjusted working-day ordinal in the year.
///
/// Returns None if the date is a weekend or a supplied holiday.
pub fn get_business_day_of_year_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<u32> {
    if !is_business_day(date) || holidays.contains(&date) {
        return None;
    }
    let first = start_of_year(date);
    let initial = if is_business_day(first) && !holidays.contains(&first) {
        1
    } else {
        0
    };
    let remaining = difference_in_business_days_excluding_holidays(date, first, holidays);
    Some((remaining + initial) as u32)
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
    #[test]
    fn holiday_aware_year_counts_cover_leap_day_and_year_end() {
        let holidays = [
            date(2024, 1, 1),
            date(2024, 2, 29),
            date(2024, 12, 31),
            date(2024, 1, 6),
            date(2024, 2, 29),
        ];
        assert_eq!(
            get_business_days_in_year_with_holidays(date(2024, 4, 5), &holidays),
            259
        );
        assert_eq!(
            get_business_day_of_year_with_holidays(date(2024, 1, 2), &holidays),
            Some(1)
        );
        assert_eq!(
            get_business_day_of_year_with_holidays(date(2024, 12, 30), &holidays),
            Some(259)
        );
        assert_eq!(
            get_business_day_of_year_with_holidays(date(2024, 12, 31), &holidays),
            None
        );
        assert_eq!(
            get_business_days_in_year_with_holidays(date(2024, 1, 1), &[]),
            get_business_days_in_year(date(2024, 1, 1))
        );
    }

}
