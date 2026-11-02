//! Business/252 year fractions on Monday-Friday holiday calendars.

use chrono::NaiveDate;

/// Compute the signed Business/252 fraction over the half-open [start, end).
///
/// Only Monday-Friday dates not in the holiday list contribute. A year is
/// always defined as 252 business days, regardless of actual holidays.
pub fn business_252_year_fraction(
    start: NaiveDate,
    end: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<f64> {
    if start > end {
        return Some(-business_252_year_fraction(end, start, holidays)?);
    }
    if start == end {
        return Some(0.0);
    }
    let last = end.pred_opt()?;
    let days = crate::count_business_days_in_interval_excluding_holidays(start, last, holidays)?;
    Some(days as f64 / 252.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn excludes_weekends_holidays_and_the_end_date() {
        let start = date(2024, 3, 1);
        let end = date(2024, 3, 6);
        let holidays = [date(2024, 3, 4), date(2024, 3, 4)];
        assert_eq!(
            business_252_year_fraction(start, end, &holidays),
            Some(2.0 / 252.0)
        );
        assert_eq!(
            business_252_year_fraction(start, end, &[]),
            Some(3.0 / 252.0)
        );
    }

    #[test]
    fn reverses_sign_and_handles_empty_intervals() {
        let start = date(2024, 3, 1);
        let end = date(2024, 3, 6);
        let forward = business_252_year_fraction(start, end, &[]).unwrap();
        assert_eq!(business_252_year_fraction(end, start, &[]), Some(-forward));
        assert_eq!(business_252_year_fraction(start, start, &[]), Some(0.0));
        assert_eq!(
            business_252_year_fraction(NaiveDate::MAX, NaiveDate::MAX, &[]),
            Some(0.0)
        );
    }
}
