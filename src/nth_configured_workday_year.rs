//! Select configured working weekdays within a calendar year.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::Workweek;

/// Select the nth configured workday of a calendar year.
///
/// Positive positions count from the start, negative ones from the end.
/// Holidays may be unsorted or duplicated. Invalid periods and positions
/// return None; the search never moves beyond the year.
pub fn nth_workday_of_year(
    week: Workweek,
    year: i32,
    occurrence: i16,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }
    let first = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let last = crate::end_of_year(first);
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
        let boundary = if forward { last } else { first };
        if current == boundary {
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
    fn selects_year_edge_workdays_with_duplicate_holidays() {
        let week = custom_week();
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(nth_workday_of_year(week, 2024, 1, &[]), Some(first));
        assert_eq!(nth_workday_of_year(week, 2024, -1, &[]), Some(last));
        assert_eq!(
            nth_workday_of_year(week, 2024, 1, &[first, first]),
            Some(day(2024, 1, 2))
        );
        assert_eq!(
            nth_workday_of_year(week, 2024, -1, &[last, last]),
            Some(day(2024, 12, 30))
        );
        let every_day = Workweek::from_mask(0x7f).unwrap();
        assert_eq!(
            nth_workday_of_year(every_day, 2024, 366, &[]),
            Some(last)
        );
        assert_eq!(nth_workday_of_year(every_day, 2024, 367, &[]), None);
    }

    #[test]
    fn rejects_invalid_periods_and_occurrences() {
        let week = custom_week();

        assert_eq!(nth_workday_of_year(week, 2024, 0, &[]), None);
        assert_eq!(nth_workday_of_year(week, 2024, i16::MIN, &[]), None);
        assert_eq!(nth_workday_of_year(week, i32::MAX, 1, &[]), None);
    }
}
