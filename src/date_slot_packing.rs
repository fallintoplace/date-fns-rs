//! Greedy packing of full-day slots into free inclusive calendar windows.

use chrono::{Days, NaiveDate};

fn fill_gap(
    start: NaiveDate,
    end: NaiveDate,
    slot_days: u64,
    gap_days: u64,
    limit: usize,
    output: &mut Vec<(NaiveDate, NaiveDate)>,
) {
    let mut cursor = Some(start);
    while output.len() < limit {
        let Some(first) = cursor else {
            break;
        };
        let Some(last) = first.checked_add_days(Days::new(slot_days - 1)) else {
            break;
        };
        if last > end {
            break;
        }
        output.push((first, last));
        cursor = last
            .succ_opt()
            .and_then(|following| following.checked_add_days(Days::new(gap_days)));
    }
}

/// Pack the earliest fixed-length slots inside an inclusive date range.
///
/// Excluded intervals may overlap or extend beyond the search window.
/// A slot occupies slot_days consecutive dates. gap_days free dates are
/// required between slots within one contiguous free segment. Returns None
/// for reversed ranges or a zero slot length, and caps output by limit.
pub fn pack_date_slots(
    start: NaiveDate,
    end: NaiveDate,
    exclusions: &[(NaiveDate, NaiveDate)],
    slot_days: u64,
    gap_days: u64,
    limit: usize,
) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    if start > end || slot_days == 0 {
        return None;
    }
    let occupied = crate::merge_intervals(exclusions)?;
    let mut result = Vec::new();
    if limit == 0 {
        return Some(result);
    }

    let mut cursor = Some(start);
    for (first, last) in occupied {
        let Some(current) = cursor else {
            break;
        };
        if first > end {
            break;
        }
        if last < current {
            continue;
        }
        if first > current {
            fill_gap(
                current,
                first.pred_opt()?,
                slot_days,
                gap_days,
                limit,
                &mut result,
            );
        }
        if result.len() >= limit {
            return Some(result);
        }
        cursor = last.succ_opt().filter(|next| *next <= end);
    }
    if let Some(current) = cursor {
        fill_gap(current, end, slot_days, gap_days, limit, &mut result);
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
    fn fills_earliest_free_ranges_and_respects_gaps() {
        assert_eq!(
            pack_date_slots(day(1), day(20), &[(day(8), day(10))], 3, 1, 10),
            Some(vec![
                (day(1), day(3)),
                (day(5), day(7)),
                (day(11), day(13)),
                (day(15), day(17)),
            ])
        );
        assert_eq!(
            pack_date_slots(day(1), day(20), &[], 2, 0, 2),
            Some(vec![(day(1), day(2)), (day(3), day(4))])
        );
    }

    #[test]
    fn validates_input_and_handles_limits_without_day_iteration() {
        assert_eq!(pack_date_slots(day(3), day(1), &[], 1, 0, 3), None);
        assert_eq!(pack_date_slots(day(1), day(3), &[], 0, 0, 3), None);
        assert_eq!(
            pack_date_slots(day(1), day(3), &[(day(2), day(1))], 1, 0, 3),
            None
        );
        assert_eq!(
            pack_date_slots(NaiveDate::MAX, NaiveDate::MAX, &[], 1, 0, 5),
            Some(vec![(NaiveDate::MAX, NaiveDate::MAX)])
        );
        assert_eq!(
            pack_date_slots(NaiveDate::MIN, NaiveDate::MAX, &[], 1, 0, 3)
                .unwrap()
                .len(),
            3
        );
        assert_eq!(pack_date_slots(day(1), day(3), &[], 1, 0, 0), Some(vec![]));
    }
}
