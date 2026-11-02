//! Subtract sets of inclusive naive datetime intervals at nanosecond precision.

use chrono::{NaiveDateTime, TimeDelta};

/// Remove all right-hand ranges from all left-hand ranges.
///
/// Inputs may be unsorted, overlapping, or adjacent. Invalid ranges return
/// None. Normalization costs O(n log n + m log m); subtraction is O(n + m).
pub fn subtract_datetime_interval_sets(
    left: &[(NaiveDateTime, NaiveDateTime)],
    right: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    let sources = crate::merge_datetime_intervals(left)?;
    let exclusions = crate::merge_datetime_intervals(right)?;
    let mut result = Vec::new();
    let mut j = 0;
    let nano = TimeDelta::nanoseconds(1);

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
                    result.push((current, excluded_start.checked_sub_signed(nano)?));
                }
                if excluded_end >= end {
                    cursor = None;
                    break;
                }
                cursor = excluded_end.checked_add_signed(nano);
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
    use chrono::NaiveDate;

    fn at(second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(12, 0, second)
            .unwrap()
    }

    #[test]
    fn subtracts_unsorted_exclusions_and_preserves_exact_nanos() {
        let nano = TimeDelta::nanoseconds(1);
        assert_eq!(
            subtract_datetime_interval_sets(
                &[(at(7), at(9)), (at(1), at(5))],
                &[(at(3), at(8)), (at(2), at(2))]
            ),
            Some(vec![
                (at(1), at(2).checked_sub_signed(nano).unwrap()),
                (
                    at(2).checked_add_signed(nano).unwrap(),
                    at(3).checked_sub_signed(nano).unwrap()
                ),
                (at(8).checked_add_signed(nano).unwrap(), at(9))
            ])
        );
    }

    #[test]
    fn brute_force_membership_matches_set_difference() {
        for a in 1..=6 {
            for b in a..=6 {
                for c in 1..=6 {
                    for d in c..=6 {
                        let output =
                            subtract_datetime_interval_sets(&[(at(a), at(b))], &[(at(c), at(d))])
                                .unwrap();
                        for n in 1..=6 {
                            let contains =
                                output.iter().any(|(lo, hi)| *lo <= at(n) && at(n) <= *hi);
                            assert_eq!(contains, (a <= n && n <= b) && !(c <= n && n <= d));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn validates_inputs_and_preserves_extreme_endpoints() {
        assert_eq!(subtract_datetime_interval_sets(&[], &[]), Some(vec![]));
        assert_eq!(
            subtract_datetime_interval_sets(&[(at(4), at(2))], &[]),
            None
        );
        assert_eq!(
            subtract_datetime_interval_sets(&[], &[(at(4), at(2))]),
            None
        );
        let nano = TimeDelta::nanoseconds(1);
        assert_eq!(
            subtract_datetime_interval_sets(
                &[(NaiveDateTime::MIN, NaiveDateTime::MAX)],
                &[
                    (NaiveDateTime::MAX, NaiveDateTime::MAX),
                    (NaiveDateTime::MIN, NaiveDateTime::MIN)
                ]
            ),
            Some(vec![(
                NaiveDateTime::MIN.checked_add_signed(nano).unwrap(),
                NaiveDateTime::MAX.checked_sub_signed(nano).unwrap()
            )])
        );
    }
}
