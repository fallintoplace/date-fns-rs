//! Mean Gregorian year to day conversions.

/// Convert whole years to days using the Gregorian mean of 365.2425 days.
///
/// Matches the date-fns yearsToDays conversion for whole-year counts.
/// Results truncate toward zero and return None if outside the i64 range.
/// This is a duration conversion, not the number of days in particular years.
pub fn years_to_days(years: i64) -> Option<i64> {
    let scaled = i128::from(years) * 3_652_425;
    i64::try_from(scaled / 10_000).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_the_mean_gregorian_year_like_date_fns() {
        assert_eq!(years_to_days(0), Some(0));
        assert_eq!(years_to_days(1), Some(365));
        assert_eq!(years_to_days(2), Some(730));
        assert_eq!(years_to_days(4), Some(1460));
        assert_eq!(years_to_days(5), Some(1826));
        assert_eq!(years_to_days(400), Some(146097));
        assert_eq!(years_to_days(-5), Some(-1826));
    }

    #[test]
    fn rejects_out_of_range_results_without_overflow() {
        assert_eq!(years_to_days(i64::MAX), None);
        assert_eq!(years_to_days(i64::MIN), None);
        let maximum =
            i64::try_from(i128::from(i64::MAX) * 10_000 / 3_652_425).unwrap();
        let minimum =
            i64::try_from(i128::from(i64::MIN) * 10_000 / 3_652_425).unwrap();
        assert!(years_to_days(maximum).is_some());
        assert!(years_to_days(minimum).is_some());
        assert_eq!(years_to_days(maximum + 1), None);
        assert_eq!(years_to_days(minimum - 1), None);
    }
}
