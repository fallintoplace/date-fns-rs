//! Complete calendar-year differences between naive datetimes.

use chrono::NaiveDateTime;

/// Count complete year anniversaries from right to left.
///
/// February 29 clamps to February 28 in non-leap years. Exact clock time
/// determines whether an anniversary is complete.
pub fn difference_in_years_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    let (start, end, sign) = if left >= right {
        (right, left, 1_i64)
    } else {
        (left, right, -1_i64)
    };
    let years = crate::difference_in_calendar_years(end.date(), start.date());
    if years == 0 {
        return 0;
    }
    let reached = i32::try_from(years)
        .ok()
        .and_then(|years| crate::add_years_datetime(start, years))
        .is_some_and(|anniversary| anniversary <= end);
    (years - i64::from(!reached)) * sign
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(hour, 0, 0, 999_999_999)
            .unwrap()
    }

    #[test]
    fn honors_leap_day_clamping_and_clock_time() {
        let start = at(2020, 2, 29, 12);
        assert_eq!(difference_in_years_datetime(at(2021, 2, 28, 11), start), 0);
        assert_eq!(difference_in_years_datetime(at(2021, 2, 28, 12), start), 1);
        assert_eq!(difference_in_years_datetime(at(2024, 2, 29, 12), start), 4);
        assert_eq!(difference_in_years_datetime(at(2025, 2, 27, 23), start), 4);
    }

    #[test]
    fn is_symmetric_and_distinguishes_calendar_boundaries() {
        let start = at(2024, 12, 31, 23);
        let end = at(2025, 1, 1, 0);
        assert_eq!(difference_in_years_datetime(end, start), 0);
        assert_eq!(difference_in_years_datetime(start, end), 0);
        assert_eq!(difference_in_years_datetime(start, start), 0);
        assert_eq!(difference_in_years_datetime(at(2026, 12, 31, 23), start), 2);
        assert_eq!(difference_in_years_datetime(start, at(2026, 12, 31, 23)), -2);
    }

    #[test]
    fn handles_chrono_datetime_limits() {
        let span = difference_in_years_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(span > 0);
        assert_eq!(
            difference_in_years_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -span
        );
    }
}
