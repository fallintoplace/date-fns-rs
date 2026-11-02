//! Duration-based overlapping-day counts for naive datetime intervals.

use chrono::{NaiveDateTime, TimeDelta};

/// Count overlapping 24-hour days, rounding any positive remainder upward.
///
/// Intervals are half-open [start, end). Touching endpoints count as zero.
/// Returns None for reversed intervals. No daylight-saving time is assumed.
pub fn overlapping_elapsed_days_datetime_intervals(
    a_start: NaiveDateTime,
    a_end: NaiveDateTime,
    b_start: NaiveDateTime,
    b_end: NaiveDateTime,
) -> Option<i64> {
    if a_start > a_end || b_start > b_end {
        return None;
    }
    let start = a_start.max(b_start);
    let end = a_end.min(b_end);
    if start >= end {
        return Some(0);
    }
    let elapsed = end.signed_duration_since(start);
    let whole = elapsed.num_days();
    Some(whole + i64::from(elapsed > TimeDelta::days(whole)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, min, 0)
            .unwrap()
    }

    #[test]
    fn rounds_partial_days_up_but_not_exact_days() {
        let a = dt(2024, 2, 28, 0, 0);
        let b = dt(2024, 3, 1, 0, 0);
        let c = dt(2024, 3, 1, 0, 1);
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(a, c, a, b),
            Some(2)
        );
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(a, c, a, c),
            Some(3)
        );
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(
                dt(2024, 2, 29, 23, 59),
                c,
                dt(2024, 2, 29, 23, 58),
                c
            ),
            Some(1)
        );
    }

    #[test]
    fn handles_touching_disjoint_and_invalid_ranges() {
        let a = dt(2024, 1, 1, 0, 0);
        let b = dt(2024, 1, 2, 0, 0);
        let c = dt(2024, 1, 3, 0, 0);
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(a, b, b, c),
            Some(0)
        );
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(a, a, a, b),
            Some(0)
        );
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(b, a, a, b),
            None
        );
        assert_eq!(
            overlapping_elapsed_days_datetime_intervals(a, b, c, b),
            None
        );
    }

    #[test]
    fn supports_the_full_chrono_range() {
        let count = overlapping_elapsed_days_datetime_intervals(
            NaiveDateTime::MIN,
            NaiveDateTime::MAX,
            NaiveDateTime::MIN,
            NaiveDateTime::MAX,
        )
        .unwrap();
        assert!(count > 0);
    }
}
