//! Calendar-month arithmetic with end-of-month clamping.

use chrono::{Months, NaiveDate};

/// Add signed calendar months, clamping to the last day of the target month.
///
/// For example, January 31 plus one month is February 28 (or 29 in a leap year).
/// Returns `None` if the target date is outside Chrono's supported range.
pub fn add_months(date: NaiveDate, months: i32) -> Option<NaiveDate> {
    if months >= 0 {
        date.checked_add_months(Months::new(months as u32))
    } else {
        date.checked_sub_months(Months::new(months.unsigned_abs()))
    }
}

/// Subtract signed calendar months, clamping to the last valid day.
///
/// Returns `None` if the target date is out of range.
pub fn sub_months(date: NaiveDate, months: i32) -> Option<NaiveDate> {
    if months >= 0 {
        date.checked_sub_months(Months::new(months as u32))
    } else {
        date.checked_add_months(Months::new(months.unsigned_abs()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn clamps_end_of_month() {
        assert_eq!(add_months(date(2024, 1, 31), 1), Some(date(2024, 2, 29)));
        assert_eq!(add_months(date(2023, 1, 31), 1), Some(date(2023, 2, 28)));
        assert_eq!(sub_months(date(2024, 3, 31), 1), Some(date(2024, 2, 29)));
        assert_eq!(add_months(date(2024, 2, 29), 12), Some(date(2025, 2, 28)));
    }

    #[test]
    fn handles_negative_months_and_year_boundaries() {
        assert_eq!(add_months(date(2024, 1, 31), -1), Some(date(2023, 12, 31)));
        assert_eq!(sub_months(date(2023, 12, 31), -1), Some(date(2024, 1, 31)));
        assert_eq!(add_months(date(2024, 1, 1), 0), Some(date(2024, 1, 1)));
    }

    #[test]
    fn extreme_offsets_return_none() {
        assert_eq!(add_months(NaiveDate::MAX, 1), None);
        assert_eq!(sub_months(NaiveDate::MIN, 1), None);
        assert_eq!(add_months(date(2024, 1, 1), i32::MIN), None);
        assert_eq!(sub_months(date(2024, 1, 1), i32::MAX), None);
    }
}
