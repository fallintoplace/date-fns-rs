//! Select a numbered weekday within a fiscal year.

use chrono::{Datelike, Days, NaiveDate, Weekday};

use crate::fiscal_years::{end_of_fiscal_year, start_of_fiscal_year};

/// Find a numbered weekday in the fiscal year containing `date`.
///
/// `first_month` is the fiscal year start (1 through 12).
/// Positive occurrences count forward, negative ones from the end.
/// Zero, out-of-range occurrences and invalid boundaries return None.
pub fn nth_weekday_of_fiscal_year(
    date: NaiveDate,
    first_month: u32,
    weekday: Weekday,
    occurrence: i16,
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }
    let first = start_of_fiscal_year(date, first_month)?;
    let last = end_of_fiscal_year(date, first_month)?;
    let target = i64::from(weekday.num_days_from_monday());
    let weeks = (i64::from(occurrence).abs() - 1) * 7;
    let selected = if occurrence > 0 {
        let current = i64::from(first.weekday().num_days_from_monday());
        let offset = (target - current).rem_euclid(7) + weeks;
        first.checked_add_days(Days::new(u64::try_from(offset).ok()?))?
    } else {
        let current = i64::from(last.weekday().num_days_from_monday());
        let offset = (current - target).rem_euclid(7) + weeks;
        last.checked_sub_days(Days::new(u64::try_from(offset).ok()?))?
    };
    (first..=last).contains(&selected).then_some(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn selects_weekdays_across_fiscal_years() {
        let january = day(2025, 1, 15);
        assert_eq!(
            nth_weekday_of_fiscal_year(january, 4, Weekday::Mon, 1),
            Some(day(2024, 4, 1))
        );
        assert_eq!(
            nth_weekday_of_fiscal_year(january, 4, Weekday::Mon, -1),
            Some(day(2025, 3, 31))
        );
        assert_eq!(
            nth_weekday_of_fiscal_year(day(2024, 2, 29), 4, Weekday::Thu, -5),
            Some(day(2024, 2, 29))
        );
    }

    #[test]
    fn rejects_invalid_months_and_occurrences() {
        let date = day(2024, 2, 29);
        assert_eq!(nth_weekday_of_fiscal_year(date, 0, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_fiscal_year(date, 13, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_fiscal_year(date, 4, Weekday::Mon, 0), None);
        assert_eq!(nth_weekday_of_fiscal_year(date, 4, Weekday::Mon, 100), None);
        assert_eq!(
            nth_weekday_of_fiscal_year(date, 4, Weekday::Mon, i16::MIN),
            None
        );
    }
}
