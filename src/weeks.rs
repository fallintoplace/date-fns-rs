//! Calendar week boundaries with explicit week-start conventions.

use chrono::{Datelike, Days, NaiveDate, Weekday};

fn days_since_week_start(date: NaiveDate, week_starts_on: Weekday) -> u64 {
    let weekday = date.weekday().num_days_from_monday();
    let first = week_starts_on.num_days_from_monday();
    u64::from((weekday + 7 - first) % 7)
}

/// Return the first day of the week containing `date`.
///
/// The `week_starts_on` argument is explicit to avoid locale-dependent results.
/// Returns `None` if the boundary is outside Chrono's supported date range.
pub fn start_of_week(date: NaiveDate, week_starts_on: Weekday) -> Option<NaiveDate> {
    date.checked_sub_days(Days::new(days_since_week_start(date, week_starts_on)))
}

/// Return the last day of the week containing `date`.
///
/// Both first and last days are inclusive.
pub fn end_of_week(date: NaiveDate, week_starts_on: Weekday) -> Option<NaiveDate> {
    let elapsed = days_since_week_start(date, week_starts_on);
    date.checked_add_days(Days::new(6 - elapsed))
}

/// Return Monday of the ISO week containing `date`.
pub fn start_of_iso_week(date: NaiveDate) -> Option<NaiveDate> {
    start_of_week(date, Weekday::Mon)
}

/// Return Sunday of the ISO week containing `date`.
pub fn end_of_iso_week(date: NaiveDate) -> Option<NaiveDate> {
    end_of_week(date, Weekday::Mon)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn supports_monday_and_sunday_first_calendars() {
        let wednesday = date(2024, 1, 3);
        assert_eq!(
            start_of_week(wednesday, Weekday::Mon),
            Some(date(2024, 1, 1))
        );
        assert_eq!(end_of_week(wednesday, Weekday::Mon), Some(date(2024, 1, 7)));
        assert_eq!(
            start_of_week(wednesday, Weekday::Sun),
            Some(date(2023, 12, 31))
        );
        assert_eq!(end_of_week(wednesday, Weekday::Sun), Some(date(2024, 1, 6)));
    }

    #[test]
    fn iso_helpers_cross_year_boundaries() {
        let new_year = date(2023, 12, 31);
        assert_eq!(start_of_iso_week(new_year), Some(date(2023, 12, 25)));
        assert_eq!(end_of_iso_week(new_year), Some(date(2023, 12, 31)));
    }

    #[test]
    fn checked_boundaries_avoid_overflow() {
        let first = NaiveDate::MIN;
        let last = NaiveDate::MAX;
        assert_eq!(start_of_week(first, first.weekday()), Some(first));
        assert_eq!(start_of_week(first, first.weekday().succ()), None);
        assert_eq!(end_of_week(last, last.weekday()), None);
    }
}
