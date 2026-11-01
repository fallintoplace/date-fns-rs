//! Checked arithmetic in six-month calendar increments.

use chrono::NaiveDate;

use crate::months::{add_months, sub_months};

/// Add signed half-years, clamping missing target days like month arithmetic.
pub fn add_half_years(date: NaiveDate, half_years: i32) -> Option<NaiveDate> {
    add_months(date, half_years.checked_mul(6)?)
}

/// Subtract signed half-years, returning None on range or offset overflow.
pub fn sub_half_years(date: NaiveDate, half_years: i32) -> Option<NaiveDate> {
    sub_months(date, half_years.checked_mul(6)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn shifts_half_years_and_clamps_february() {
        assert_eq!(add_half_years(date(2024, 1, 31), 1), Some(date(2024, 7, 31)));
        assert_eq!(add_half_years(date(2023, 8, 31), 1), Some(date(2024, 2, 29)));
        assert_eq!(sub_half_years(date(2024, 8, 31), 1), Some(date(2024, 2, 29)));
        assert_eq!(add_half_years(date(2024, 8, 31), -1), Some(date(2024, 2, 29)));
        assert_eq!(sub_half_years(date(2024, 8, 31), -1), Some(date(2025, 2, 28)));
    }

    #[test]
    fn rejects_offsets_and_dates_out_of_range() {
        assert_eq!(add_half_years(date(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_half_years(date(2024, 1, 1), i32::MIN), None);
        assert_eq!(add_half_years(NaiveDate::MAX, 1), None);
        assert_eq!(sub_half_years(NaiveDate::MIN, 1), None);
        assert_eq!(add_half_years(NaiveDate::MAX, 0), Some(NaiveDate::MAX));
    }
}
