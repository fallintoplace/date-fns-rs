//! Overlap and union sizes of inclusive calendar date intervals.

use chrono::NaiveDate;

/// Count common calendar days between two inclusive date intervals.
/// Returns None if either input interval is reversed; disjoint intervals return zero.
pub fn overlapping_days_in_intervals(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> Option<i64> {
    let intersection = crate::intersection_of_intervals(a_start, a_end, b_start, b_end)?;
    match intersection {
        Some((start, end)) => crate::interval_length_in_days(start, end),
        None => Some(0),
    }
}

/// Count calendar dates covered by the union of inclusive date intervals.
/// Handles unsorted, adjacent and overlapping ranges; empty input returns zero.
/// Returns None when any input interval is reversed.
pub fn count_distinct_days_covered(intervals: &[(NaiveDate, NaiveDate)]) -> Option<i64> {
    crate::merge_intervals(intervals)?
        .into_iter()
        .try_fold(0_i64, |total, (start, end)| {
            total.checked_add(crate::interval_length_in_days(start, end)?)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn counts_inclusive_overlap_and_union_without_duplicates() {
        assert_eq!(overlapping_days_in_intervals(date(1), date(3), date(3), date(5)), Some(1));
        assert_eq!(overlapping_days_in_intervals(date(1), date(2), date(3), date(4)), Some(0));
        assert_eq!(overlapping_days_in_intervals(date(2), date(1), date(1), date(2)), None);
        assert_eq!(count_distinct_days_covered(&[(date(3), date(5)), (date(1), date(3)), (date(8), date(8))]), Some(6));
        assert_eq!(count_distinct_days_covered(&[]), Some(0));
    }

    #[test]
    fn works_at_chrono_limits() {
        assert_eq!(count_distinct_days_covered(&[(NaiveDate::MAX, NaiveDate::MAX)]), Some(1));
        assert_eq!(overlapping_days_in_intervals(NaiveDate::MIN, NaiveDate::MAX, NaiveDate::MAX, NaiveDate::MAX), Some(1));
        assert_eq!(count_distinct_days_covered(&[(date(5), date(4))]), None);
    }
}