//! Fiscal month labels for years starting in any calendar month.

use chrono::{Datelike, NaiveDate};

/// Return the 1-based month within a fiscal year starting in first_month.
///
/// Returns None unless first_month is in 1..=12.
pub fn get_fiscal_month(date: NaiveDate, first_month: u32) -> Option<u32> {
    (1..=12)
        .contains(&first_month)
        .then_some((date.month() + 12 - first_month) % 12 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn labels_months_across_calendar_boundaries() {
        let january = date(2024, 1, 31);
        assert_eq!(get_fiscal_month(january, 4), Some(10));
        assert_eq!(get_fiscal_month(date(2024, 3, 1), 4), Some(12));
        assert_eq!(get_fiscal_month(date(2024, 4, 1), 4), Some(1));
        assert_eq!(get_fiscal_month(date(2024, 12, 1), 1), Some(12));
        assert_eq!(get_fiscal_month(date(2024, 1, 1), 1), Some(1));
    }

    #[test]
    fn rejects_invalid_fiscal_starts() {
        let day = date(2024, 2, 29);
        assert_eq!(get_fiscal_month(day, 0), None);
        assert_eq!(get_fiscal_month(day, 13), None);
        assert_eq!(get_fiscal_month(day, u32::MAX), None);
        assert_eq!(get_fiscal_month(NaiveDate::MIN, 1), Some(1));
    }
}
