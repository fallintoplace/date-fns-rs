//! Difference of inclusive naive datetime intervals at nanosecond resolution.

use chrono::{NaiveDateTime, TimeDelta};

/// Subtract one inclusive datetime interval from another.
///
/// Returns None for reversed input and otherwise returns zero, one or two
/// ordered intervals. Shared endpoints are removed at nanosecond precision.
pub fn subtract_datetime_intervals(
    start: NaiveDateTime,
    end: NaiveDateTime,
    exclude_start: NaiveDateTime,
    exclude_end: NaiveDateTime,
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if start > end || exclude_start > exclude_end {
        return None;
    }
    if exclude_end < start || exclude_start > end {
        return Some(vec![(start, end)]);
    }

    let mut result = Vec::with_capacity(2);
    if exclude_start > start {
        let before = exclude_start.checked_sub_signed(TimeDelta::nanoseconds(1))?;
        result.push((start, before));
    }
    if exclude_end < end {
        let after = exclude_end.checked_add_signed(TimeDelta::nanoseconds(1))?;
        result.push((after, end));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn stamp(second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(23, 59, second)
            .unwrap()
    }

    #[test]
    fn splits_around_exact_nanosecond_exclusions() {
        let before = stamp(1)
            .checked_sub_signed(TimeDelta::nanoseconds(1))
            .unwrap();
        let after = stamp(1)
            .checked_add_signed(TimeDelta::nanoseconds(1))
            .unwrap();
        assert_eq!(
            subtract_datetime_intervals(stamp(0), stamp(2), stamp(1), stamp(1)),
            Some(vec![(stamp(0), before), (after, stamp(2))])
        );
    }

    #[test]
    fn checks_disjoint_partial_and_invalid_ranges() {
        assert_eq!(
            subtract_datetime_intervals(stamp(0), stamp(1), stamp(2), stamp(3)),
            Some(vec![(stamp(0), stamp(1))])
        );
        assert_eq!(
            subtract_datetime_intervals(stamp(0), stamp(2), stamp(0), stamp(1)),
            Some(vec![(
                stamp(1).checked_add_signed(TimeDelta::nanoseconds(1)).unwrap(),
                stamp(2)
            )])
        );
        assert_eq!(
            subtract_datetime_intervals(stamp(0), stamp(2), stamp(0), stamp(2)),
            Some(vec![])
        );
        assert_eq!(
            subtract_datetime_intervals(stamp(2), stamp(0), stamp(0), stamp(1)),
            None
        );
        assert_eq!(
            subtract_datetime_intervals(stamp(0), stamp(2), stamp(2), stamp(1)),
            None
        );
    }

    #[test]
    fn handles_extreme_datetime_endpoints() {
        assert_eq!(
            subtract_datetime_intervals(
                NaiveDateTime::MIN,
                NaiveDateTime::MAX,
                NaiveDateTime::MAX,
                NaiveDateTime::MAX
            ),
            Some(vec![(
                NaiveDateTime::MIN,
                NaiveDateTime::MAX
                    .checked_sub_signed(TimeDelta::nanoseconds(1))
                    .unwrap()
            )])
        );
    }
}
