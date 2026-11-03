//! Constant-time inclusive period counts for date intervals.

use chrono::NaiveDate;

/// Count distinct standard Gregorian decades touched by an inclusive interval.
///
/// Endpoints are included. Reversed intervals return None.
/// No period boundaries are constructed.
pub fn count_calendar_decades_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_decades(end, start) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_inclusive_periods_across_boundaries() {
        assert_eq!(
            count_calendar_decades_in_interval(day(2019, 12, 31), day(2020, 1, 1)),
            Some(2)
        );
        assert_eq!(count_calendar_decades_in_interval(day(-1, 12, 31), day(0, 1, 1)), Some(2));
        assert_eq!(
            count_calendar_decades_in_interval(day(2020, 3, 1), day(2029, 12, 31)),
            Some(1)
        );
    }

    #[test]
    fn validates_input_and_handles_date_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(count_calendar_decades_in_interval(last, first), None);
        let count = count_calendar_decades_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert!(count > 1);
        assert_eq!(count_calendar_decades_in_interval(NaiveDate::MAX, NaiveDate::MAX), Some(1));
    }
}
