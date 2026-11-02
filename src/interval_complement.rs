//! Compute uncovered days inside an inclusive calendar-date interval.

use chrono::NaiveDate;

/// Return the uncovered inclusive ranges inside `[start, end]`.
///
/// Exclusions may overlap, be unsorted, or extend beyond the search window.
/// All inputs must have ordered endpoints. This uses O(n log n) time to sort
/// exclusions and O(n) additional space, never iterating individual dates.
pub fn complement_date_intervals(
    start: NaiveDate,
    end: NaiveDate,
    exclusions: &[(NaiveDate, NaiveDate)],
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if start > end || exclusions.iter().any(|(first, last)| first > last) {
        return None;
    }

    let mut relevant: Vec<_> = exclusions
        .iter()
        .copied()
        .filter(|(first, last)| *first <= end && *last >= start)
        .collect();
    relevant.sort_unstable_by_key(|(first, _)| *first);

    let mut gaps = Vec::new();
    let mut next = Some(start);
    for (first, last) in relevant {
        let Some(cursor) = next else {
            break;
        };
        if last < cursor {
            continue;
        }
        if first > cursor {
            gaps.push((cursor, first.pred_opt()?));
        }
        next = last.succ_opt().filter(|following| *following <= end);
    }

    if let Some(cursor) = next {
        gaps.push((cursor, end));
    }
    Some(gaps)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn clips_unsorted_nested_and_overlapping_exclusions() {
        let input = [
            (date(7), date(12)),
            (date(2), date(3)),
            (date(3), date(5)),
            (date(1), date(1)),
            (date(8), date(8)),
        ];
        assert_eq!(
            complement_date_intervals(date(2), date(10), &input),
            Some(vec![(date(6), date(6))])
        );
    }

    #[test]
    fn distinguishes_empty_full_and_invalid_intervals() {
        assert_eq!(
            complement_date_intervals(date(1), date(3), &[]),
            Some(vec![(date(1), date(3))])
        );
        assert_eq!(
            complement_date_intervals(date(1), date(3), &[(date(1), date(3))]),
            Some(vec![])
        );
        assert_eq!(complement_date_intervals(date(3), date(1), &[]), None);
        assert_eq!(
            complement_date_intervals(date(1), date(3), &[(date(3), date(2))]),
            None
        );
    }

    #[test]
    fn preserves_chrono_minimum_and_maximum_boundaries() {
        let second = NaiveDate::MIN.succ_opt().unwrap();
        let before_last = NaiveDate::MAX.pred_opt().unwrap();
        assert_eq!(
            complement_date_intervals(NaiveDate::MIN, NaiveDate::MAX, &[(second, before_last)]),
            Some(vec![
                (NaiveDate::MIN, NaiveDate::MIN),
                (NaiveDate::MAX, NaiveDate::MAX),
            ])
        );
        assert_eq!(
            complement_date_intervals(
                NaiveDate::MAX,
                NaiveDate::MAX,
                &[(NaiveDate::MAX, NaiveDate::MAX)]
            ),
            Some(vec![])
        );
    }

    #[test]
    fn matches_exhaustive_six_day_bitmasks() {
        for mask in 0_u32..64 {
            let exclusions: Vec<_> = (0..6)
                .filter(|bit| mask & (1 << bit) != 0)
                .map(|bit| (date(bit + 1), date(bit + 1)))
                .collect();
            let gaps = complement_date_intervals(date(1), date(6), &exclusions).unwrap();
            for day in 1..=6 {
                let uncovered = gaps
                    .iter()
                    .any(|(first, last)| *first <= date(day) && date(day) <= *last);
                assert_eq!(uncovered, mask & (1 << (day - 1)) == 0);
            }
        }
    }
}
