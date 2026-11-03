//! Select configured working weekdays within a calendar half-year.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::Workweek;

/// Select the nth configured workday of a calendar half-year.
///
/// Positive positions count from the start, negative ones from the end.
/// Holidays may be unsorted or duplicated. Invalid periods and positions
/// return None; the search never moves beyond the half-year.
pub fn nth_workday_of_half_year(
    week: Workweek,
    year: i32,
    half_year: u32,
    occurrence: i16,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if !(1..=2).contains(&half_year) || occurrence == 0 {
        return None;
    }
    let first = NaiveDate::from_ymd_opt(year, if half_year == 1 { 1 } else { 7 }, 1)?;
    let last = crate::end_of_half_year(first);
    let blocked: BTreeSet<_> = holidays.iter().copied().collect();
    let forward = occurrence > 0;
    let mut current = if forward { first } else { last };
    let mut remaining = u32::from(occurrence.unsigned_abs());

    loop {
        if week.is_workday(current) && !blocked.contains(&current) {
            remaining -= 1;
            if remaining == 0 {
                return Some(current);
            }
        }
        if current == if forward { last } else { first } {
            return None;
        }
        current = if forward {
            current.succ_opt()?
        } else {
            current.pred_opt()?
        };
    }
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn custom_week() -> Workweek {
        Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap()
    }

    #[test]
    fn selects_workdays_across_a_leap_half_year() {
        let week = custom_week();
        let start = day(2024, 1, 1);
        let end = day(2024, 6, 30);
        assert_eq!(nth_workday_of_half_year(week, 2024, 1, 1, &[]), Some(start));
        assert_eq!(nth_workday_of_half_year(week, 2024, 1, -1, &[]), Some(end));
        assert_eq!(
            nth_workday_of_half_year(week, 2024, 1, 1, &[start]),
            Some(day(2024, 1, 2))
        );
        assert_eq!(
            nth_workday_of_half_year(week, 2024, 1, -1, &[end, end]),
            Some(day(2024, 6, 27))
        );
        assert_eq!(
            nth_workday_of_half_year(week, 2024, 2, -1, &[]),
            Some(day(2024, 12, 31))
        );
    }

    #[test]
    fn rejects_invalid_periods_and_occurrences() {
        let week = custom_week();

        assert_eq!(nth_workday_of_half_year(week, 2024, 0, 1, &[]), None);
        assert_eq!(nth_workday_of_half_year(week, 2024, 3, 1, &[]), None);
        assert_eq!(nth_workday_of_half_year(week, 2024, 1, 0, &[]), None);
        assert_eq!(nth_workday_of_half_year(week, 2024, 1, 200, &[]), None);
        assert_eq!(nth_workday_of_half_year(week, 2024, 1, i16::MIN, &[]), None);
        assert_eq!(nth_workday_of_half_year(week, i32::MAX, 1, 1, &[]), None);
    }
}
