//! Select a numbered weekday within a fiscal quarter.

use chrono::{Datelike, Days, NaiveDate, Weekday};

use crate::fiscal_quarters::{end_of_fiscal_quarter, start_of_fiscal_quarter};

/// Find a weekday occurrence within the fiscal quarter containing `date`.
///
/// `first_month` is the fiscal year's starting month (1 through 12).
/// Positive occurrences count from the start, negative ones from the end.
/// Zero, out-of-range occurrences and unrepresentable bounds return None.
pub fn nth_weekday_of_fiscal_quarter(
    date: NaiveDate,
    first_month: u32,
    weekday: Weekday,
    occurrence: i16,
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }
    let first = start_of_fiscal_quarter(date, first_month)?;
    let last = end_of_fiscal_quarter(date, first_month)?;
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
    fn selects_weekdays_across_a_fiscal_year_boundary() {
        let january = day(2024, 1, 15);
        assert_eq!(
            nth_weekday_of_fiscal_quarter(january, 2, Weekday::Wed, 1),
            Some(day(2023, 11, 1))
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(january, 2, Weekday::Wed, -1),
            Some(day(2024, 1, 31))
        );
        let leap_day = day(2024, 2, 29);
        assert_eq!(
            nth_weekday_of_fiscal_quarter(leap_day, 2, Weekday::Thu, 1),
            Some(day(2024, 2, 1))
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(leap_day, 2, Weekday::Tue, -1),
            Some(day(2024, 4, 30))
        );
    }

    #[test]
    fn checks_months_occurrences_and_supported_bounds() {
        let date = day(2024, 2, 29);
        assert_eq!(
            nth_weekday_of_fiscal_quarter(date, 0, Weekday::Mon, 1),
            None
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(date, 13, Weekday::Mon, 1),
            None
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(date, 2, Weekday::Mon, 0),
            None
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(date, 2, Weekday::Mon, 20),
            None
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(date, 2, Weekday::Mon, i16::MIN),
            None
        );
        assert_eq!(
            nth_weekday_of_fiscal_quarter(NaiveDate::MAX, 2, Weekday::Mon, 1),
            None
        );
    }
}
