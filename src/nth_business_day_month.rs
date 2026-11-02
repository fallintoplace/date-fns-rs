//! Select business days within a month with explicit holiday exclusions.

use std::collections::BTreeSet;

use chrono::{Datelike, NaiveDate};

/// Return the nth Monday-Friday business date in a calendar month.
///
/// Positive positions count from the beginning, and negative positions from
/// the end. The holiday list may be unsorted or contain duplicates. Zero,
/// nonexisting positions, and invalid dates return None.
pub fn nth_business_day_of_month(
    year: i32,
    month: u32,
    occurrence: i8,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if occurrence == 0 {
        return None;
    }
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let last = crate::end_of_month(first);
    let blocked: BTreeSet<_> = holidays.iter().copied().collect();
    let business: Vec<_> = (1..=last.day())
        .filter_map(|day| NaiveDate::from_ymd_opt(year, month, day))
        .filter(|date| date.weekday().num_days_from_monday() < 5 && !blocked.contains(date))
        .collect();

    let nth = usize::from(occurrence.unsigned_abs());
    let index = if occurrence > 0 {
        nth - 1
    } else {
        business.len().checked_sub(nth)?
    };
    business.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 2, n).unwrap()
    }

    #[test]
    fn selects_first_last_and_holiday_excluded_working_days() {
        assert_eq!(nth_business_day_of_month(2024, 2, 1, &[]), Some(day(1)));
        assert_eq!(nth_business_day_of_month(2024, 2, 2, &[]), Some(day(2)));
        assert_eq!(nth_business_day_of_month(2024, 2, -1, &[]), Some(day(29)));
        assert_eq!(
            nth_business_day_of_month(2024, 2, -1, &[day(29), day(29), day(1)]),
            Some(day(28))
        );
    }

    #[test]
    fn validates_positions_and_leap_year_boundaries() {
        assert_eq!(nth_business_day_of_month(2024, 2, 0, &[]), None);
        assert_eq!(nth_business_day_of_month(2024, 2, 32, &[]), None);
        assert_eq!(nth_business_day_of_month(2024, 2, -32, &[]), None);
        assert_eq!(
            nth_business_day_of_month(2023, 2, -1, &[]),
            Some(NaiveDate::from_ymd_opt(2023, 2, 28).unwrap())
        );
        assert_eq!(nth_business_day_of_month(2024, 13, 1, &[]), None);
        assert_eq!(nth_business_day_of_month(i32::MAX, 1, 1, &[]), None);
        assert_eq!(nth_business_day_of_month(2024, 2, i8::MIN, &[]), None);
    }
}
