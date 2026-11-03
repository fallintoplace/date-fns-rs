//! Select configured workdays within a month with holiday exclusions.

use std::collections::BTreeSet;

use chrono::{Datelike, NaiveDate};

use crate::Workweek;

/// Select the nth allowed workday of a calendar month.
///
/// Positive positions count from the start, negative positions from the
/// end. Zero, out-of-range positions and invalid dates return None.
pub fn nth_workday_of_month(
    week: Workweek,
    year: i32,
    month: u32,
    occurrence: i8,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }

    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let last = crate::end_of_month(first);
    let blocked: BTreeSet<_> = holidays.iter().copied().collect();

    let workdays: Vec<_> = (1..=last.day())
        .filter_map(|day| NaiveDate::from_ymd_opt(year, month, day))
        .filter(|date| week.is_workday(*date) && !blocked.contains(date))
        .collect();

    let position = usize::from(occurrence.unsigned_abs());
    let index = if occurrence > 0 {
        position - 1
    } else {
        workdays.len().checked_sub(position)?
    };
    workdays.get(index).copied()
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use super::*;

    fn day(number: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, number).unwrap()
    }

    #[test]
    fn selects_early_and_late_sunday_to_thursday_workdays() {
        let week = Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap();
        assert_eq!(nth_workday_of_month(week, 2024, 3, 1, &[]), Some(day(3)));
        assert_eq!(
            nth_workday_of_month(week, 2024, 3, 1, &[day(3), day(3)]),
            Some(day(4))
        );
        assert_eq!(
            nth_workday_of_month(week, 2024, 3, 2, &[day(3)]),
            Some(day(5))
        );
        assert_eq!(nth_workday_of_month(week, 2024, 3, -1, &[]), Some(day(31)));
        assert_eq!(
            nth_workday_of_month(week, 2024, 3, -1, &[day(31)]),
            Some(day(28))
        );
    }

    #[test]
    fn checks_positions_and_unrepresentable_months() {
        let week = Workweek::from_mask(0x7f).unwrap();
        assert_eq!(nth_workday_of_month(week, 2024, 3, 0, &[]), None);
        assert_eq!(nth_workday_of_month(week, 2024, 3, i8::MIN, &[]), None);
        assert_eq!(nth_workday_of_month(week, 2024, 13, 1, &[]), None);
        assert_eq!(nth_workday_of_month(week, i32::MAX, 1, 1, &[]), None);
        assert_eq!(nth_workday_of_month(week, 2024, 3, 31, &[]), Some(day(31)));
    }
}
