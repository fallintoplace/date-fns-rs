//! Subtraction of multiple inclusive date intervals from one source range.

use chrono::NaiveDate;

/// Remove any number of inclusive exclusion ranges from a source interval.
///
/// Exclusions may overlap, touch or arrive unsorted. Results are nonoverlapping
/// and chronological. Returns None if the source or any exclusion is reversed.
pub fn subtract_multiple_intervals(
    start: NaiveDate,
    end: NaiveDate,
    exclusions: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if start > end {
        return None;
    }
    let mut cursor = start;
    let mut result = Vec::new();
    for (excluded_start, excluded_end) in crate::merge_intervals(exclusions)? {
        if excluded_end < cursor {
            continue;
        }
        if excluded_start > end {
            break;
        }
        if excluded_start > cursor {
            result.push((cursor, excluded_start.pred_opt()?));
        }
        if excluded_end >= end {
            return Some(result);
        }
        cursor = excluded_end.succ_opt()?;
    }
    if cursor <= end {
        result.push((cursor, end));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn subtracts_unsorted_overlapping_exclusions_without_gaps() {
        let excluded = [(date(8), date(9)), (date(4), date(5)), (date(2), date(4))];
        assert_eq!(
            subtract_multiple_intervals(date(1), date(10), &excluded),
            Some(vec![
                (date(1), date(1)),
                (date(6), date(7)),
                (date(10), date(10)),
            ])
        );
        assert_eq!(
            subtract_multiple_intervals(date(1), date(10), &[]),
            Some(vec![(date(1), date(10))])
        );
    }

    #[test]
    fn rejects_invalid_exclusions_and_handles_date_extremes() {
        assert_eq!(subtract_multiple_intervals(date(2), date(1), &[]), None);
        assert_eq!(
            subtract_multiple_intervals(date(1), date(10), &[(date(9), date(8))]),
            None
        );
        assert_eq!(
            subtract_multiple_intervals(
                NaiveDate::MIN,
                NaiveDate::MAX,
                &[
                    (NaiveDate::MIN, NaiveDate::MIN),
                    (NaiveDate::MAX, NaiveDate::MAX)
                ]
            ),
            Some(vec![(
                NaiveDate::MIN.succ_opt().unwrap(),
                NaiveDate::MAX.pred_opt().unwrap(),
            )])
        );
        assert_eq!(
            subtract_multiple_intervals(date(3), date(5), &[(date(1), date(20))]),
            Some(vec![])
        );
    }
}
