//! Relative-day predicates with an explicit reference date and no clock access.

use chrono::NaiveDate;

/// Return whether a date equals the caller-provided local calendar day.
pub fn is_today(date: NaiveDate, reference: NaiveDate) -> bool {
    date == reference
}

/// Return whether a date is the day immediately after the reference.
///
/// Returns false when the reference has no successor in Chrono's range.
pub fn is_tomorrow(date: NaiveDate, reference: NaiveDate) -> bool {
    reference.succ_opt() == Some(date)
}

/// Return whether a date is the day immediately before the reference.
///
/// Returns false when the reference has no predecessor in Chrono's range.
pub fn is_yesterday(date: NaiveDate, reference: NaiveDate) -> bool {
    reference.pred_opt() == Some(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn classifies_days_across_year_and_leap_day_boundaries() {
        let reference = date(2024, 12, 31);
        assert!(is_today(reference, reference));
        assert!(is_tomorrow(date(2025, 1, 1), reference));
        assert!(is_yesterday(date(2024, 12, 30), reference));
        assert!(!is_tomorrow(reference, reference));
        assert!(!is_yesterday(date(2025, 1, 1), reference));
        assert!(is_tomorrow(date(2024, 2, 29), date(2024, 2, 28)));
    }

    #[test]
    fn does_not_wrap_dates_at_chrono_limits() {
        assert!(!is_tomorrow(NaiveDate::MIN, NaiveDate::MAX));
        assert!(!is_yesterday(NaiveDate::MAX, NaiveDate::MIN));
        assert!(is_today(NaiveDate::MAX, NaiveDate::MAX));
    }
}
