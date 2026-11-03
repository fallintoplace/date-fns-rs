//! Constant-time count of calendar weeks touched by inclusive intervals.

use chrono::{NaiveDate, Weekday};

/// Count distinct calendar weeks intersecting [start, end].
///
/// Both endpoints are included, and the first weekday is configurable.
/// Reversed intervals return None. Week boundaries outside Chrono's
/// supported date range do not affect the result.
pub fn count_calendar_weeks_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    week_starts_on: Weekday,
) -> Option<i64> {
    if start > end {
        return None;
    }
    Some(crate::difference_in_calendar_weeks(end, start, week_starts_on) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn includes_partial_weeks_under_explicit_week_starts() {
        let sunday = day(2024, 1, 7);
        let monday = day(2024, 1, 8);
        assert_eq!(
            count_calendar_weeks_in_interval(sunday, monday, Weekday::Mon),
            Some(2)
        );
        assert_eq!(
            count_calendar_weeks_in_interval(sunday, monday, Weekday::Sun),
            Some(1)
        );
        assert_eq!(
            count_calendar_weeks_in_interval(sunday, sunday, Weekday::Mon),
            Some(1)
        );
    }

    #[test]
    fn rejects_reversed_ranges_and_handles_chrono_limits() {
        assert_eq!(
            count_calendar_weeks_in_interval(
                day(2024, 2, 29),
                day(2024, 2, 28),
                Weekday::Mon
            ),
            None
        );
        let total =
            count_calendar_weeks_in_interval(NaiveDate::MIN, NaiveDate::MAX, Weekday::Mon)
                .unwrap();
        assert!(total > 0);
        assert_eq!(
            count_calendar_weeks_in_interval(
                NaiveDate::MAX,
                NaiveDate::MAX,
                Weekday::Sun
            ),
            Some(1)
        );
    }
}
