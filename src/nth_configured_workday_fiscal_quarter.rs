//! Select configured workdays within a fiscal quarter.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::Workweek;

/// Select the nth configured workday in the fiscal quarter containing `date`.
///
/// The fiscal year starts in `first_month` (1 through 12).
/// Positive positions count forward, negative positions count backward.
/// Holidays can be unordered or duplicated. Invalid or out-of-range
/// periods and positions return None without leaving the fiscal period.
pub fn nth_workday_of_fiscal_quarter(
    date: NaiveDate,
    first_month: u32,
    week: Workweek,
    occurrence: i16,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }
    let first = crate::start_of_fiscal_quarter(date, first_month)?;
    let last = crate::end_of_fiscal_quarter(date, first_month)?;
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
    use super::*;
    use chrono::Weekday;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn standard_week() -> Workweek {
        Workweek::new(&[
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
        ])
        .unwrap()
    }

    #[test]
    fn selects_workdays_in_a_cross_year_quarter() {
        let week = standard_week();
        let date = day(2024, 1, 15);
        assert_eq!(
            nth_workday_of_fiscal_quarter(date, 2, week, 1, &[]),
            Some(day(2023, 11, 1))
        );
        assert_eq!(
            nth_workday_of_fiscal_quarter(date, 2, week, -1, &[]),
            Some(day(2024, 1, 31))
        );
        let holidays = [day(2023, 11, 1), day(2024, 1, 31), day(2023, 11, 1)];
        assert_eq!(
            nth_workday_of_fiscal_quarter(date, 2, week, 1, &holidays),
            Some(day(2023, 11, 2))
        );
        assert_eq!(
            nth_workday_of_fiscal_quarter(date, 2, week, -1, &holidays),
            Some(day(2024, 1, 30))
        );
    }

    #[test]
    fn rejects_invalid_months_and_positions() {
        let week = standard_week();
        let date = day(2024, 6, 1);
        assert_eq!(nth_workday_of_fiscal_quarter(date, 0, week, 1, &[]), None);
        assert_eq!(nth_workday_of_fiscal_quarter(date, 13, week, 1, &[]), None);
        assert_eq!(nth_workday_of_fiscal_quarter(date, 4, week, 0, &[]), None);
        assert_eq!(
            nth_workday_of_fiscal_quarter(date, 4, week, i16::MIN, &[]),
            None
        );
        assert_eq!(
            nth_workday_of_fiscal_quarter(NaiveDate::MAX, 2, week, 1, &[]),
            None
        );
    }
}
