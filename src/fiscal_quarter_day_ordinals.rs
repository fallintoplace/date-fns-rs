//! One-based day positions and lengths of configurable fiscal quarters.

use chrono::NaiveDate;

/// Return the one-based calendar day within a fiscal quarter.
///
/// The first fiscal month must be 1 through 12. Unrepresentable
/// quarter boundaries return None.
pub fn get_day_of_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<u32> {
    let start = crate::start_of_fiscal_quarter(date, first_month)?;
    u32::try_from(date.signed_duration_since(start).num_days() + 1).ok()
}

/// Return the complete length of the current fiscal quarter in days.
pub fn get_days_in_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<u32> {
    let start = crate::start_of_fiscal_quarter(date, first_month)?;
    let end = crate::end_of_fiscal_quarter(date, first_month)?;
    u32::try_from(end.signed_duration_since(start).num_days() + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_february_start_quarters_through_leap_day() {
        assert_eq!(get_day_of_fiscal_quarter(day(2024, 2, 29), 2), Some(29));
        assert_eq!(get_days_in_fiscal_quarter(day(2024, 2, 29), 2), Some(90));
        assert_eq!(get_day_of_fiscal_quarter(day(2024, 4, 30), 2), Some(90));
        assert_eq!(get_day_of_fiscal_quarter(day(2024, 5, 1), 2), Some(1));
        assert_eq!(get_days_in_fiscal_quarter(day(2024, 1, 31), 2), Some(92));
        assert_eq!(get_day_of_fiscal_quarter(day(2024, 1, 31), 2), Some(92));
    }

    #[test]
    fn rejects_invalid_months_and_unrepresentable_quarters() {
        let date = day(2024, 8, 15);
        assert_eq!(get_day_of_fiscal_quarter(date, 0), None);
        assert_eq!(get_days_in_fiscal_quarter(date, 13), None);
        assert_eq!(get_day_of_fiscal_quarter(NaiveDate::MIN, 2), None);
        assert!(get_days_in_fiscal_quarter(NaiveDate::MAX, 1).is_some());
    }
}
