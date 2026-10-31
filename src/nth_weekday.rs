//! Locate a numbered weekday occurrence within a calendar month.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::boundaries::end_of_month;

/// Find the nth occurrence of a weekday in a month.
///
/// Positive occurrences count from the start, and negative occurrences count
/// from the end (-1 means the last occurrence). Zero and missing occurrences
/// return None, as do invalid months or years.
pub fn nth_weekday_of_month(
    year: i32,
    month: u32,
    weekday: Weekday,
    occurrence: i8,
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }

    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let last = end_of_month(first);
    let target = weekday.num_days_from_monday() as i32;
    let n = i32::from(occurrence).abs();

    let day = if occurrence > 0 {
        let first_weekday = first.weekday().num_days_from_monday() as i32;
        let offset = (target - first_weekday).rem_euclid(7);
        1 + offset + (n - 1) * 7
    } else {
        let last_weekday = last.weekday().num_days_from_monday() as i32;
        let offset = (last_weekday - target).rem_euclid(7);
        last.day() as i32 - offset - (n - 1) * 7
    };

    if day < 1 || day > last.day() as i32 {
        return None;
    }
    NaiveDate::from_ymd_opt(year, month, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_third_thursday_and_last_friday() {
        assert_eq!(
            nth_weekday_of_month(2024, 11, Weekday::Thu, 3),
            Some(date(2024, 11, 21))
        );
        assert_eq!(
            nth_weekday_of_month(2024, 3, Weekday::Fri, -1),
            Some(date(2024, 3, 29))
        );
        assert_eq!(
            nth_weekday_of_month(2024, 3, Weekday::Fri, -2),
            Some(date(2024, 3, 22))
        );
    }

    #[test]
    fn handles_leap_years_and_missing_occurrences() {
        assert_eq!(
            nth_weekday_of_month(2024, 2, Weekday::Thu, 5),
            Some(date(2024, 2, 29))
        );
        assert_eq!(nth_weekday_of_month(2024, 2, Weekday::Mon, 5), None);
        assert_eq!(nth_weekday_of_month(2024, 2, Weekday::Mon, -5), None);
    }

    #[test]
    fn rejects_invalid_inputs_without_overflow() {
        assert_eq!(nth_weekday_of_month(2024, 2, Weekday::Mon, 0), None);
        assert_eq!(nth_weekday_of_month(2024, 0, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_month(2024, 13, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_month(i32::MAX, 1, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_month(2024, 2, Weekday::Mon, i8::MIN), None);
    }
}
