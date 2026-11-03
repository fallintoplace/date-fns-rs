//! Complete calendar-month differences between naive datetimes.

use chrono::NaiveDateTime;

/// Count complete month anniversaries from right to left.
///
/// Month ends clamp as in add_months_datetime. Clock time is significant.
/// Incomplete months truncate toward zero, including for reversed inputs.
pub fn difference_in_months_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    let (start, end, sign) = if left >= right {
        (right, left, 1_i64)
    } else {
        (left, right, -1_i64)
    };
    let months = crate::difference_in_months(end.date(), start.date());
    if months == 0 {
        return 0;
    }
    let reached = i32::try_from(months)
        .ok()
        .and_then(|months| crate::add_months_datetime(start, months))
        .is_some_and(|anniversary| anniversary <= end);
    (months - i64::from(!reached)) * sign
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(hour, 0, 0, 123_456_789)
            .unwrap()
    }

    #[test]
    fn counts_clock_time_at_clamped_month_ends() {
        let start = at(2024, 1, 31, 12);
        assert_eq!(difference_in_months_datetime(at(2024, 2, 29, 11), start), 0);
        assert_eq!(difference_in_months_datetime(at(2024, 2, 29, 12), start), 1);
        assert_eq!(difference_in_months_datetime(at(2024, 3, 31, 11), start), 1);
        assert_eq!(difference_in_months_datetime(at(2024, 3, 31, 12), start), 2);
        assert_eq!(difference_in_months_datetime(at(2023, 2, 28, 12), at(2023, 1, 31, 12)), 1);
    }

    #[test]
    fn is_symmetric_and_truncates_incomplete_months() {
        let start = at(2024, 2, 29, 12);
        let end = at(2024, 3, 28, 23);
        assert_eq!(difference_in_months_datetime(end, start), 0);
        assert_eq!(difference_in_months_datetime(start, end), 0);
        assert_eq!(difference_in_months_datetime(start, start), 0);
        assert_eq!(difference_in_months_datetime(start, at(2024, 1, 29, 12)), 1);
        assert_eq!(difference_in_months_datetime(at(2024, 1, 29, 12), start), -1);
    }

    #[test]
    fn supports_chrono_datetime_limits() {
        let span = difference_in_months_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(span > 0);
        assert_eq!(
            difference_in_months_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -span
        );
    }
}
