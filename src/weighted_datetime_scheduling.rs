//! Maximum-weight scheduling of inclusive nanosecond datetime ranges.

use chrono::NaiveDateTime;

/// Select nonoverlapping inclusive timestamp ranges with maximum total weight.
///
/// Equal endpoints overlap. Negative weights may be omitted. Returns original
/// indices in chronological order and None for reversed ranges. No time zone
/// is assumed. Complexity is O(n log n) time and O(n) memory.
pub fn select_max_weight_datetime_intervals(
    intervals: &[(NaiveDateTime, NaiveDateTime, i64)],
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
    use chrono::{NaiveDate, TimeDelta};

    fn tick(n: i64) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
            .and_hms_opt(0, 0, 0).unwrap()
            .checked_add_signed(TimeDelta::nanoseconds(n)).unwrap()
    }

    #[test]
    fn chooses_precise_disjoint_time_slots() {
        let input = [
            (tick(0), tick(20), 7),
            (tick(0), tick(9), 5),
            (tick(10), tick(20), 8),
            (tick(20), tick(21), 20),
        ];
        assert_eq!(select_max_weight_datetime_intervals(&input), Some(vec![1, 3]));
    }

    #[test]
    fn verifies_optimal_weight_on_small_nanosecond_ranges() {
        let candidates = [
            (tick(0), tick(2), 3),
            (tick(2), tick(4), 10),
            (tick(3), tick(6), 6),
            (tick(5), tick(7), 9),
            (tick(8), tick(8), -10),
        ];
        for input in 0..32_u32 {
            let ranges: Vec<_> = candidates.iter().enumerate()
                .filter(|(i, _)| input & (1 << i) != 0)
                .map(|(_, &item)| item).collect();
            let selected = select_max_weight_datetime_intervals(&ranges).unwrap();
            let obtained: i128 = selected.iter().map(|&i| i128::from(ranges[i].2)).sum();
            for pair in selected.windows(2) {
                assert!(ranges[pair[0]].1 < ranges[pair[1]].0);
            }
            let mut expected = 0_i128;
            for mask in 0..(1_usize << ranges.len()) {
                let indices: Vec<_> = (0..ranges.len())
                    .filter(|&i| mask & (1 << i) != 0).collect();
                if indices.iter().enumerate().all(|(k, &i)| {
                    indices.iter().skip(k + 1).all(|&j| {
                        ranges[i].1 < ranges[j].0 || ranges[j].1 < ranges[i].0
                    })
                }) {
                    expected = expected.max(indices.iter().map(|&i| i128::from(ranges[i].2)).sum());
                }
            }
            assert_eq!(obtained, expected, "mask {input}");
        }
    }

    #[test]
    fn includes_maximum_instants_and_rejects_bad_ranges() {
        assert_eq!(select_max_weight_datetime_intervals(&[]), Some(vec![]));
        assert_eq!(select_max_weight_datetime_intervals(&[(tick(2), tick(1), 8)]), None);
        assert_eq!(select_max_weight_datetime_intervals(&[
            (NaiveDateTime::MIN, NaiveDateTime::MIN, i64::MAX),
            (NaiveDateTime::MAX, NaiveDateTime::MAX, i64::MAX),
        ]), Some(vec![0, 1]));
    }
}
