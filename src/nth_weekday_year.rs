//! Find a numbered occurrence of a weekday inside a calendar year.

use chrono::{Datelike, Days, NaiveDate, Weekday};

/// Find the nth weekday of the year, counting backward for negative values.
///
/// For example, -1 gives the last occurrence in December. Zero and
/// occurrences outside the calendar year return None.
pub fn nth_weekday_of_year(
    year: i32,
    weekday: Weekday,
    occurrence: i16,
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }
    let first = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let last = NaiveDate::from_ymd_opt(year, 12, 31)?;
    let target = i64::from(weekday.num_days_from_monday());
    let nth = i64::from(occurrence).abs();
    if occurrence > 0 {
        let initial = i64::from(first.weekday().num_days_from_monday());
        let offset = (target - initial).rem_euclid(7) + (nth - 1) * 7;
        first
            .checked_add_days(Days::new(u64::try_from(offset).ok()?))
            .filter(|day| *day <= last)
    } else {
        let final_day = i64::from(last.weekday().num_days_from_monday());
        let offset = (final_day - target).rem_euclid(7) + (nth - 1) * 7;
        last.checked_sub_days(Days::new(u64::try_from(offset).ok()?))
            .filter(|day| *day >= first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_weekdays_in_leap_and_common_years() {
        assert_eq!(
            nth_weekday_of_year(2024, Weekday::Mon, 1),
            Some(date(2024, 1, 1))
        );
        assert_eq!(
            nth_weekday_of_year(2024, Weekday::Mon, -1),
            Some(date(2024, 12, 30))
        );
        assert_eq!(
            nth_weekday_of_year(2024, Weekday::Thu, 9),
            Some(date(2024, 2, 29))
        );
        assert_eq!(nth_weekday_of_year(2024, Weekday::Mon, 54), None);
    }

    #[test]
    fn validates_out_of_range_occurrences_and_years() {
        assert_eq!(nth_weekday_of_year(2024, Weekday::Sun, 0), None);
        assert_eq!(nth_weekday_of_year(2024, Weekday::Sun, -54), None);
        assert_eq!(nth_weekday_of_year(i32::MAX, Weekday::Sun, 1), None);
        assert_eq!(
            nth_weekday_of_year(2024, Weekday::Mon, i16::MIN),
            None
        );
    }
}
