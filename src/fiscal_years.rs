//! Fiscal years with an explicit starting calendar month.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::end_of_month;

/// Find the first day of the fiscal year containing `date`.
///
/// `first_month` must be in 1..=12. Returns `None` for invalid
/// months or dates whose fiscal-year start is outside Chrono's range.
pub fn start_of_fiscal_year(date: NaiveDate, first_month: u32) -> Option<NaiveDate> {
    if !(1..=12).contains(&first_month) {
        return None;
    }
    let year = if date.month() < first_month {
        date.year().checked_sub(1)?
    } else {
        date.year()
    };
    NaiveDate::from_ymd_opt(year, first_month, 1)
}

/// Return the starting calendar year used to label a fiscal year.
///
/// For example, March 2024 belongs to fiscal year 2023 when the fiscal
/// year starts in April.
pub fn get_fiscal_year(date: NaiveDate, first_month: u32) -> Option<i32> {
    Some(start_of_fiscal_year(date, first_month)?.year())
}

/// Find the last inclusive calendar day of the date's fiscal year.
///
/// Returns `None` when the boundary would exceed Chrono's date range.
pub fn end_of_fiscal_year(date: NaiveDate, first_month: u32) -> Option<NaiveDate> {
    let start = start_of_fiscal_year(date, first_month)?;
    let (year, month) = if first_month == 1 {
        (start.year(), 12)
    } else {
        (start.year().checked_add(1)?, first_month - 1)
    };
    Some(end_of_month(NaiveDate::from_ymd_opt(year, month, 1)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn labels_and_bounds_april_to_march_fiscal_years() {
        let day = date(2024, 2, 29);
        assert_eq!(get_fiscal_year(day, 4), Some(2023));
        assert_eq!(start_of_fiscal_year(day, 4), Some(date(2023, 4, 1)));
        assert_eq!(end_of_fiscal_year(day, 4), Some(date(2024, 3, 31)));
        assert_eq!(get_fiscal_year(date(2024, 4, 1), 4), Some(2024));
    }

    #[test]
    fn handles_january_and_december_starts() {
        let day = date(2024, 12, 31);
        assert_eq!(start_of_fiscal_year(day, 1), Some(date(2024, 1, 1)));
        assert_eq!(end_of_fiscal_year(day, 1), Some(day));
        assert_eq!(
            start_of_fiscal_year(date(2024, 1, 1), 12),
            Some(date(2023, 12, 1))
        );
        assert_eq!(
            end_of_fiscal_year(date(2024, 1, 1), 12),
            Some(date(2024, 11, 30))
        );
    }

    #[test]
    fn validates_months_and_rejects_unrepresentable_boundaries() {
        let day = date(2024, 6, 1);
        assert_eq!(get_fiscal_year(day, 0), None);
        assert_eq!(start_of_fiscal_year(day, 13), None);
        assert_eq!(end_of_fiscal_year(day, u32::MAX), None);
        assert_eq!(
            start_of_fiscal_year(NaiveDate::MIN, 1),
            Some(NaiveDate::MIN)
        );
        assert_eq!(
            end_of_fiscal_year(NaiveDate::MAX, 1),
            Some(NaiveDate::MAX)
        );
        assert_eq!(end_of_fiscal_year(NaiveDate::MAX, 2), None);
    }
}
