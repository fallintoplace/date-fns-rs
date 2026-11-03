//! Constant-time counts of calendar periods touched by inclusive date intervals.

use chrono::NaiveDate;

/// Count one-based Gregorian centuries touched by an inclusive interval.
///
/// Both endpoints count. Reversed intervals return None.
/// The calculation does not construct any out-of-range period boundaries.
pub fn count_calendar_centuries_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(i64::from(crate::difference_in_calendar_centuries(end, start)) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_periods_across_boundaries() {
        assert_eq!(
            count_calendar_centuries_in_interval(day(2000, 12, 31), day(2001, 1, 1)),
            Some(2)
        );
        assert_eq!(
            count_calendar_centuries_in_interval(day(0, 12, 31), day(1, 1, 1)),
            Some(2)
        );
        assert_eq!(
            count_calendar_centuries_in_interval(day(2001, 1, 1), day(2100, 12, 31)),
            Some(1)
        );
    }

    #[test]
    fn validates_ranges_and_date_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(count_calendar_centuries_in_interval(last, first), None);
        let count = count_calendar_centuries_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert!(count > 1);
        assert_eq!(
            count_calendar_centuries_in_interval(NaiveDate::MAX, NaiveDate::MAX),
            Some(1)
        );
    }
}
