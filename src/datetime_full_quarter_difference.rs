//! Complete three-month differences between naive datetimes.

use chrono::NaiveDateTime;

/// Count complete three-month offsets from right to left.
///
/// A quarter means three elapsed calendar months, not a quarter boundary.
/// Target month days clamp like add_quarters_datetime.
pub fn difference_in_quarters_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    let (start, end, sign) = if left >= right {
        (right, left, 1_i64)
    } else {
        (left, right, -1_i64)
    };
    let quarters = crate::difference_in_calendar_months(end.date(), start.date()) / 3;
    if quarters == 0 {
        return 0;
    }
    let reached = i32::try_from(quarters)
        .ok()
        .and_then(|quarters| crate::add_quarters_datetime(start, quarters))
        .is_some_and(|anniversary| anniversary <= end);
    (quarters - i64::from(!reached)) * sign
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(hour, 15, 0, 123_456_789)
            .unwrap()
    }

    #[test]
    fn honors_clamped_quarter_anniversaries_and_clock_time() {
        let start = at(2024, 1, 31, 12);
        assert_eq!(difference_in_quarters_datetime(at(2024, 4, 30, 11), start), 0);
        assert_eq!(difference_in_quarters_datetime(at(2024, 4, 30, 12), start), 1);
        assert_eq!(difference_in_quarters_datetime(at(2024, 7, 31, 12), start), 2);
        assert_eq!(difference_in_quarters_datetime(at(2024, 4, 29, 23), start), 0);
    }

    #[test]
    fn reversals_are_signed_and_partial_periods_truncate() {
        let start = at(2023, 12, 31, 13);
        let end = at(2024, 3, 31, 13);
        assert_eq!(difference_in_quarters_datetime(end, start), 1);
        assert_eq!(difference_in_quarters_datetime(start, end), -1);
        assert_eq!(difference_in_quarters_datetime(end, end), 0);
        assert_eq!(difference_in_quarters_datetime(at(2024, 1, 1, 0), start), 0);
    }

    #[test]
    fn supports_chrono_datetime_limits() {
        let span = difference_in_quarters_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(span > 0);
        assert_eq!(
            difference_in_quarters_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -span
        );
    }
}
