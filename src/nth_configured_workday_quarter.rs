//! Select configured working weekdays within a calendar quarter.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::Workweek;

/// Select the nth configured workday of a calendar quarter.
///
/// Positive positions count from the start, negative ones from the end.
/// Holidays may be unsorted or duplicated. Invalid periods and positions
/// return None; the search never moves beyond the quarter.
pub fn nth_workday_of_quarter(
    week: Workweek,
    year: i32,
    quarter: u32,
    occurrence: i16,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if !(1..=4).contains(&quarter) || occurrence == 0 {
        return None;
    }
    let first = NaiveDate::from_ymd_opt(year, (quarter - 1) * 3 + 1, 1)?;
    let last = crate::end_of_quarter(first);
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
    fn selects_configured_weekdays_across_quarter_end() {
        let week = custom_week();
        let start = day(2024, 1, 1);
        let end = day(2024, 3, 31);
        assert_eq!(nth_workday_of_quarter(week, 2024, 1, 1, &[]), Some(start));
        assert_eq!(nth_workday_of_quarter(week, 2024, 1, -1, &[]), Some(end));
        assert_eq!(
            nth_workday_of_quarter(week, 2024, 1, 1, &[start, start]),
            Some(day(2024, 1, 2))
        );
        assert_eq!(
            nth_workday_of_quarter(week, 2024, 1, -1, &[end]),
            Some(day(2024, 3, 28))
        );
    }

    #[test]
    fn rejects_invalid_periods_and_occurrences() {
        let week = custom_week();

        assert_eq!(nth_workday_of_quarter(week, 2024, 0, 1, &[]), None);
        assert_eq!(nth_workday_of_quarter(week, 2024, 5, 1, &[]), None);
        assert_eq!(nth_workday_of_quarter(week, 2024, 1, 0, &[]), None);
        assert_eq!(nth_workday_of_quarter(week, 2024, 1, 100, &[]), None);
        assert_eq!(nth_workday_of_quarter(week, 2024, 1, i16::MIN, &[]), None);
        assert_eq!(nth_workday_of_quarter(week, i32::MAX, 1, 1, &[]), None);
    }
}
