//! Greedy maximum-cardinality scheduling for inclusive calendar date ranges.

use chrono::NaiveDate;

/// Select the largest possible number of non-overlapping inclusive date ranges.
///
/// Returns original indices, ordered by the selected ranges' end dates.
/// Intervals may arrive unsorted or contain duplicates. Equal endpoints
/// overlap and cannot both be selected. When choices tie, prefer earlier starts
/// and then lower original indices. Complexity is O(n log n).
pub fn select_max_nonoverlapping_date_intervals(
    intervals: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<usize>> {
    if intervals.iter().any(|(start, end)| start > end) {
        return None;
    }
    let mut order: Vec<usize> = (0..intervals.len()).collect();
    order.sort_unstable_by_key(|&i| (intervals[i].1, intervals[i].0, i));
    let mut result = Vec::new();
    let mut last_end = None;
    for index in order {
        let (first, last) = intervals[index];
        if last_end.map_or(true, |end| first > end) {
            result.push(index);
            last_end = Some(last);
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, n).unwrap()
    }

    #[test]
    fn prefers_short_intervals_to_maximize_schedule_count() {
        let ranges = [
            (day(1), day(9)),
            (day(1), day(2)),
            (day(3), day(4)),
            (day(5), day(6)),
            (day(7), day(8)),
        ];
        assert_eq!(
            select_max_nonoverlapping_date_intervals(&ranges),
            Some(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn matches_brute_force_optimal_cardinality() {
        let candidates = [
            (day(1), day(3)),
            (day(2), day(2)),
            (day(3), day(5)),
            (day(5), day(6)),
            (day(7), day(7)),
        ];
        for input_mask in 0..32_u32 {
            let ranges: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| input_mask & (1 << i) != 0)
                .map(|(_, range)| *range)
                .collect();
            let selected = select_max_nonoverlapping_date_intervals(&ranges).unwrap();
            for pair in selected.windows(2) {
                assert!(ranges[pair[0]].1 < ranges[pair[1]].0);
            }
            let mut best = 0;
            for mask in 0..(1_usize << ranges.len()) {
                let chosen: Vec<_> = ranges
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, range)| *range)
                    .collect();
                if chosen
                    .iter()
                    .enumerate()
                    .all(|(i, &(a, b))| chosen.iter().skip(i + 1).all(|&(c, d)| b < c || d < a))
                {
                    best = best.max(chosen.len());
                }
            }
            assert_eq!(selected.len(), best);
        }
    }

    #[test]
    fn handles_duplicate_touching_invalid_and_extreme_ranges() {
        assert_eq!(select_max_nonoverlapping_date_intervals(&[]), Some(vec![]));
        assert_eq!(
            select_max_nonoverlapping_date_intervals(&[(day(1), day(1)); 2]),
            Some(vec![0])
        );
        assert_eq!(
            select_max_nonoverlapping_date_intervals(&[(day(2), day(1))]),
            None
        );
        assert_eq!(
            select_max_nonoverlapping_date_intervals(&[
                (NaiveDate::MAX, NaiveDate::MAX),
                (NaiveDate::MIN, NaiveDate::MIN)
            ]),
            Some(vec![1, 0])
        );
    }
}
