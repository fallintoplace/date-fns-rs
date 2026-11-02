//! Intersection of two unions of inclusive calendar-date intervals.

use chrono::NaiveDate;

/// Intersect two arbitrary collections of inclusive date ranges.
///
/// Inputs may be unsorted, overlapping, or adjacent. Each side is first
/// normalized and the resulting intersections are returned sorted and
/// disjoint. Empty inputs produce an empty result. A reversed interval
/// anywhere in either collection returns None.
///
/// After normalization the intersection scan is linear in the number of
/// disjoint intervals, not in the number of calendar days covered.
pub fn intersect_interval_sets(
    left: &[(NaiveDate, NaiveDate)],
    right: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    let left = crate::merge_intervals(left)?;
    let right = crate::merge_intervals(right)?;
    let mut intersections = Vec::with_capacity(left.len().min(right.len()));
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        let (a_start, a_end) = left[i];
        let (b_start, b_end) = right[j];
        let start = a_start.max(b_start);
        let end = a_end.min(b_end);
        if start <= end {
            intersections.push((start, end));
        }

        if a_end <= b_end {
            i += 1;
        }
        if b_end <= a_end {
            j += 1;
        }
    }
    Some(intersections)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn intersects_unsorted_overlapping_and_nested_inputs() {
        let left = [
            (date(10), date(15)),
            (date(1), date(5)),
            (date(4), date(11)),
            (date(20), date(21)),
        ];
        let right = [
            (date(12), date(12)),
            (date(3), date(7)),
            (date(21), date(24)),
        ];
        assert_eq!(
            intersect_interval_sets(&left, &right),
            Some(vec![
                (date(3), date(7)),
                (date(12), date(12)),
                (date(21), date(21)),
            ])
        );
        assert_eq!(
            intersect_interval_sets(&right, &left),
            intersect_interval_sets(&left, &right)
        );
    }

    #[test]
    fn handles_empty_disjoint_and_invalid_inputs() {
        assert_eq!(intersect_interval_sets(&[], &[]), Some(vec![]));
        assert_eq!(
            intersect_interval_sets(&[(date(1), date(3))], &[]),
            Some(vec![])
        );
        assert_eq!(
            intersect_interval_sets(&[(date(1), date(3))], &[(date(5), date(7))]),
            Some(vec![])
        );
        assert_eq!(
            intersect_interval_sets(&[(date(3), date(1))], &[]),
            None
        );
        assert_eq!(
            intersect_interval_sets(&[], &[(date(3), date(1))]),
            None
        );
    }

    #[test]
    fn handles_full_chrono_bounds_without_date_arithmetic() {
        let full = [(NaiveDate::MIN, NaiveDate::MAX)];
        let edges = [
            (NaiveDate::MAX, NaiveDate::MAX),
            (NaiveDate::MIN, NaiveDate::MIN),
        ];
        assert_eq!(
            intersect_interval_sets(&full, &edges),
            Some(vec![edges[1], edges[0]])
        );
    }

    #[test]
    fn agrees_with_exhaustive_day_membership_for_small_interval_sets() {
        let start = date(1);
        let end = date(8);
        let candidates = [
            (date(1), date(2)),
            (date(2), date(5)),
            (date(4), date(6)),
            (date(7), date(8)),
        ];
        for mask_a in 0..16_u32 {
            for mask_b in 0..16_u32 {
                let left: Vec<_> = candidates
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask_a & (1 << i) != 0)
                    .map(|(_, pair)| *pair)
                    .collect();
                let right: Vec<_> = candidates
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask_b & (1 << i) != 0)
                    .map(|(_, pair)| *pair)
                    .collect();
                let result = intersect_interval_sets(&left, &right).unwrap();
                for day in crate::each_day_of_interval(start, end).unwrap() {
                    let expected = left.iter().any(|(a, b)| *a <= day && day <= *b)
                        && right.iter().any(|(a, b)| *a <= day && day <= *b);
                    let actual = result.iter().any(|(a, b)| *a <= day && day <= *b);
                    assert_eq!(actual, expected, "{mask_a}, {mask_b}, {day}");
                }
            }
        }
    }
}
