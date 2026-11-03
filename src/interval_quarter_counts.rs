//! Constant-time quarter counts for inclusive Gregorian date intervals.

use chrono::NaiveDate;

/// Count the distinct calendar quarters intersecting [start, end].
///
/// Quarter boundaries are January, April, July and October. Both
/// endpoints are included, and reversed intervals return None.
pub fn count_calendar_quarters_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_quarters(end, start) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_quarters_at_month_and_year_boundaries() {
        assert_eq!(
            count_calendar_quarters_in_interval(day(2024, 2, 29), day(2024, 4, 1)),
            Some(2)
        );
        assert_eq!(
            count_calendar_quarters_in_interval(day(2023, 12, 31), day(2024, 7, 1)),
            Some(4)
        );
        assert_eq!(
            count_calendar_quarters_in_interval(day(2024, 3, 31), day(2024, 3, 31)),
            Some(1)
        );
        assert_eq!(
            count_calendar_quarters_in_interval(day(2024, 4, 1), day(2024, 3, 31)),
            None
        );
    }

    #[test]
    fn handles_the_complete_chrono_date_range() {
        let total =
            count_calendar_quarters_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert!(total > 0);
        assert_eq!(
            count_calendar_quarters_in_interval(NaiveDate::MIN, NaiveDate::MIN),
            Some(1)
        );
    }
}
