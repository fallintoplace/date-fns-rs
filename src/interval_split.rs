//! Checked splitting of inclusive calendar intervals.

use chrono::NaiveDate;

/// Split an interval after cut, including cut in the left-hand part.
///
/// Returns None for reversed intervals or cuts outside [start, end].
/// The right-hand part is None when cut == end; an empty left is not allowed.
pub fn split_interval_after(
    start: NaiveDate,
    end: NaiveDate,
    cut: NaiveDate,
) -> Option<((NaiveDate, NaiveDate), Option<(NaiveDate, NaiveDate)>)> {
    if start > end || cut < start || cut > end {
        return None;
    }
    let right = cut
        .succ_opt()
        .filter(|next| *next <= end)
        .map(|next| (next, end));
    Some(((start, cut), right))
}

/// Split an inclusive date interval before a cut, keeping cut in the right part.
///
/// The left part is None when cut equals start. Reversed ranges and cuts
/// outside the inclusive interval return None.
pub fn split_interval_before(
    start: NaiveDate,
    end: NaiveDate,
    cut: NaiveDate,
) -> Option<(Option<(NaiveDate, NaiveDate)>, (NaiveDate, NaiveDate))> {
    if start > end || cut < start || cut > end {
        return None;
    }
    let left = cut
        .pred_opt()
        .filter(|previous| *previous >= start)
        .map(|previous| (start, previous));
    Some((left, (cut, end)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn splits_after_leap_day_without_losing_dates() {
        let start = date(2024, 2, 28);
        let leap_day = date(2024, 2, 29);
        let end = date(2024, 3, 1);
        assert_eq!(
            split_interval_after(start, end, leap_day),
            Some(((start, leap_day), Some((end, end))))
        );
        assert_eq!(
            split_interval_after(start, end, end),
            Some(((start, end), None))
        );
    }

    #[test]
    fn rejects_invalid_cuts_and_handles_maximum_date() {
        let start = date(2024, 2, 28);
        let end = date(2024, 3, 1);
        assert_eq!(split_interval_after(end, start, start), None);
        assert_eq!(split_interval_after(start, end, date(2024, 2, 27)), None);
        assert_eq!(split_interval_after(start, end, date(2024, 3, 2)), None);
        assert_eq!(
            split_interval_after(NaiveDate::MAX, NaiveDate::MAX, NaiveDate::MAX),
            Some(((NaiveDate::MAX, NaiveDate::MAX), None))
        );
    }
    #[test]
    fn splits_before_leap_day_and_handles_both_empty_edges() {
        let start = date(2024, 2, 28);
        let cut = date(2024, 2, 29);
        let end = date(2024, 3, 1);
        assert_eq!(
            split_interval_before(start, end, cut),
            Some((Some((start, start)), (cut, end)))
        );
        assert_eq!(split_interval_before(start, end, start), Some((None, (start, end))));
        assert_eq!(split_interval_before(start, end, end), Some((Some((start, cut)), (end, end))));
        assert_eq!(
            split_interval_before(NaiveDate::MIN, NaiveDate::MIN, NaiveDate::MIN),
            Some((None, (NaiveDate::MIN, NaiveDate::MIN)))
        );
        assert_eq!(split_interval_before(end, start, cut), None);
        assert_eq!(split_interval_before(start, end, date(2024, 2, 27)), None);
    }

}
