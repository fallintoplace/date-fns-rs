//! Count calendar periods touched by inclusive date intervals.

use chrono::{NaiveDate, Weekday};

/// Count distinct week-numbering years intersecting an inclusive date interval.
///
/// Includes both endpoints and rejects reversed intervals.
/// Use Monday and 4 for ISO week-years, Sunday and 1 for US week-years.
pub fn count_week_years_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    week_starts_on: Weekday,
    first_week_contains_date: u32,
) -> Option<i64> {
    if start > end {
        return None;
    }
    let start_year = crate::get_week_year(start, week_starts_on, first_week_contains_date)?;
    let end_year = crate::get_week_year(end, week_starts_on, first_week_contains_date)?;
    Some(i64::from(end_year) - i64::from(start_year) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_boundaries_using_the_configured_calendar() {
        assert_eq!(
            count_week_years_in_interval(day(2020, 12, 31), day(2021, 1, 1), Weekday::Mon, 4),
            Some(1)
        );
        assert_eq!(
            count_week_years_in_interval(day(2021, 1, 3), day(2021, 1, 4), Weekday::Mon, 4),
            Some(2)
        );
        assert_eq!(
            count_week_years_in_interval(day(2020, 12, 26), day(2020, 12, 27), Weekday::Sun, 1),
            Some(2)
        );
    }

    #[test]
    fn validates_rules_and_supported_date_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(
            count_week_years_in_interval(last, first, Weekday::Mon, 4),
            None
        );
        assert_eq!(
            count_week_years_in_interval(first, last, Weekday::Mon, 0),
            None
        );
        assert_eq!(
            count_week_years_in_interval(first, last, Weekday::Mon, 8),
            None
        );
        let count =
            count_week_years_in_interval(NaiveDate::MIN, NaiveDate::MAX, Weekday::Mon, 4).unwrap();
        assert!(count > 1);
        assert_eq!(
            count_week_years_in_interval(NaiveDate::MAX, NaiveDate::MAX, Weekday::Mon, 4),
            Some(1)
        );
    }
}
