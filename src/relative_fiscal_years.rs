//! Adjacent fiscal-year comparisons with a caller-specified first month.

use chrono::{Datelike, NaiveDate};

fn fiscal_year_index(date: NaiveDate, first_month: u32) -> Option<i64> {
    if !(1..=12).contains(&first_month) {
        return None;
    }
    let preceding = if date.month() < first_month { 1 } else { 0 };
    Some(i64::from(date.year()) - preceding)
}

/// Whether target belongs to the fiscal year immediately after reference.
///
/// Returns None for first_month outside 1..=12.
pub fn is_next_fiscal_year(
    target: NaiveDate,
    reference: NaiveDate,
    first_month: u32,
) -> Option<bool> {
    Some(fiscal_year_index(target, first_month)? - fiscal_year_index(reference, first_month)? == 1)
}

/// Whether target belongs to the fiscal year immediately before reference.
pub fn is_previous_fiscal_year(
    target: NaiveDate,
    reference: NaiveDate,
    first_month: u32,
) -> Option<bool> {
    Some(fiscal_year_index(target, first_month)? - fiscal_year_index(reference, first_month)? == -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn compares_april_fiscal_years_at_the_boundary() {
        let march = date(2024, 3, 31);
        let april = date(2024, 4, 1);
        assert_eq!(is_next_fiscal_year(april, march, 4), Some(true));
        assert_eq!(is_previous_fiscal_year(march, april, 4), Some(true));
        assert_eq!(is_next_fiscal_year(april, march, 1), Some(false));
        assert_eq!(is_next_fiscal_year(date(2024, 6, 1), april, 4), Some(false));
        assert_eq!(is_next_fiscal_year(april, march, 0), None);
        assert_eq!(is_previous_fiscal_year(march, april, 13), None);
    }

    #[test]
    fn supports_negative_years_without_date_overflow() {
        assert_eq!(
            is_next_fiscal_year(date(0, 4, 1), date(0, 3, 31), 4),
            Some(true)
        );
        assert_eq!(
            is_next_fiscal_year(NaiveDate::MIN, NaiveDate::MAX, 4),
            Some(false)
        );
    }
}
