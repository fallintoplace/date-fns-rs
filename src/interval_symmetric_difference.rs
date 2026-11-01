//! Symmetric difference of two inclusive calendar date intervals.

use chrono::NaiveDate;

/// Return dates belonging to exactly one of two inclusive intervals.
///
/// The result contains zero, one or two nonoverlapping intervals, ordered by
/// their starting dates. Returns None if either input interval is reversed.
pub fn symmetric_difference_of_intervals(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    let mut result = crate::subtract_intervals(a_start, a_end, b_start, b_end)?;
    result.extend(crate::subtract_intervals(b_start, b_end, a_start, a_end)?);
    result.sort_unstable_by_key(|(start, _)| *start);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn handles_partial_overlap_and_containment() {
        assert_eq!(
            symmetric_difference_of_intervals(date(1), date(6), date(4), date(9)),
            Some(vec![(date(1), date(3)), (date(7), date(9))])
        );
        assert_eq!(
            symmetric_difference_of_intervals(date(1), date(6), date(3), date(4)),
            Some(vec![(date(1), date(2)), (date(5), date(6))])
        );
        assert_eq!(
            symmetric_difference_of_intervals(date(1), date(3), date(1), date(3)),
            Some(vec![])
        );
    }

    #[test]
    fn handles_disjoint_touching_and_invalid_intervals() {
        assert_eq!(
            symmetric_difference_of_intervals(date(5), date(6), date(1), date(2)),
            Some(vec![(date(1), date(2)), (date(5), date(6))])
        );
        assert_eq!(
            symmetric_difference_of_intervals(date(1), date(3), date(3), date(5)),
            Some(vec![(date(1), date(2)), (date(4), date(5))])
        );
        assert_eq!(
            symmetric_difference_of_intervals(date(2), date(1), date(1), date(5)),
            None
        );
        assert_eq!(
            symmetric_difference_of_intervals(
                NaiveDate::MAX,
                NaiveDate::MAX,
                NaiveDate::MAX,
                NaiveDate::MAX
            ),
            Some(vec![])
        );
    }
}
