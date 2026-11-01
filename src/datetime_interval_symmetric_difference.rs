//! Symmetric difference of inclusive naive datetime intervals.

use chrono::NaiveDateTime;

/// Return timestamps that belong to exactly one of two inclusive intervals.
///
/// Ranges are ordered and retain nanosecond precision. Invalid input returns None.
pub fn symmetric_difference_of_datetime_intervals(
    a_start: NaiveDateTime,
    a_end: NaiveDateTime,
    b_start: NaiveDateTime,
    b_end: NaiveDateTime,
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    let mut result = crate::subtract_datetime_intervals(a_start, a_end, b_start, b_end)?;
    result.extend(crate::subtract_datetime_intervals(
        b_start, b_end, a_start, a_end,
    )?);
    result.sort_unstable_by_key(|(start, _)| *start);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    fn at(second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(12, 0, second)
            .unwrap()
    }

    #[test]
    fn splits_partial_overlap_at_nanosecond_boundaries() {
        let before = at(4).checked_sub_signed(TimeDelta::nanoseconds(1)).unwrap();
        let after = at(6).checked_add_signed(TimeDelta::nanoseconds(1)).unwrap();
        assert_eq!(
            symmetric_difference_of_datetime_intervals(at(1), at(6), at(4), at(9)),
            Some(vec![(at(1), before), (after, at(9))])
        );
    }

    #[test]
    fn handles_disjoint_contained_identical_and_invalid_ranges() {
        assert_eq!(
            symmetric_difference_of_datetime_intervals(at(3), at(4), at(1), at(2)),
            Some(vec![(at(1), at(2)), (at(3), at(4))])
        );
        assert_eq!(
            symmetric_difference_of_datetime_intervals(at(1), at(4), at(2), at(3)),
            Some(vec![
                (at(1), at(2).checked_sub_signed(TimeDelta::nanoseconds(1)).unwrap()),
                (at(3).checked_add_signed(TimeDelta::nanoseconds(1)).unwrap(), at(4)),
            ])
        );
        assert_eq!(
            symmetric_difference_of_datetime_intervals(at(1), at(4), at(1), at(4)),
            Some(vec![])
        );
        assert_eq!(
            symmetric_difference_of_datetime_intervals(at(4), at(1), at(1), at(3)),
            None
        );
    }

    #[test]
    fn handles_datetime_extremes() {
        assert_eq!(
            symmetric_difference_of_datetime_intervals(
                NaiveDateTime::MIN,
                NaiveDateTime::MIN,
                NaiveDateTime::MAX,
                NaiveDateTime::MAX,
            ),
            Some(vec![
                (NaiveDateTime::MIN, NaiveDateTime::MIN),
                (NaiveDateTime::MAX, NaiveDateTime::MAX),
            ])
        );
    }
}
