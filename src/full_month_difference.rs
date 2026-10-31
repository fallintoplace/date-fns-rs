//! Signed complete-month differences using clamped month anniversaries.

use chrono::{Datelike, NaiveDate};

use crate::difference::difference_in_calendar_months;
use crate::metadata::get_days_in_month;

/// Count complete calendar-month offsets between two dates.
///
/// A month is complete when advancing the earlier date by that many
/// calendar months (clamping at month end) does not pass the later date.
/// Reversing the arguments negates the result.
pub fn difference_in_months(left: NaiveDate, right: NaiveDate) -> i64 {
    let (start, end, sign) = if left >= right {
        (right, left, 1_i64)
    } else {
        (left, right, -1_i64)
    };

    let months = difference_in_calendar_months(end, start);
    if months == 0 {
        return 0;
    }

    // A clamped anniversary is reached if the target month's last day
    // comes before the original day of month.
    let complete = end.day() >= start.day() || end.day() == get_days_in_month(end);
    (months - i64::from(!complete)) * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_partial_and_complete_months() {
        assert_eq!(
            difference_in_months(date(2024, 3, 28), date(2024, 2, 29)),
            0
        );
        assert_eq!(
            difference_in_months(date(2024, 3, 29), date(2024, 2, 29)),
            1
        );
        assert_eq!(
            difference_in_months(date(2024, 2, 28), date(2024, 1, 31)),
            0
        );
        assert_eq!(
            difference_in_months(date(2024, 2, 29), date(2024, 1, 31)),
            1
        );
        assert_eq!(
            difference_in_months(date(2023, 2, 28), date(2023, 1, 31)),
            1
        );
    }

    #[test]
    fn signed_results_are_symmetric_for_reversed_arguments() {
        let first = date(2023, 12, 31);
        let last = date(2024, 3, 30);
        let forward = difference_in_months(last, first);
        assert_eq!(forward, 2);
        assert_eq!(difference_in_months(first, last), -forward);
        assert_eq!(difference_in_months(first, first), 0);
    }

    #[test]
    fn handles_supported_date_limits() {
        let forward = difference_in_months(NaiveDate::MAX, NaiveDate::MIN);
        assert!(forward > 0);
        assert_eq!(
            difference_in_months(NaiveDate::MIN, NaiveDate::MAX),
            -forward
        );
    }
}
