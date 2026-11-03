//! Constant-time calendar-month counts for inclusive date intervals.

use chrono::NaiveDate;

/// Count the distinct Gregorian calendar months intersecting [start, end].
///
/// Both endpoints are included, even for an interval of one day.
/// Returns None for reversed date intervals.
pub fn count_calendar_months_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_months(end, start) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_leap_month_and_calendar_year_transitions() {
        assert_eq!(
            count_calendar_months_in_interval(day(2023, 12, 31), day(2024, 2, 29)),
            Some(3)
        );
        assert_eq!(
            count_calendar_months_in_interval(day(2024, 2, 29), day(2024, 2, 29)),
            Some(1)
        );
        assert_eq!(
            count_calendar_months_in_interval(day(2024, 2, 29), day(2024, 3, 1)),
            Some(2)
        );
        assert_eq!(
            count_calendar_months_in_interval(day(2024, 3, 1), day(2024, 2, 29)),
            None
        );
    }

    #[test]
    fn handles_the_complete_chrono_date_range() {
        let total = count_calendar_months_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert!(total > 0);
        assert_eq!(
            count_calendar_months_in_interval(NaiveDate::MAX, NaiveDate::MAX),
            Some(1)
        );
    }
}
