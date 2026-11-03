//! Business-day totals and ordinals within fiscal half-years.

use chrono::NaiveDate;

/// Count Monday-Friday dates in the fiscal half-year containing `date`.
///
/// `first_month` is the fiscal year's start (1 through 12).
/// Returns None for invalid months or unrepresentable period boundaries.
pub fn get_business_days_in_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    let first = crate::start_of_fiscal_half_year(date, first_month)?;
    let last = crate::end_of_fiscal_half_year(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval(first, last)?).ok()
}

/// Return the one-based business-day ordinal in the fiscal half-year.
///
/// Returns None on weekends or when the fiscal start is invalid.
pub fn get_business_day_of_fiscal_half_year(date: NaiveDate, first_month: u32) -> Option<u32> {
    if !crate::is_business_day(date) {
        return None;
    }
    let first = crate::start_of_fiscal_half_year(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval(first, date)?).ok()
}

/// Count fiscal half-year workdays excluding supplied holidays.
///
/// Duplicate and weekend holidays are ignored. Returns None for invalid
/// months or unrepresentable period boundaries.
pub fn get_business_days_in_fiscal_half_year_with_holidays(
    date: NaiveDate,
    first_month: u32,
    holidays: &[NaiveDate],
) -> Option<u32> {
    let first = crate::start_of_fiscal_half_year(date, first_month)?;
    let last = crate::end_of_fiscal_half_year(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval_excluding_holidays(
        first, last, holidays,
    )?)
    .ok()
}

/// Return the holiday-adjusted one-based business-day ordinal.
///
/// Returns None for weekends, excluded holidays, or invalid fiscal starts.
pub fn get_business_day_of_fiscal_half_year_with_holidays(
    date: NaiveDate,
    first_month: u32,
    holidays: &[NaiveDate],
) -> Option<u32> {
    if !crate::is_business_day(date) || holidays.contains(&date) {
        return None;
    }
    let first = crate::start_of_fiscal_half_year(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval_excluding_holidays(
        first, date, holidays,
    )?)
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_cross_year_fiscal_workdays() {
        let date = day(2024, 1, 15);
        assert_eq!(get_business_days_in_fiscal_half_year(date, 4), Some(130));
        assert_eq!(
            get_business_day_of_fiscal_half_year(day(2023, 10, 2), 4),
            Some(1)
        );
        assert_eq!(
            get_business_day_of_fiscal_half_year(day(2024, 1, 2), 4),
            Some(67)
        );
        assert_eq!(
            get_business_day_of_fiscal_half_year(day(2024, 3, 29), 4),
            Some(130)
        );
        assert_eq!(
            get_business_day_of_fiscal_half_year(day(2024, 3, 31), 4),
            None
        );
    }

    #[test]
    fn excludes_unique_weekday_holidays() {
        let date = day(2024, 1, 15);
        let holidays = [
            day(2023, 10, 2),
            day(2024, 2, 29),
            day(2024, 2, 29),
            day(2024, 3, 30),
            day(2024, 4, 1),
        ];
        assert_eq!(
            get_business_days_in_fiscal_half_year_with_holidays(date, 4, &holidays),
            Some(128)
        );
        assert_eq!(
            get_business_day_of_fiscal_half_year_with_holidays(day(2024, 1, 2), 4, &holidays),
            Some(66)
        );
        assert_eq!(
            get_business_day_of_fiscal_half_year_with_holidays(day(2024, 3, 29), 4, &holidays),
            Some(128)
        );
        assert_eq!(
            get_business_day_of_fiscal_half_year_with_holidays(day(2024, 2, 29), 4, &holidays),
            None
        );
        assert_eq!(
            get_business_days_in_fiscal_half_year_with_holidays(date, 4, &[]),
            get_business_days_in_fiscal_half_year(date, 4)
        );
    }

    #[test]
    fn validates_fiscal_start_months_and_chrono_limits() {
        let date = day(2024, 5, 1);
        assert_eq!(get_business_days_in_fiscal_half_year(date, 0), None);
        assert_eq!(get_business_days_in_fiscal_half_year(date, 13), None);
        assert_eq!(get_business_day_of_fiscal_half_year(date, 0), None);
        assert_eq!(
            get_business_day_of_fiscal_half_year_with_holidays(date, 13, &[]),
            None
        );
        assert_eq!(
            get_business_days_in_fiscal_half_year(NaiveDate::MAX, 4),
            None
        );
        assert_eq!(
            get_business_days_in_fiscal_half_year_with_holidays(NaiveDate::MAX, 4, &[]),
            None
        );
    }
}
