//! Middle day of an inclusive calendar interval.

use chrono::{Datelike, NaiveDate};

/// Return the middle date of an inclusive range.
///
/// Even-length ranges choose the earlier of the two central dates.
/// Reversed ranges return None, without overflowing at supported limits.
pub fn interval_midpoint(start: NaiveDate, end: NaiveDate) -> Option<NaiveDate> {
    if start > end {
        return None;
    }
    let first = i64::from(start.num_days_from_ce());
    let last = i64::from(end.num_days_from_ce());
    let middle = first + (last - first) / 2;
    NaiveDate::from_num_days_from_ce_opt(i32::try_from(middle).ok()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn chooses_earlier_midpoint_for_even_ranges() {
        let start = date(2024, 2, 28);
        assert_eq!(interval_midpoint(start, date(2024, 3, 1)), Some(date(2024, 2, 29)));
        assert_eq!(interval_midpoint(start, date(2024, 2, 29)), Some(start));
        assert_eq!(interval_midpoint(start, start), Some(start));
        assert_eq!(interval_midpoint(date(2024, 3, 1), start), None);
    }

    #[test]
    fn handles_full_chrono_date_range_without_overflow() {
        let result = interval_midpoint(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert!(result > NaiveDate::MIN && result < NaiveDate::MAX);
        assert_eq!(
            interval_midpoint(NaiveDate::MAX, NaiveDate::MAX),
            Some(NaiveDate::MAX)
        );
    }
}
