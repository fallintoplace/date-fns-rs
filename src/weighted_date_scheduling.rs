//! Maximum-weight scheduling for inclusive date intervals.

use chrono::NaiveDate;

/// Select disjoint date ranges maximizing the sum of their signed weights.
///
/// Returns original indices in chronological order. Equal endpoints overlap.
/// Negative-weight intervals can be skipped. When weights tie, the algorithm
/// keeps the earlier solution in end-date order. Invalid ranges return None.
/// Complexity is O(n log n) time and O(n) memory.
pub fn select_max_weight_date_intervals(
    intervals: &[(NaiveDate, NaiveDate, i64)],
) -> Option<Vec<usize>> {
    if intervals.iter().any(|(start, end, _)| start > end) {
        return None;
    }
    let mut order: Vec<_> = (0..intervals.len()).collect();
    order.sort_unstable_by_key(|&i| (intervals[i].1, intervals[i].0, i));
    let ends: Vec<_> = order.iter().map(|&i| intervals[i].1).collect();
    let mut best = vec![0_i128; order.len() + 1];
    let mut chosen = vec![false; order.len()];
    let mut predecessor = vec![0_usize; order.len()];

    for (i, &index) in order.iter().enumerate() {
        let (start, _, weight) = intervals[index];
        let preceding = ends[..i].partition_point(|&end| end < start);
        let with = best[preceding].checked_add(i128::from(weight))?;
        let without = best[i];
        predecessor[i] = preceding;
        chosen[i] = with > without;
        best[i + 1] = with.max(without);
    }

    let mut output = Vec::new();
    let mut cursor = order.len();
    while cursor > 0 {
        let i = cursor - 1;
        if chosen[i] {
            output.push(order[i]);
            cursor = predecessor[i];
        } else {
            cursor = i;
        }
    }
    output.reverse();
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn prefers_two_profitable_jobs_over_a_longer_one() {
        let ranges = [
            (date(1), date(10), 9),
            (date(1), date(3), 6),
            (date(4), date(10), 7),
            (date(11), date(11), -10),
        ];
        assert_eq!(
            select_max_weight_date_intervals(&ranges),
            Some(vec![1, 2])
        );
    }

    #[test]
    fn checks_all_small_schedules_against_brute_force() {
        let candidates = [
            (date(1), date(3), 7),
            (date(2), date(2), 4),
            (date(3), date(5), 5),
            (date(6), date(7), 2),
            (date(8), date(8), -3),
        ];
        for input in 0..32_u32 {
            let ranges: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| input & (1 << i) != 0)
                .map(|(_, &item)| item)
                .collect();
            let selected = select_max_weight_date_intervals(&ranges).unwrap();
            for pair in selected.windows(2) {
                assert!(ranges[pair[0]].1 < ranges[pair[1]].0);
            }
            let obtained: i128 = selected.iter().map(|&i| i128::from(ranges[i].2)).sum();
            let mut expected = 0_i128;
            for mask in 0..(1_usize << ranges.len()) {
                let indices: Vec<_> = (0..ranges.len())
                    .filter(|&i| mask & (1 << i) != 0)
                    .collect();
                let disjoint = indices.iter().enumerate().all(|(k, &i)| {
                    indices
                        .iter()
                        .skip(k + 1)
                        .all(|&j| ranges[i].1 < ranges[j].0 || ranges[j].1 < ranges[i].0)
                });
                if disjoint {
                    expected = expected.max(indices.iter().map(|&i| i128::from(ranges[i].2)).sum());
                }
            }
            assert_eq!(obtained, expected, "mask {input}");
        }
    }

    #[test]
    fn handles_empty_negative_ties_and_invalid_ranges() {
        assert_eq!(select_max_weight_date_intervals(&[]), Some(vec![]));
        assert_eq!(
            select_max_weight_date_intervals(&[(date(1), date(1), -1)]),
            Some(vec![])
        );
        assert_eq!(
            select_max_weight_date_intervals(&[(date(1), date(1), 5), (date(1), date(1), 5)]),
            Some(vec![0])
        );
        assert_eq!(
            select_max_weight_date_intervals(&[(date(2), date(1), 3)]),
            None
        );
        assert_eq!(
            select_max_weight_date_intervals(&[
                (NaiveDate::MAX, NaiveDate::MAX, i64::MAX),
                (NaiveDate::MIN, NaiveDate::MIN, i64::MAX),
            ]),
            Some(vec![1, 0])
        );
    }
}
