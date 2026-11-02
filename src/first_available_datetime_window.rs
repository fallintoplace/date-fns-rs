//! Find free datetime windows at exact nanosecond resolution.

use chrono::{DateTime, NaiveDateTime, Utc};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

fn to_nanos(value: NaiveDateTime) -> Option<i128> {
    let utc = value.and_utc();
    let nanos = utc.timestamp_subsec_nanos();
    if nanos >= 1_000_000_000 {
        return None;
    }
    Some(i128::from(utc.timestamp()) * NANOS_PER_SECOND + i128::from(nanos))
}

fn from_nanos(value: i128) -> Option<NaiveDateTime> {
    let seconds = i64::try_from(value.div_euclid(NANOS_PER_SECOND)).ok()?;
    let nanos = u32::try_from(value.rem_euclid(NANOS_PER_SECOND)).ok()?;
    DateTime::<Utc>::from_timestamp(seconds, nanos).map(|dt| dt.naive_utc())
}

/// Find the earliest gap of at least `required_nanoseconds` inclusive ticks.
///
/// Search is inside the inclusive [start, end] range. Blocked ranges may be
/// unsorted, overlapping, and outside the search range. No time zone is chosen.
/// An outer None means an invalid argument, reversed blocked interval, or
/// an unsupported leap-second representation. Some(None) means no free gap.
/// Uses i128 timestamp arithmetic and O(n log n) interval normalization.
pub fn first_available_datetime_window(
    start: NaiveDateTime,
    end: NaiveDateTime,
    blocked: &[(NaiveDateTime, NaiveDateTime)],
    required_nanoseconds: i128,
) -> Option<Option<(NaiveDateTime, NaiveDateTime)>> {
    if start > end || required_nanoseconds <= 0 {
        return None;
    }
    let from = to_nanos(start)?;
    let through = to_nanos(end)?;
    let exclusions = crate::merge_datetime_intervals(blocked)?;
    if required_nanoseconds > through - from + 1 {
        return Some(None);
    }
    let mut cursor = from;
    let needed_offset = required_nanoseconds - 1;

    for (first, last) in exclusions {
        let blocked_start = to_nanos(first)?;
        let blocked_end = to_nanos(last)?;
        if blocked_end < cursor {
            continue;
        }
        if blocked_start > through {
            break;
        }
        let end_of_candidate = cursor + needed_offset;
        if blocked_start > cursor && end_of_candidate < blocked_start && end_of_candidate <= through
        {
            return Some(Some((from_nanos(cursor)?, from_nanos(end_of_candidate)?)));
        }
        cursor = blocked_end + 1;
        if cursor > through {
            return Some(None);
        }
    }

    let end_of_candidate = cursor + needed_offset;
    if end_of_candidate > through {
        return Some(None);
    }
    Some(Some((from_nanos(cursor)?, from_nanos(end_of_candidate)?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(nanos: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, nanos)
            .unwrap()
    }

    #[test]
    fn finds_earliest_free_window_between_overlapping_blocks() {
        let blocked = [(at(7), at(7)), (at(2), at(3)), (at(3), at(3))];
        assert_eq!(
            first_available_datetime_window(at(0), at(10), &blocked, 2),
            Some(Some((at(0), at(1))))
        );
        assert_eq!(
            first_available_datetime_window(at(0), at(10), &blocked, 3),
            Some(Some((at(4), at(6))))
        );
        assert_eq!(
            first_available_datetime_window(at(0), at(10), &blocked, 5),
            Some(None)
        );
        assert_eq!(
            first_available_datetime_window(at(0), at(10), &[], 11),
            Some(Some((at(0), at(10))))
        );
    }

    #[test]
    fn matches_brute_force_windows_for_small_exclusion_sets() {
        let candidates = [(at(2), at(4)), (at(5), at(8)), (at(9), at(9))];
        for mask in 0..8_u32 {
            let blocked: Vec<_> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, value)| *value)
                .collect();
            for count in 1..=12_u32 {
                let expected = (0..=10_u32).find_map(|start| {
                    let end = start.checked_add(count - 1)?;
                    if end > 10 {
                        return None;
                    }
                    (start..=end)
                        .all(|tick| {
                            !blocked
                                .iter()
                                .any(|(a, b)| *a <= at(tick) && at(tick) <= *b)
                        })
                        .then_some((at(start), at(end)))
                });
                assert_eq!(
                    first_available_datetime_window(at(0), at(10), &blocked, i128::from(count)),
                    Some(expected),
                    "mask {mask}, count {count}"
                );
            }
        }
    }

    #[test]
    fn distinguishes_invalid_queries_from_exhausted_capacity() {
        assert_eq!(first_available_datetime_window(at(2), at(1), &[], 1), None);
        assert_eq!(first_available_datetime_window(at(0), at(1), &[], 0), None);
        assert_eq!(
            first_available_datetime_window(at(0), at(1), &[(at(2), at(1))], 1),
            None
        );
        assert_eq!(
            first_available_datetime_window(at(0), at(1), &[], i128::MAX),
            Some(None)
        );
        assert_eq!(
            first_available_datetime_window(NaiveDateTime::MIN, NaiveDateTime::MAX, &[], 1),
            Some(Some((NaiveDateTime::MIN, NaiveDateTime::MIN)))
        );
        assert_eq!(
            first_available_datetime_window(
                NaiveDateTime::MAX,
                NaiveDateTime::MAX,
                &[(NaiveDateTime::MAX, NaiveDateTime::MAX)],
                1
            ),
            Some(None)
        );
    }
}
