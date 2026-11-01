//! Adjacency of valid inclusive calendar intervals.

use chrono::NaiveDate;

/// Whether two non-overlapping intervals have no missing day between them.
///
/// Returns None for reversed input intervals. Identical or overlapping
/// intervals are not considered adjacent.
pub fn are_intervals_adjacent(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> Option<bool> {
    if a_start > a_end || b_start > b_end {
        return None;
    }
    Some(a_end.succ_opt() == Some(b_start) || b_end.succ_opt() == Some(a_start))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn detects_adjacency_across_leap_day() {
        let february = date(2024, 2, 28);
        let leap_day = date(2024, 2, 29);
        let march = date(2024, 3, 1);
        assert_eq!(
            are_intervals_adjacent(february, february, leap_day, march),
            Some(true)
        );
        assert_eq!(
            are_intervals_adjacent(leap_day, march, february, february),
            Some(true)
        );
        assert_eq!(
            are_intervals_adjacent(february, february, march, march),
            Some(false)
        );
    }

    #[test]
    fn excludes_overlap_invalid_order_and_overflow() {
        let one = date(2024, 1, 1);
        let two = date(2024, 1, 2);
        assert_eq!(are_intervals_adjacent(one, two, two, two), Some(false));
        assert_eq!(are_intervals_adjacent(two, one, one, two), None);
        let penultimate = NaiveDate::MAX.pred_opt().unwrap();
        assert_eq!(
            are_intervals_adjacent(penultimate, penultimate, NaiveDate::MAX, NaiveDate::MAX),
            Some(true)
        );
        assert_eq!(
            are_intervals_adjacent(NaiveDate::MAX, NaiveDate::MAX, one, one),
            Some(false)
        );
    }
}
