//! Calendar-year arithmetic that clamps leap-day dates.

use chrono::NaiveDate;

use crate::months::{add_months, sub_months};

/// Add signed calendar years, clamping February 29 in non-leap years.
///
/// Returns `None` if the resulting date or year-to-month conversion overflows.
pub fn add_years(date: NaiveDate, years: i32) -> Option<NaiveDate> {
    add_months(date, years.checked_mul(12)?)
}

/// Subtract signed calendar years, clamping February 29 where necessary.
///
/// Returns `None` if the resulting date or year-to-month conversion overflows.
pub fn sub_years(date: NaiveDate, years: i32) -> Option<NaiveDate> {
    sub_months(date, years.checked_mul(12)?)
}

/// Add years while preserving the end-of-month position of the source.
///
/// February 28, 2023 plus one year becomes February 29, 2024.
pub fn add_years_preserving_end_of_month(date: NaiveDate, years: i32) -> Option<NaiveDate> {
    use crate::boundaries::end_of_month;

    let shifted = add_years(date, years)?;
    Some(if date == end_of_month(date) {
        end_of_month(shifted)
    } else {
        shifted
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn clamps_leap_days() {
        assert_eq!(add_years(date(2020, 2, 29), 1), Some(date(2021, 2, 28)));
        assert_eq!(add_years(date(2020, 2, 29), 4), Some(date(2024, 2, 29)));
        assert_eq!(sub_years(date(2024, 2, 29), 1), Some(date(2023, 2, 28)));
    }

    #[test]
    fn handles_negative_and_zero_offsets() {
        assert_eq!(add_years(date(2024, 6, 15), -1), Some(date(2023, 6, 15)));
        assert_eq!(sub_years(date(2023, 6, 15), -1), Some(date(2024, 6, 15)));
        assert_eq!(add_years(date(2024, 6, 15), 0), Some(date(2024, 6, 15)));
    }

    #[test]
    fn returns_none_on_range_overflow() {
        assert_eq!(add_years(NaiveDate::MAX, 1), None);
        assert_eq!(sub_years(NaiveDate::MIN, 1), None);
        assert_eq!(add_years(date(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_years(date(2024, 1, 1), i32::MIN), None);
    }
    #[test]
    fn preserves_end_of_february_across_leap_years() {
        assert_eq!(
            add_years_preserving_end_of_month(date(2023, 2, 28), 1),
            Some(date(2024, 2, 29))
        );
        assert_eq!(
            add_years_preserving_end_of_month(date(2024, 2, 29), 1),
            Some(date(2025, 2, 28))
        );
        assert_eq!(
            add_years_preserving_end_of_month(date(2024, 2, 28), 1),
            Some(date(2025, 2, 28))
        );
    }

    #[test]
    fn checks_year_range_overflow() {
        assert_eq!(add_years_preserving_end_of_month(NaiveDate::MAX, 1), None);
        assert_eq!(add_years_preserving_end_of_month(NaiveDate::MIN, -1), None);
    }
}
