//! Select a numbered weekday within a Gregorian half-year.

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};

/// Find the nth occurrence of a weekday in January-June or July-December.
///
/// Positive positions count from the start, negative positions from the end.
/// Returns None for half-years outside 1..=2, zero, and unavailable positions.
pub fn nth_weekday_of_half_year(
    year: i32,
    half_year: u32,
    weekday: Weekday,
    occurrence: i8,
) -> Option<NaiveDate> {
    if !(1..=2).contains(&half_year) || occurrence == 0 {
        return None;
    }
    let first_month = if half_year == 1 { 1 } else { 7 };
    let first = NaiveDate::from_ymd_opt(year, first_month, 1)?;
    let last = crate::end_of_half_year(first);
    let target = i64::from(weekday.num_days_from_monday());
    let backwards = occurrence < 0;
    let edge = if backwards { last } else { first };
    let edge_weekday = i64::from(edge.weekday().num_days_from_monday());
    let offset = if backwards {
        (edge_weekday - target).rem_euclid(7)
    } else {
        (target - edge_weekday).rem_euclid(7)
    };
    let steps = i64::from(occurrence).abs() - 1;
    let days = offset.checked_add(steps.checked_mul(7)?)?;
    let found = if backwards {
        edge.checked_sub_signed(TimeDelta::days(days))?
    } else {
        edge.checked_add_signed(TimeDelta::days(days))?
    };
    (first <= found && found <= last).then_some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_from_each_half_year_boundary() {
        assert_eq!(nth_weekday_of_half_year(2024, 1, Weekday::Mon, 1), Some(date(2024, 1, 1)));
        assert_eq!(nth_weekday_of_half_year(2024, 1, Weekday::Mon, -1), Some(date(2024, 6, 24)));
        assert_eq!(nth_weekday_of_half_year(2024, 1, Weekday::Fri, -1), Some(date(2024, 6, 28)));
        assert_eq!(nth_weekday_of_half_year(2024, 2, Weekday::Mon, 1), Some(date(2024, 7, 1)));
        assert_eq!(nth_weekday_of_half_year(2024, 2, Weekday::Fri, -1), Some(date(2024, 12, 27)));
    }

    #[test]
    fn rejects_zero_invalid_halves_and_out_of_range_occurrences() {
        assert_eq!(nth_weekday_of_half_year(2024, 1, Weekday::Mon, 0), None);
        assert_eq!(nth_weekday_of_half_year(2024, 0, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_half_year(2024, 3, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_half_year(2024, 1, Weekday::Mon, 28), None);
        assert_eq!(nth_weekday_of_half_year(2024, 2, Weekday::Mon, -28), None);
        assert_eq!(nth_weekday_of_half_year(i32::MAX, 1, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_half_year(2024, 1, Weekday::Mon, i8::MIN), None);
    }
}
