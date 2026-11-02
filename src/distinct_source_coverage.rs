//! Sweep calendar intervals while counting distinct active source IDs.

use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};

/// Find inclusive date spans covered by at least `required_sources` distinct IDs.
///
/// Each tuple is `(source_id, first, last)`. Overlapping intervals from one
/// source count only once. Input ranges can be unordered, nested and adjacent.
/// Returns None for zero quorum or any reversed range. The sweep uses calendar
/// day ordinals, so a final event after NaiveDate::MAX cannot overflow dates.
pub fn coverage_by_distinct_sources(
    intervals: &[(usize, NaiveDate, NaiveDate)],
    required_sources: usize,
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if required_sources == 0 {
        return None;
    }
    let mut groups: BTreeMap<usize, Vec<(NaiveDate, NaiveDate)>> = BTreeMap::new();
    for &(source, first, last) in intervals {
        if first > last {
            return None;
        }
        groups.entry(source).or_default().push((first, last));
    }
    if required_sources > groups.len() {
        return Some(Vec::new());
    }

    let mut events = Vec::new();
    for ranges in groups.values() {
        for (first, last) in crate::merge_intervals(ranges)? {
            let lower = i64::from(first.num_days_from_ce());
            let upper_exclusive = i64::from(last.num_days_from_ce()) + 1;
            events.push((lower, 1_i64));
            events.push((upper_exclusive, -1_i64));
        }
    }
    events.sort_unstable_by_key(|(day, _)| *day);

    let quorum = i64::try_from(required_sources).ok()?;
    let mut active = 0_i64;
    let mut result: Vec<(NaiveDate, NaiveDate)> = Vec::new();
    let mut cursor = 0_usize;

    while cursor < events.len() {
        let day = events[cursor].0;
        while cursor < events.len() && events[cursor].0 == day {
            active += events[cursor].1;
            cursor += 1;
        }
        if cursor == events.len() || active < quorum {
            continue;
        }
        let end_day = events[cursor].0 - 1;
        let first = NaiveDate::from_num_days_from_ce_opt(i32::try_from(day).ok()?)?;
        let last = NaiveDate::from_num_days_from_ce_opt(i32::try_from(end_day).ok()?)?;
        if let Some(previous) = result.last_mut() {
            if previous.1.succ_opt() == Some(first) {
                previous.1 = last;
                continue;
            }
        }
        result.push((first, last));
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
    fn counts_distinct_sources_not_overlapping_input_records() {
        let intervals = [
            (0, date(1), date(4)),
            (0, date(3), date(6)),
            (1, date(3), date(5)),
            (2, date(4), date(7)),
        ];
        assert_eq!(
            coverage_by_distinct_sources(&intervals, 2),
            Some(vec![(date(3), date(6))])
        );
        assert_eq!(
            coverage_by_distinct_sources(&intervals, 3),
            Some(vec![(date(4), date(5))])
        );
        assert_eq!(
            coverage_by_distinct_sources(&intervals, 4),
            Some(vec![])
        );
    }

    #[test]
    fn handles_empty_invalid_and_extreme_ranges() {
        assert_eq!(coverage_by_distinct_sources(&[], 1), Some(vec![]));
        assert_eq!(coverage_by_distinct_sources(&[], 0), None);
        assert_eq!(
            coverage_by_distinct_sources(&[(0, date(5), date(4))], 1),
            None
        );
        assert_eq!(
            coverage_by_distinct_sources(
                &[
                    (0, NaiveDate::MIN, NaiveDate::MAX),
                    (1, NaiveDate::MIN, NaiveDate::MIN),
                    (1, NaiveDate::MAX, NaiveDate::MAX),
                ],
                2,
            ),
            Some(vec![
                (NaiveDate::MIN, NaiveDate::MIN),
                (NaiveDate::MAX, NaiveDate::MAX),
            ])
        );
    }

    #[test]
    fn matches_exhaustive_three_source_boolean_quorums() {
        for mask_a in 0_u32..8 {
            for mask_b in 0_u32..8 {
                for mask_c in 0_u32..8 {
                    let mut intervals = Vec::new();
                    for (source, mask) in [(0, mask_a), (1, mask_b), (2, mask_c)] {
                        for bit in 0..3 {
                            if mask & (1 << bit) != 0 {
                                intervals.push((source, date(bit + 1), date(bit + 1)));
                                intervals.push((source, date(bit + 1), date(bit + 1)));
                            }
                        }
                    }
                    let covered = coverage_by_distinct_sources(&intervals, 2).unwrap();
                    for bit in 0..3 {
                        let expected = [mask_a, mask_b, mask_c]
                            .iter()
                            .filter(|mask| **mask & (1 << bit) != 0)
                            .count()
                            >= 2;
                        let actual = covered
                            .iter()
                            .any(|(lo, hi)| *lo <= date(bit + 1) && date(bit + 1) <= *hi);
                        assert_eq!(actual, expected);
                    }
                }
            }
        }
    }
}
