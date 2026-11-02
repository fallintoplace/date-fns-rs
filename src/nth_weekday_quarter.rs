//! Select a numbered weekday within a Gregorian calendar quarter.

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};

/// Find the nth occurrence of a weekday in a Gregorian quarter.
///
/// Positive indices count from the start; negative indices count from the end.
/// Zero, invalid quarters, and positions outside the quarter return None.
pub fn nth_weekday_of_quarter(
    year: i32,
    quarter: u32,
    weekday: Weekday,
    occurrence: i8,
) -> Option<NaiveDate> {
    if !(1..=4).contains(&quarter) || occurrence == 0 {
        return None;
    }
    let first = NaiveDate::from_ymd_opt(year, (quarter - 1) * 3 + 1, 1)?;
    let last = crate::end_of_quarter(first);
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
    fn finds_weekdays_from_both_quarter_ends() {
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Thu, 1), Some(date(2024, 1, 4)));
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Thu, 9), Some(date(2024, 2, 29)));
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Fri, -1), Some(date(2024, 3, 29)));
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Fri, -2), Some(date(2024, 3, 22)));
        assert_eq!(nth_weekday_of_quarter(2023, 4, Weekday::Mon, 1), Some(date(2023, 10, 2)));
    }

    #[test]
    fn rejects_invalid_positions_and_calendar_values() {
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Fri, 14), None);
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Fri, -14), None);
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Fri, 0), None);
        assert_eq!(nth_weekday_of_quarter(2024, 0, Weekday::Fri, 1), None);
        assert_eq!(nth_weekday_of_quarter(2024, 5, Weekday::Fri, 1), None);
        assert_eq!(nth_weekday_of_quarter(i32::MAX, 1, Weekday::Fri, 1), None);
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Fri, i8::MIN), None);
    }
}
