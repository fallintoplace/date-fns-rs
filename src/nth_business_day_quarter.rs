//! Select the nth business day of a Gregorian calendar quarter.

use chrono::{Datelike, NaiveDate};
use std::collections::BTreeSet;

/// Return the nth Monday-Friday date in a calendar quarter.
///
/// Positive positions count from the start; negative positions count from the
/// end. Zero and unavailable positions return None. Holidays can be unsorted,
/// repeated, or outside the quarter. The search visits at most 92 dates.
pub fn nth_business_day_of_quarter(
    year: i32,
    quarter: u32,
    occurrence: i16,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if !(1..=4).contains(&quarter) || occurrence == 0 {
        return None;
    }
    let start = NaiveDate::from_ymd_opt(year, (quarter - 1) * 3 + 1, 1)?;
    let end = crate::end_of_quarter(start);
    let blocked: BTreeSet<_> = holidays.iter().copied().collect();
    let mut days = Vec::new();
    let mut cursor = start;
    loop {
        if cursor.weekday().num_days_from_monday() < 5 && !blocked.contains(&cursor) {
            days.push(cursor);
        }
        if cursor == end {
            break;
        }
        cursor = cursor.succ_opt()?;
    }
    let n = usize::from(occurrence.unsigned_abs());
    let index = if occurrence > 0 {
        n.checked_sub(1)?
    } else {
        days.len().checked_sub(n)?
    };
    days.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn selects_quarter_end_workdays_and_excludes_duplicates() {
        assert_eq!(
            nth_business_day_of_quarter(2024, 1, 1, &[]),
            Some(date(2024, 1, 1))
        );
        assert_eq!(
            nth_business_day_of_quarter(2024, 1, -1, &[]),
            Some(date(2024, 3, 29))
        );
        let holidays = [date(2024, 1, 1), date(2024, 3, 29), date(2024, 3, 29)];
        assert_eq!(
            nth_business_day_of_quarter(2024, 1, 1, &holidays),
            Some(date(2024, 1, 2))
        );
        assert_eq!(
            nth_business_day_of_quarter(2024, 1, -1, &holidays),
            Some(date(2024, 3, 28))
        );
    }

    #[test]
    fn handles_cross_month_and_year_end_quarters() {
        assert_eq!(
            nth_business_day_of_quarter(2024, 2, 1, &[]),
            Some(date(2024, 4, 1))
        );
        assert_eq!(
            nth_business_day_of_quarter(2024, 4, -1, &[]),
            Some(date(2024, 12, 31))
        );
        assert_eq!(
            nth_business_day_of_quarter(2023, 4, -1, &[]),
            Some(date(2023, 12, 29))
        );
    }

    #[test]
    fn rejects_invalid_quarters_positions_and_years() {
        assert_eq!(nth_business_day_of_quarter(2024, 0, 1, &[]), None);
        assert_eq!(nth_business_day_of_quarter(2024, 5, 1, &[]), None);
        assert_eq!(nth_business_day_of_quarter(2024, 1, 0, &[]), None);
        assert_eq!(nth_business_day_of_quarter(2024, 1, 100, &[]), None);
        assert_eq!(nth_business_day_of_quarter(2024, 1, i16::MIN, &[]), None);
        assert_eq!(nth_business_day_of_quarter(i32::MAX, 1, 1, &[]), None);
    }
}
