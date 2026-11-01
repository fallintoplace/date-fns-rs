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
    let right = cut.succ_opt().filter(|next| *next <= end).map(|next| (next, end));
    Some(((start, cut), right))
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
        assert_eq!(split_interval_after(start, end, end), Some(((start, end), None)));
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
}
