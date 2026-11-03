//! Day ordinals and complete lengths of one-based Gregorian centuries.

use chrono::NaiveDate;

/// Return the one-based day position within the date's century.
///
/// Uses the existing century definition, including astronomical year zero.
/// Returns None if the century start is outside the supported date range.
pub fn get_day_of_century(date: NaiveDate) -> Option<u32> {
    let start = crate::start_of_century(date)?;
    u32::try_from(date.signed_duration_since(start).num_days() + 1).ok()
}

/// Return the number of calendar days in the date's complete century.
///
/// Returns None if either century boundary cannot be represented.
pub fn get_days_in_century(date: NaiveDate) -> Option<u32> {
    let start = crate::start_of_century(date)?;
    let end = crate::end_of_century(date)?;
    u32::try_from(end.signed_duration_since(start).num_days() + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn distinguishes_centuries_with_and_without_a_four_hundredth_leap_year() {
        assert_eq!(get_day_of_century(day(2001, 1, 1)), Some(1));
        assert_eq!(get_days_in_century(day(2024, 2, 29)), Some(36524));
        assert_eq!(get_days_in_century(day(2000, 12, 31)), Some(36525));
        assert_eq!(get_day_of_century(day(2000, 12, 31)), Some(36525));
        assert_eq!(get_day_of_century(day(2100, 12, 31)), Some(36524));
    }

    #[test]
    fn handles_bce_and_date_limits_without_overflow() {
        assert_eq!(
            get_day_of_century(day(0, 12, 31)),
            get_days_in_century(day(0, 1, 1))
        );
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            if let (Some(day), Some(days)) = (get_day_of_century(edge), get_days_in_century(edge)) {
                assert!((1..=days).contains(&day));
            }
        }
    }
}
