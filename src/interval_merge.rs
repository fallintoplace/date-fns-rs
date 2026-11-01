//! Normalize overlapping and adjacent inclusive calendar intervals.

use chrono::NaiveDate;

/// Merge overlapping and adjacent inclusive intervals in chronological order.
///
/// Inputs may be unsorted. Returns `None` if any interval is reversed.
/// Consecutive intervals also merge because there is no calendar day between
/// their inclusive endpoints. An empty input returns an empty vector.
pub fn merge_intervals(
    intervals: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if intervals.iter().any(|(start, end)| start > end) {
        return None;
    }

    let mut sorted = intervals.to_vec();
    sorted.sort_unstable_by_key(|(start, _)| *start);

    let mut merged: Vec<(NaiveDate, NaiveDate)> = Vec::with_capacity(sorted.len());
    for (start, end) in sorted {
        if let Some(previous) = merged.last_mut() {
            if start <= previous.1 || previous.1.succ_opt() == Some(start) {
                previous.1 = previous.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    Some(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn merges_unsorted_nested_overlapping_and_adjacent_ranges() {
        let input = [
            (date(8), date(10)),
            (date(2), date(4)),
            (date(4), date(6)),
            (date(1), date(3)),
            (date(9), date(9)),
            (date(7), date(8)),
        ];
        assert_eq!(merge_intervals(&input), Some(vec![(date(1), date(10))]));
    }

    #[test]
    fn preserves_gaps_and_handles_empty_input() {
        assert_eq!(merge_intervals(&[]), Some(vec![]));
        assert_eq!(
            merge_intervals(&[(date(6), date(7)), (date(1), date(2))]),
            Some(vec![(date(1), date(2)), (date(6), date(7))])
        );
    }

    #[test]
    fn rejects_reversed_intervals_without_partial_results() {
        assert_eq!(merge_intervals(&[(date(2), date(1))]), None);
        assert_eq!(
            merge_intervals(&[(date(1), date(2)), (date(9), date(8))]),
            None
        );
    }

    #[test]
    fn handles_chrono_date_boundaries() {
        let penultimate = NaiveDate::MAX.pred_opt().unwrap();
        assert_eq!(
            merge_intervals(&[
                (NaiveDate::MAX, NaiveDate::MAX),
                (penultimate, penultimate),
                (NaiveDate::MIN, NaiveDate::MIN),
            ]),
            Some(vec![
                (NaiveDate::MIN, NaiveDate::MIN),
                (penultimate, NaiveDate::MAX),
            ])
        );
    }
}
