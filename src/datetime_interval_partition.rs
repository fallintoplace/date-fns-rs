//! Partition and gap operations on inclusive naive datetime intervals.

use chrono::{NaiveDateTime, TimeDelta};

/// Split an inclusive datetime interval immediately after cut.
///
/// The left part includes cut. The right part is None if cut is the end.
/// Reversed intervals or out-of-range cuts return None.
pub fn split_datetime_interval_after(
    start: NaiveDateTime,
    end: NaiveDateTime,
    cut: NaiveDateTime,
) -> Option<(
    (NaiveDateTime, NaiveDateTime),
    Option<(NaiveDateTime, NaiveDateTime)>,
)> {
    if start > end || cut < start || cut > end {
        return None;
    }
    let right = if cut == end {
        None
    } else {
        Some((cut.checked_add_signed(TimeDelta::nanoseconds(1))?, end))
    };
    Some(((start, cut), right))
}

/// Split an inclusive datetime interval immediately before cut.
///
/// The right part includes cut. The left part is None if cut is the start.
/// Reversed intervals or out-of-range cuts return None.
pub fn split_datetime_interval_before(
    start: NaiveDateTime,
    end: NaiveDateTime,
    cut: NaiveDateTime,
) -> Option<(
    Option<(NaiveDateTime, NaiveDateTime)>,
    (NaiveDateTime, NaiveDateTime),
)> {
    if start > end || cut < start || cut > end {
        return None;
    }
    let left = if cut == start {
        None
    } else {
        Some((start, cut.checked_sub_signed(TimeDelta::nanoseconds(1))?))
    };
    Some((left, (cut, end)))
}

/// Find timestamps missing strictly between two inclusive datetime intervals.
///
/// Returns None for an invalid interval and Some(None) for overlapping or
/// adjacent intervals. The gap is independent of the input order.
pub fn gap_between_datetime_intervals(
    a_start: NaiveDateTime,
    a_end: NaiveDateTime,
    b_start: NaiveDateTime,
    b_end: NaiveDateTime,
) -> Option<Option<(NaiveDateTime, NaiveDateTime)>> {
    if a_start > a_end || b_start > b_end {
        return None;
    }
    let (left_end, right_start) = if a_start <= b_start {
        (a_end, b_start)
    } else {
        (b_end, a_start)
    };
    Some(
        match (
            left_end.checked_add_signed(TimeDelta::nanoseconds(1)),
            right_start.checked_sub_signed(TimeDelta::nanoseconds(1)),
        ) {
            (Some(start), Some(end)) if start <= end => Some((start, end)),
            _ => None,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn stamp(year: i32, month: u32, day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    #[test]
    fn splits_exactly_at_nanosecond_boundaries_across_midnight() {
        let midnight = stamp(2024, 3, 1);
        let cut = midnight
            .checked_sub_signed(TimeDelta::nanoseconds(1))
            .unwrap();
        let end = midnight
            .checked_add_signed(TimeDelta::nanoseconds(1))
            .unwrap();
        assert_eq!(
            split_datetime_interval_after(cut, end, cut),
            Some(((cut, cut), Some((midnight, end))))
        );
        assert_eq!(
            split_datetime_interval_before(cut, end, midnight),
            Some((Some((cut, cut)), (midnight, end)))
        );
        assert_eq!(
            split_datetime_interval_before(cut, end, cut),
            Some((None, (cut, end)))
        );
    }

    #[test]
    fn rejects_invalid_cuts_and_handles_datetime_limits() {
        let start = stamp(2024, 1, 1);
        let end = stamp(2024, 1, 2);
        assert_eq!(split_datetime_interval_after(end, start, start), None);
        assert_eq!(
            split_datetime_interval_before(start, end, end + TimeDelta::seconds(1)),
            None
        );
        assert_eq!(
            split_datetime_interval_after(
                NaiveDateTime::MAX,
                NaiveDateTime::MAX,
                NaiveDateTime::MAX
            ),
            Some(((NaiveDateTime::MAX, NaiveDateTime::MAX), None))
        );
        assert_eq!(
            split_datetime_interval_before(
                NaiveDateTime::MIN,
                NaiveDateTime::MIN,
                NaiveDateTime::MIN
            ),
            Some((None, (NaiveDateTime::MIN, NaiveDateTime::MIN)))
        );
    }

    #[test]
    fn finds_gaps_of_exactly_two_nanoseconds_and_skips_adjacent_ranges() {
        let a = stamp(2024, 1, 1);
        let b = a.checked_add_signed(TimeDelta::nanoseconds(1)).unwrap();
        let c = a.checked_add_signed(TimeDelta::nanoseconds(2)).unwrap();
        let d = a.checked_add_signed(TimeDelta::nanoseconds(3)).unwrap();
        assert_eq!(
            gap_between_datetime_intervals(a, a, d, d),
            Some(Some((b, c)))
        );
        assert_eq!(
            gap_between_datetime_intervals(d, d, a, a),
            Some(Some((b, c)))
        );
        assert_eq!(gap_between_datetime_intervals(a, b, c, d), Some(None));
        assert_eq!(gap_between_datetime_intervals(a, c, b, d), Some(None));
        assert_eq!(gap_between_datetime_intervals(c, a, b, d), None);
    }
}
