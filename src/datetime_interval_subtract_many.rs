//! Subtract multiple inclusive datetime intervals at nanosecond resolution.

use chrono::{NaiveDateTime, TimeDelta};

/// Remove any number of inclusive datetime ranges from an inclusive source.
///
/// Exclusions can be unsorted or overlapping. Invalid ranges return None.
pub fn subtract_multiple_datetime_intervals(
    start: NaiveDateTime,
    end: NaiveDateTime,
    exclusions: &[(NaiveDateTime, NaiveDateTime)],
) -> Option<Vec<(NaiveDateTime, NaiveDateTime)>> {
    if start > end {
        return None;
    }

    let mut cursor = start;
    let mut result = Vec::new();
    for (excluded_start, excluded_end) in crate::merge_datetime_intervals(exclusions)? {
        if excluded_end < cursor {
            continue;
        }
        if excluded_start > end {
            break;
        }
        if excluded_start > cursor {
            let before = excluded_start.checked_sub_signed(TimeDelta::nanoseconds(1))?;
            result.push((cursor, before));
        }
        if excluded_end >= end {
            return Some(result);
        }
        cursor = excluded_end.checked_add_signed(TimeDelta::nanoseconds(1))?;
    }

    if cursor <= end {
        result.push((cursor, end));
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
    fn handles_unsorted_and_overlapping_exclusions() {
        let excluded = [(at(7), at(8)), (at(2), at(4)), (at(4), at(6))];
        assert_eq!(
            subtract_multiple_datetime_intervals(at(1), at(9), &excluded),
            Some(vec![
                (at(1), at(2).checked_sub_signed(TimeDelta::nanoseconds(1)).unwrap()),
                (at(6).checked_add_signed(TimeDelta::nanoseconds(1)).unwrap(),
                 at(7).checked_sub_signed(TimeDelta::nanoseconds(1)).unwrap()),
                (at(8).checked_add_signed(TimeDelta::nanoseconds(1)).unwrap(), at(9)),
            ])
        );
        assert_eq!(
            subtract_multiple_datetime_intervals(at(1), at(9), &[]),
            Some(vec![(at(1), at(9))])
        );
    }

    #[test]
    fn rejects_invalid_input_and_handles_full_exclusion() {
        assert_eq!(subtract_multiple_datetime_intervals(at(3), at(2), &[]), None);
        assert_eq!(
            subtract_multiple_datetime_intervals(at(1), at(9), &[(at(5), at(4))]),
            None
        );
        assert_eq!(
            subtract_multiple_datetime_intervals(at(3), at(5), &[(at(1), at(9))]),
            Some(vec![])
        );
    }

    #[test]
    fn preserves_edges_of_supported_datetime_range() {
        assert_eq!(
            subtract_multiple_datetime_intervals(
                NaiveDateTime::MIN,
                NaiveDateTime::MAX,
                &[(NaiveDateTime::MIN, NaiveDateTime::MIN),
                  (NaiveDateTime::MAX, NaiveDateTime::MAX)],
            ),
            Some(vec![(
                NaiveDateTime::MIN.checked_add_signed(TimeDelta::nanoseconds(1)).unwrap(),
                NaiveDateTime::MAX.checked_sub_signed(TimeDelta::nanoseconds(1)).unwrap(),
            )])
        );
    }
}
