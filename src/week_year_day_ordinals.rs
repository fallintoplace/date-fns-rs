//! Day positions and lengths within configurable week-numbering years.

use chrono::{NaiveDate, Weekday};

/// Return the one-based day within the week-numbering year of date.
///
/// For ISO 8601, pass Monday and 4. Returns None for invalid first-week
/// rules or if the week-year boundary is outside Chrono's supported range.
pub fn get_day_of_week_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<u32> {
    let start = crate::start_of_week_year(date, week_starts_on, first_week_contains_date)?;
    u32::try_from(date.signed_duration_since(start).num_days() + 1).ok()
}

/// Return 364 or 371 days in the complete configurable week-numbering year.
pub fn get_days_in_week_year(
    date: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<u32> {
    crate::get_weeks_in_week_year(date, week_starts_on, first_week_contains_date)?.checked_mul(7)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn includes_cross_year_days_in_iso_week_year_2020() {
        assert_eq!(
            get_day_of_week_year(day(2019, 12, 30), Weekday::Mon, 4),
            Some(1)
        );
        assert_eq!(
            get_day_of_week_year(day(2021, 1, 1), Weekday::Mon, 4),
            Some(369)
        );
        assert_eq!(
            get_day_of_week_year(day(2021, 1, 3), Weekday::Mon, 4),
            Some(371)
        );
        assert_eq!(
            get_days_in_week_year(day(2021, 1, 1), Weekday::Mon, 4),
            Some(371)
        );
        assert_eq!(
            get_days_in_week_year(day(2021, 6, 1), Weekday::Mon, 4),
            Some(364)
        );
        assert_eq!(
            get_day_of_week_year(day(2018, 12, 31), Weekday::Mon, 4),
            Some(1)
        );
    }

    #[test]
    fn accepts_sunday_first_and_rejects_invalid_conventions() {
        assert_eq!(
            get_day_of_week_year(day(2021, 1, 1), Weekday::Sun, 1),
            Some(6)
        );
        assert_eq!(get_day_of_week_year(day(2024, 1, 1), Weekday::Mon, 0), None);
        assert_eq!(
            get_days_in_week_year(day(2024, 1, 1), Weekday::Sun, 8),
            None
        );
        let _ = get_day_of_week_year(NaiveDate::MIN, Weekday::Sun, 1);
        let _ = get_days_in_week_year(NaiveDate::MAX, Weekday::Mon, 4);
    }
}
