//! Count distinct nanosecond ticks in inclusive datetime intervals.

use chrono::NaiveDateTime;

/// Count nanoseconds covered by the union of inclusive datetime intervals.
///
/// Ranges may overlap, touch, or be unsorted. The count uses i128 so even
/// Chrono's full supported datetime range fits. Invalid ranges return None.
pub fn count_distinct_nanoseconds_covered(
    intervals: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<i128> {
    let mut total = 0_i128;
    for (start, end) in crate::merge_datetime_intervals(intervals)? {
        let from = start.and_utc();
        let to = end.and_utc();
        let seconds = i128::from(to.timestamp()) - i128::from(from.timestamp());
        let subsecond =
            i128::from(to.timestamp_subsec_nanos()) - i128::from(from.timestamp_subsec_nanos());
        let inclusive = seconds * 1_000_000_000 + subsecond + 1;
        total = total.checked_add(inclusive)?;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(nanosecond: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, nanosecond)
            .unwrap()
    }

    #[test]
    fn counts_union_without_double_counting_overlap() {
        assert_eq!(
            count_distinct_nanoseconds_covered(&[(at(2), at(4)), (at(0), at(2)), (at(4), at(4)),]),
            Some(5)
        );
        assert_eq!(
            count_distinct_nanoseconds_covered(&[(at(1), at(1)), (at(3), at(3))]),
            Some(2)
        );
        assert_eq!(count_distinct_nanoseconds_covered(&[]), Some(0));
    }

    #[test]
    fn validates_reversed_ranges_and_supports_large_results() {
        assert_eq!(count_distinct_nanoseconds_covered(&[(at(2), at(1))]), None);
        assert_eq!(
            count_distinct_nanoseconds_covered(&[(NaiveDateTime::MIN, NaiveDateTime::MAX)])
                .is_some_and(|count| count > i128::from(i64::MAX)),
            true
        );
    }
}
