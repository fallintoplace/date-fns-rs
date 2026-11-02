//! Intersect multiple inclusive calendar-date interval sets.

use chrono::NaiveDate;

/// Intersect the union of each supplied set of inclusive date ranges.
///
/// Overlapping or unsorted intervals within each input are normalized first.
/// Empty input or any empty set produces an empty result. Returns None if any
/// interval is reversed. Complexity is O(sum(n log n) + k * sum(n)), where k
/// is the number of calendars, regardless of the number of days covered.
pub fn intersect_many_date_interval_sets(
    sets: &[&[(NaiveDate, NaiveDate)]],
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if sets.is_empty() {
        return Some(Vec::new());
    }
    let normalized: Vec<_> = sets
        .iter()
        .map(|ranges| crate::merge_intervals(ranges))
        .collect::<Option<_>>()?;
    if normalized.iter().any(Vec::is_empty) {
        return Some(Vec::new());
    }

    let mut positions = vec![0_usize; normalized.len()];
    let mut result: Vec<(NaiveDate, NaiveDate)> = Vec::new();

    loop {
        let mut latest_start = NaiveDate::MIN;
        let mut earliest_end = NaiveDate::MAX;
        for (ranges, &index) in normalized.iter().zip(positions.iter()) {
            let Some(&(start, end)) = ranges.get(index) else {
                return Some(result);
            };
            latest_start = latest_start.max(start);
            earliest_end = earliest_end.min(end);
        }

        if latest_start <= earliest_end {
            if let Some(last) = result.last_mut() {
                if latest_start <= last.1 || last.1.succ_opt() == Some(latest_start) {
                    last.1 = last.1.max(earliest_end);
                } else {
                    result.push((latest_start, earliest_end));
                }
            } else {
                result.push((latest_start, earliest_end));
            }
        }
        for (ranges, index) in normalized.iter().zip(positions.iter_mut()) {
            if ranges[*index].1 == earliest_end {
                *index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn intersects_three_overlapping_unsorted_calendars() {
        let first = [(day(9), day(12)), (day(1), day(6))];
        let second = [(day(2), day(4)), (day(5), day(10))];
        let third = [(day(3), day(5)), (day(9), day(11))];
        assert_eq!(
            intersect_many_date_interval_sets(&[&first, &second, &third]),
            Some(vec![(day(3), day(5)), (day(9), day(10))])
        );
    }

    #[test]
    fn validates_all_inputs_and_handles_empty_calendars() {
        let a = [(day(1), day(3))];
        let bad = [(day(5), day(4))];
        assert_eq!(intersect_many_date_interval_sets(&[]), Some(vec![]));
        assert_eq!(intersect_many_date_interval_sets(&[&a, &[]]), Some(vec![]));
        assert_eq!(intersect_many_date_interval_sets(&[&a, &bad]), None);
        assert_eq!(
            intersect_many_date_interval_sets(&[&a]),
            Some(a.to_vec())
        );
    }

    #[test]
    fn retains_extreme_endpoints_and_excludes_missing_days() {
        let first = [(NaiveDate::MIN, NaiveDate::MAX)];
        let second = [
            (NaiveDate::MAX, NaiveDate::MAX),
            (NaiveDate::MIN, NaiveDate::MIN),
        ];
        assert_eq!(
            intersect_many_date_interval_sets(&[&first, &second]),
            Some(vec![
                (NaiveDate::MIN, NaiveDate::MIN),
                (NaiveDate::MAX, NaiveDate::MAX),
            ])
        );
    }

    #[test]
    fn agrees_with_boolean_intersection_for_all_small_sets() {
        for mask_a in 0_u32..16 {
            for mask_b in 0_u32..16 {
                let a: Vec<_> = (0..4)
                    .filter(|bit| mask_a & (1 << bit) != 0)
                    .map(|bit| (day(bit + 1), day(bit + 1)))
                    .collect();
                let b: Vec<_> = (0..4)
                    .filter(|bit| mask_b & (1 << bit) != 0)
                    .map(|bit| (day(bit + 1), day(bit + 1)))
                    .collect();
                let common = intersect_many_date_interval_sets(&[&a, &b]).unwrap();
                for bit in 0..4 {
                    let covered = common
                        .iter()
                        .any(|(lo, hi)| *lo <= day(bit + 1) && day(bit + 1) <= *hi);
                    assert_eq!(
                        covered,
                        mask_a & (1 << bit) != 0 && mask_b & (1 << bit) != 0
                    );
                }
            }
        }
    }
}
