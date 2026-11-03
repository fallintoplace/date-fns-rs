//! Business-day totals and ordinals within fiscal quarters.

use chrono::NaiveDate;

/// Count Monday-Friday dates in the fiscal quarter containing `date`.
///
/// `first_month` is the fiscal year's start (1 through 12).
/// Returns None for invalid months or unrepresentable period boundaries.
pub fn get_business_days_in_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<u32> {
    let first = crate::start_of_fiscal_quarter(date, first_month)?;
    let last = crate::end_of_fiscal_quarter(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval(first, last)?).ok()
}

/// Return the one-based business-day ordinal in the fiscal quarter.
///
/// Returns None on weekends or when the fiscal start is invalid.
pub fn get_business_day_of_fiscal_quarter(date: NaiveDate, first_month: u32) -> Option<u32> {
    if !crate::is_business_day(date) {
        return None;
    }
    let first = crate::start_of_fiscal_quarter(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval(first, date)?).ok()
}

/// Count fiscal quarter workdays excluding supplied holidays.
///
/// Duplicate and weekend holidays are ignored. Returns None for invalid
/// months or unrepresentable period boundaries.
pub fn get_business_days_in_fiscal_quarter_with_holidays(
    date: NaiveDate,
    first_month: u32,
    holidays: &[NaiveDate],
) -> Option<u32> {
    let first = crate::start_of_fiscal_quarter(date, first_month)?;
    let last = crate::end_of_fiscal_quarter(date, first_month)?;
    u32::try_from(crate::count_business_days_in_interval_excluding_holidays(
        first, last, holidays,
    )?)
    .ok()
}

/// Return the holiday-adjusted one-based business-day ordinal.
///
/// Returns None for weekends, excluded holidays, or invalid fiscal starts.
pub fn get_business_day_of_fiscal_quarter_with_holidays(
    date: NaiveDate,
    first_month: u32,
    holidays: &[NaiveDate],
) -> Option<u32> {
    if !crate::is_business_day(date) || holidays.contains(&date) {
        return None;
    }
    let first = crate::start_of_fiscal_quarter(date, first_month)?;
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
        assert_eq!(get_business_days_in_fiscal_quarter(date, 2), Some(66));
        assert_eq!(
            get_business_day_of_fiscal_quarter(day(2023, 11, 1), 2),
            Some(1)
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter(day(2024, 1, 1), 2),
            Some(44)
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter(day(2024, 1, 31), 2),
            Some(66)
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter(day(2024, 1, 28), 2),
            None
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter(day(2024, 2, 1), 2),
            Some(1)
        );
    }

    #[test]
    fn excludes_unique_weekday_holidays() {
        let date = day(2024, 1, 15);
        let holidays = [
            day(2023, 11, 1),
            day(2024, 1, 31),
            day(2024, 1, 31),
            day(2023, 11, 4),
            day(2024, 2, 1),
        ];
        assert_eq!(
            get_business_days_in_fiscal_quarter_with_holidays(date, 2, &holidays),
            Some(64)
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter_with_holidays(day(2023, 11, 2), 2, &holidays),
            Some(1)
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter_with_holidays(day(2024, 1, 30), 2, &holidays),
            Some(64)
        );
        assert_eq!(
            get_business_day_of_fiscal_quarter_with_holidays(day(2024, 1, 31), 2, &holidays),
            None
        );
        assert_eq!(
            get_business_days_in_fiscal_quarter_with_holidays(date, 2, &[]),
            get_business_days_in_fiscal_quarter(date, 2)
        );
    }

    #[test]
    fn validates_fiscal_start_months_and_chrono_limits() {
        let date = day(2024, 5, 1);
        assert_eq!(get_business_days_in_fiscal_quarter(date, 0), None);
        assert_eq!(get_business_days_in_fiscal_quarter(date, 13), None);
        assert_eq!(get_business_day_of_fiscal_quarter(date, 0), None);
        assert_eq!(
            get_business_day_of_fiscal_quarter_with_holidays(date, 13, &[]),
            None
        );
        assert_eq!(get_business_days_in_fiscal_quarter(NaiveDate::MAX, 2), None);
        assert_eq!(
            get_business_days_in_fiscal_quarter_with_holidays(NaiveDate::MAX, 2, &[]),
            None
        );
    }
}
