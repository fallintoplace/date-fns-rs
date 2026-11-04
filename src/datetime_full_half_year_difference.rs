//! Complete six-month calendar differences between naive datetimes.

use chrono::NaiveDateTime;

/// Count complete six-month periods from right to left.
///
/// Anniversary dates clamp to the target month's last day. Fractional
/// half-years truncate toward zero, including for negative differences.
pub fn difference_in_half_years_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    let (start, end, sign) = if left >= right {
        (right, left, 1_i64)
    } else {
        (left, right, -1_i64)
    };
    let half_years = crate::difference_in_calendar_months(end.date(), start.date()) / 6;
    if half_years == 0 {
        return 0;
    }
    let reached = i32::try_from(half_years)
        .ok()
        .and_then(|periods| periods.checked_mul(6))
        .and_then(|months| crate::add_months_datetime(start, months))
        .is_some_and(|anniversary| anniversary <= end);
    (half_years - i64::from(!reached)) * sign
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(hour, 15, 16, 987_654_321)
            .unwrap()
    }

    #[test]
    fn honors_clamped_anniversaries_and_nanoseconds() {
        let start = at(2023, 8, 31, 12);
        assert_eq!(
            difference_in_half_years_datetime(at(2024, 2, 29, 11), start),
            0
        );
        assert_eq!(
            difference_in_half_years_datetime(at(2024, 2, 29, 12), start),
            1
        );
        assert_eq!(
            difference_in_half_years_datetime(at(2024, 8, 31, 12), start),
            2
        );
        assert_eq!(
            difference_in_half_years_datetime(start, at(2024, 2, 29, 12)),
            -1
        );
    }

    #[test]
    fn handles_zero_partial_offsets_and_date_limits() {
        let first = at(2024, 12, 31, 23);
        let next = at(2025, 1, 1, 0);
        assert_eq!(difference_in_half_years_datetime(next, first), 0);
        assert_eq!(difference_in_half_years_datetime(first, first), 0);
        let span = difference_in_half_years_datetime(
            NaiveDateTime::MAX,
            NaiveDateTime::MIN,
        );
        assert!(span > 0);
        assert_eq!(
            difference_in_half_years_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -span
        );
    }
}
