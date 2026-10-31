//! Padded monthly calendar grids with an explicit first weekday.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::boundaries::{end_of_month, start_of_month};
use crate::weeks::{end_of_week, start_of_week};

/// Return the first and last visible dates of a monthly calendar grid.
///
/// A grid includes all complete weeks touching the calendar month.
/// Returns None if padding would exceed Chrono's supported date range.
pub fn calendar_month_grid(
    date: NaiveDate,
    week_starts_on: Weekday,
) -> Option<(NaiveDate, NaiveDate)> {
    Some((
        start_of_week(start_of_month(date), week_starts_on)?,
        end_of_week(end_of_month(date), week_starts_on)?,
    ))
}

/// Count complete week rows (four to six) in a padded monthly grid.
///
/// Returns None when either padded endpoint is not representable.
pub fn get_calendar_weeks_in_month(
    date: NaiveDate,
    week_starts_on: Weekday,
) -> Option<u32> {
    let (start, end) = calendar_month_grid(date, week_starts_on)?;
    let days = i64::from(end.num_days_from_ce()) - i64::from(start.num_days_from_ce()) + 1;
    u32::try_from(days / 7).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn pads_leap_february_to_five_monday_first_weeks() {
        let day = date(2024, 2, 29);
        assert_eq!(
            calendar_month_grid(day, Weekday::Mon),
            Some((date(2024, 1, 29), date(2024, 3, 3)))
        );
        assert_eq!(get_calendar_weeks_in_month(day, Weekday::Mon), Some(5));
    }

    #[test]
    fn supports_six_week_sunday_first_months() {
        let day = date(2024, 3, 12);
        assert_eq!(
            calendar_month_grid(day, Weekday::Sun),
            Some((date(2024, 2, 25), date(2024, 4, 6)))
        );
        assert_eq!(get_calendar_weeks_in_month(day, Weekday::Sun), Some(6));
    }

    #[test]
    fn handles_unpadded_months_and_checked_date_limits() {
        assert_eq!(
            calendar_month_grid(date(2021, 2, 10), Weekday::Mon),
            Some((date(2021, 2, 1), date(2021, 2, 28)))
        );
        assert_eq!(
            get_calendar_weeks_in_month(date(2021, 2, 10), Weekday::Mon),
            Some(4)
        );
        assert_eq!(
            calendar_month_grid(NaiveDate::MIN, NaiveDate::MIN.weekday().succ()),
            None
        );
        assert_eq!(
            calendar_month_grid(NaiveDate::MAX, NaiveDate::MAX.weekday()),
            None
        );
    }
}
