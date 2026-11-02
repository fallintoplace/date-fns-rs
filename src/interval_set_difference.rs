//! Subtract a union of inclusive date intervals from another union.

use chrono::NaiveDate;

/// Remove the right-hand set of intervals from the left-hand set.
///
/// Inputs can be unsorted and overlapping. The result is sorted, disjoint,
/// and inclusive. Invalid input ranges return None. After normalization,
/// the sweep is O(n + m), independent of the number of days in any range.
pub fn subtract_interval_sets(
    left: &[(NaiveDate, NaiveDate)],
    right: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    let sources = crate::merge_intervals(left)?;
    let exclusions = crate::merge_intervals(right)?;
    let mut result = Vec::new();
    let mut j = 0;

    for (start, end) in sources {
        while j < exclusions.len() && exclusions[j].1 < start {
            j += 1;
        }
        let mut cursor = Some(start);
        let mut k = j;
        while k < exclusions.len() && exclusions[k].0 <= end {
            let (excluded_start, excluded_end) = exclusions[k];
            if let Some(current) = cursor {
                if excluded_start > current {
                    result.push((current, excluded_start.pred_opt()?));
                }
                if excluded_end >= end {
                    cursor = None;
                    break;
                }
                cursor = excluded_end.succ_opt();
            }
            k += 1;
        }
        if let Some(current) = cursor {
            if current <= end {
                result.push((current, end));
            }
        }
        j = k;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn removes_overlapping_unsorted_ranges_and_preserves_gaps() {
        assert_eq!(
            subtract_interval_sets(
                &[(d(10), d(14)), (d(1), d(8)), (d(7), d(7))],
                &[(d(5), d(12)), (d(2), d(3))]
            ),
            Some(vec![(d(1), d(1)), (d(4), d(4)), (d(13), d(14))])
        );
        assert_eq!(
            subtract_interval_sets(&[(d(1), d(2))], &[]),
            Some(vec![(d(1), d(2))])
        );
    }

    #[test]
    fn agrees_with_brute_force_for_short_intervals() {
        for a in 1..=7 {
            for b in a..=7 {
                for c in 1..=7 {
                    for d in c..=7 {
                        let result = subtract_interval_sets(
                            &[(d_date(a), d_date(b))],
                            &[(d_date(c), d_date(d))],
                        )
                        .unwrap();
                        for n in 1..=7 {
                            let actual = result
                                .iter()
                                .any(|(lo, hi)| *lo <= d_date(n) && d_date(n) <= *hi);
                            assert_eq!(actual, (a <= n && n <= b) && !(c <= n && n <= d));
                        }
                    }
                }
            }
        }
    }

    fn d_date(day: u32) -> NaiveDate {
        d(day)
    }

    #[test]
    fn validates_all_ranges_and_handles_absolute_edges() {
        assert_eq!(subtract_interval_sets(&[(d(3), d(2))], &[]), None);
        assert_eq!(subtract_interval_sets(&[], &[(d(3), d(2))]), None);
        assert_eq!(subtract_interval_sets(&[], &[]), Some(vec![]));
        assert_eq!(
            subtract_interval_sets(
                &[(NaiveDate::MIN, NaiveDate::MAX)],
                &[
                    (NaiveDate::MIN, NaiveDate::MIN),
                    (NaiveDate::MAX, NaiveDate::MAX)
                ]
            ),
            Some(vec![(
                NaiveDate::MIN.succ_opt().unwrap(),
                NaiveDate::MAX.pred_opt().unwrap()
            )])
        );
    }
}
