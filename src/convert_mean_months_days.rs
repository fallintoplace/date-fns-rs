//! Approximate month-duration conversions based on the Gregorian 400-year cycle.

/// Convert a whole-day duration into complete mean Gregorian months.
///
/// One mean month is 146097 / 4800 days. The result truncates toward zero;
/// it is not the number of calendar months between two particular dates.
pub fn days_to_mean_months(days: i64) -> i64 {
    (i128::from(days) * 4_800 / 146_097) as i64
}

/// Convert whole mean Gregorian months into days, truncating toward zero.
///
/// Returns None if the resulting number of days cannot fit in i64.
pub fn mean_months_to_days(months: i64) -> Option<i64> {
    i64::try_from(i128::from(months) * 146_097 / 4_800).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_with_the_400_year_gregorian_average() {
        assert_eq!(days_to_mean_months(0), 0);
        assert_eq!(days_to_mean_months(30), 0);
        assert_eq!(days_to_mean_months(31), 1);
        assert_eq!(days_to_mean_months(365), 11);
        assert_eq!(days_to_mean_months(366), 12);
        assert_eq!(days_to_mean_months(146_097), 4_800);
        assert_eq!(days_to_mean_months(-146_097), -4_800);
        assert_eq!(mean_months_to_days(1), Some(30));
        assert_eq!(mean_months_to_days(-1), Some(-30));
        assert_eq!(mean_months_to_days(12), Some(365));
        assert_eq!(mean_months_to_days(4_800), Some(146_097));
    }

    #[test]
    fn checks_large_results_without_panicking() {
        assert_eq!(mean_months_to_days(i64::MAX), None);
        assert_eq!(mean_months_to_days(i64::MIN), None);
        assert!(days_to_mean_months(i64::MAX) > 0);
        assert!(days_to_mean_months(i64::MIN) < 0);
    }
}
