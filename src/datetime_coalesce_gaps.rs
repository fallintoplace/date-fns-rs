//! Coalesce inclusive timestamp intervals across bounded uncovered gaps.

use chrono::NaiveDateTime;

fn epoch_nanos(value: NaiveDateTime) -> Option<i128> {
    let utc = value.and_utc();
    let subsecond = utc.timestamp_subsec_nanos();
    if subsecond >= 1_000_000_000 {
        return None;
    }
    Some(i128::from(utc.timestamp()) * 1_000_000_000 + i128::from(subsecond))
}

/// Merge timestamp ranges when at most max_gap_nanos uncovered ticks separate them.
///
/// Operates on inclusive nanosecond timestamps. Invalid input, negative
/// thresholds and unsupported leap-second encodings return None.
pub fn coalesce_datetime_intervals_with_gap(
    intervals: &[(NaiveDateTime, NaiveDateTime)],
    max_gap_nanos: i128,
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if max_gap_nanos < 0 {
        return None;
    }
    for &(first, last) in intervals {
        if first > last {
            return None;
        }
        epoch_nanos(first)?;
        epoch_nanos(last)?;
    }
    let normalized = crate::merge_datetime_intervals(intervals)?;
    let mut result: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::with_capacity(normalized.len());
    for (first, last) in normalized {
        if let Some(previous) = result.last_mut() {
            let gap = epoch_nanos(first)? - epoch_nanos(previous.1)? - 1;
            if gap <= max_gap_nanos {
                previous.1 = last;
                continue;
            }
        }
        result.push((first, last));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    fn tick(n: i64) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::nanoseconds(n))
            .unwrap()
    }

    #[test]
    fn distinguishes_one_and_two_tick_gaps() {
        let intervals = [(tick(0), tick(2)), (tick(5), tick(6)), (tick(8), tick(9))];
        assert_eq!(
            coalesce_datetime_intervals_with_gap(&intervals, 1),
            Some(vec![(tick(0), tick(2)), (tick(5), tick(9))])
        );
        assert_eq!(
            coalesce_datetime_intervals_with_gap(&intervals, 2),
            Some(vec![(tick(0), tick(9))])
        );
        assert_eq!(
            coalesce_datetime_intervals_with_gap(&intervals, 0),
            crate::merge_datetime_intervals(&intervals)
        );
    }

    #[test]
    fn rejects_reversed_ranges_and_negative_thresholds() {
        assert_eq!(coalesce_datetime_intervals_with_gap(&[], -1), None);
        assert_eq!(coalesce_datetime_intervals_with_gap(&[(tick(3), tick(1))], 2), None);
        assert_eq!(coalesce_datetime_intervals_with_gap(&[], 0), Some(vec![]));
        assert_eq!(
            coalesce_datetime_intervals_with_gap(
                &[
                    (NaiveDateTime::MIN, NaiveDateTime::MIN),
                    (NaiveDateTime::MAX, NaiveDateTime::MAX)
                ],
                i128::MAX
            ),
            Some(vec![(NaiveDateTime::MIN, NaiveDateTime::MAX)])
        );
    }
}
