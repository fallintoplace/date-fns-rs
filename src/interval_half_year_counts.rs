//! Constant-time count of calendar half-years touched by inclusive date intervals.

use chrono::NaiveDate;

/// Count January-June and July-December half-years touched by an inclusive interval.
///
/// Both endpoints count. Reversed intervals return None. The calculation
/// works across year zero without constructing out-of-range period starts.
pub fn count_calendar_half_years_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_half_years(end, start) + 1)
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
            count_calendar_half_years_in_interval(day(2024, 6, 30), day(2025, 1, 1)),
            Some(3)
        );
        assert_eq!(
            count_calendar_half_years_in_interval(day(2024, 7, 1), day(2024, 12, 31)),
            Some(1)
        );
        assert_eq!(
            count_calendar_half_years_in_interval(day(-1, 12, 31), day(0, 1, 1)),
            Some(2)
        );
    }

    #[test]
    fn rejects_reversed_intervals_and_handles_date_limits() {
        let first = day(2024, 1, 1);
        let last = day(2024, 12, 31);
        assert_eq!(count_calendar_half_years_in_interval(last, first), None);
        assert!(count_calendar_half_years_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap() > 1);
        assert_eq!(
            count_calendar_half_years_in_interval(NaiveDate::MAX, NaiveDate::MAX),
            Some(1)
        );
    }
}
