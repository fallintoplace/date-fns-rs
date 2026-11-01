//! Missing calendar dates strictly between two inclusive intervals.

use chrono::NaiveDate;

/// Return the gap between two inclusive calendar intervals.
///
/// None means at least one input interval is reversed. Some(None) means the
/// intervals overlap or are adjacent. Some(Some((start, end))) is the missing
/// inclusive range, and is independent of input order.
pub fn gap_between_intervals(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> Option<Option<(NaiveDate, NaiveDate)>> {
    if a_start > a_end || b_start > b_end {
        return None;
    }
    let (left_end, right_start) = if a_start <= b_start {
        (a_end, b_start)
    } else {
        (b_end, a_start)
    };
    Some(match (left_end.succ_opt(), right_start.pred_opt()) {
        (Some(start), Some(end)) if start <= end => Some((start, end)),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_gap_and_handles_reversed_argument_order() {
        let a = date(2024, 2, 28);
        let b = date(2024, 3, 2);
        let c = date(2024, 3, 5);
        let expected = Some(Some((date(2024, 3, 3), date(2024, 3, 4))));
        assert_eq!(gap_between_intervals(a, b, c, c), expected);
        assert_eq!(gap_between_intervals(c, c, a, b), expected);
    }

    #[test]
    fn distinguishes_adjacent_overlapping_and_invalid() {
        let a = date(2024, 1, 1);
        let b = date(2024, 1, 2);
        let c = date(2024, 1, 3);
        assert_eq!(gap_between_intervals(a, b, c, c), Some(None));
        assert_eq!(gap_between_intervals(a, c, b, c), Some(None));
        assert_eq!(gap_between_intervals(b, a, a, c), None);
        assert_eq!(
            gap_between_intervals(NaiveDate::MAX, NaiveDate::MAX, b, c),
            Some(Some((date(2024, 1, 4), NaiveDate::MAX.pred_opt().unwrap())))
        );
    }
}
