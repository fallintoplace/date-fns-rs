//! Intersection of inclusive nanosecond-precision datetime interval sets.

use chrono::NaiveDateTime;

/// Intersect two unions of inclusive naive datetime intervals.
///
/// Inputs are normalized independently. Reversed intervals return None,
/// including when the other side is empty. Output is sorted and disjoint.
/// The scan is linear after O(n log n) input normalization, regardless of
/// the width of the datetime ranges.
pub fn intersect_datetime_interval_sets(
    left: &[(NaiveDateTime, NaiveDateTime)],
    right: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    let left = crate::merge_datetime_intervals(left)?;
    let right = crate::merge_datetime_intervals(right)?;
    let mut result = Vec::with_capacity(left.len().min(right.len()));
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        let (a_start, a_end) = left[i];
        let (b_start, b_end) = right[j];
        let start = a_start.max(b_start);
        let end = a_end.min(b_end);
        if start <= end {
            result.push((start, end));
        }

        if a_end <= b_end {
            i += 1;
        }
        if b_end <= a_end {
            j += 1;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(nanos: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, nanos)
            .unwrap()
    }

    #[test]
    fn preserves_exact_nanosecond_endpoints() {
        let left = [(at(3), at(6)), (at(0), at(2)), (at(2), at(4))];
        let right = [(at(6), at(6)), (at(2), at(2)), (at(8), at(9))];
        assert_eq!(
            intersect_datetime_interval_sets(&left, &right),
            Some(vec![(at(2), at(2)), (at(6), at(6))])
        );
        assert_eq!(
            intersect_datetime_interval_sets(&right, &left),
            intersect_datetime_interval_sets(&left, &right)
        );
    }

    #[test]
    fn rejects_invalid_ranges_even_when_other_input_is_empty() {
        assert_eq!(intersect_datetime_interval_sets(&[], &[]), Some(vec![]));
        assert_eq!(
            intersect_datetime_interval_sets(&[(at(4), at(2))], &[]),
            None
        );
        assert_eq!(
            intersect_datetime_interval_sets(&[], &[(at(4), at(2))]),
            None
        );
        assert_eq!(
            intersect_datetime_interval_sets(&[(at(0), at(1))], &[(at(3), at(4))]),
            Some(vec![])
        );
    }

    #[test]
    fn supports_minimum_and_maximum_datetime_values() {
        let full = [(NaiveDateTime::MIN, NaiveDateTime::MAX)];
        let edges = [
            (NaiveDateTime::MAX, NaiveDateTime::MAX),
            (NaiveDateTime::MIN, NaiveDateTime::MIN),
        ];
        assert_eq!(
            intersect_datetime_interval_sets(&full, &edges),
            Some(vec![edges[1], edges[0]])
        );
    }

    #[test]
    fn agrees_with_exhaustive_nanosecond_membership() {
        let candidates = [
            (at(0), at(2)),
            (at(2), at(5)),
            (at(4), at(6)),
            (at(7), at(9)),
        ];
        for first_mask in 0..16_u32 {
            for second_mask in 0..16_u32 {
                let first: Vec<_> = candidates
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| first_mask & (1 << i) != 0)
                    .map(|(_, interval)| *interval)
                    .collect();
                let second: Vec<_> = candidates
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| second_mask & (1 << i) != 0)
                    .map(|(_, interval)| *interval)
                    .collect();
                let result = intersect_datetime_interval_sets(&first, &second).unwrap();
                for tick in 0..=9 {
                    let value = at(tick);
                    let expected = first.iter().any(|(a, b)| *a <= value && value <= *b)
                        && second.iter().any(|(a, b)| *a <= value && value <= *b);
                    let actual = result.iter().any(|(a, b)| *a <= value && value <= *b);
                    assert_eq!(
                        actual, expected,
                        "{first_mask}, {second_mask}, {tick}"
                    );
                }
            }
        }
    }
}
