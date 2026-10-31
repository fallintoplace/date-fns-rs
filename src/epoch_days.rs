//! Checked conversion between calendar dates and Unix epoch days.

use chrono::{Datelike, NaiveDate};

const UNIX_EPOCH_CE_DAY: i64 = 719_163;

/// Return whole calendar days since 1970-01-01 (negative before the epoch).
///
/// This is a date-only count, not a Unix timestamp in seconds.
pub fn to_epoch_day(date: NaiveDate) -> i64 {
    i64::from(date.num_days_from_ce()) - UNIX_EPOCH_CE_DAY
}

/// Convert a signed Unix epoch-day count to a calendar date.
///
/// Returns None when the number is outside Chrono's supported date range.
pub fn from_epoch_day(days: i64) -> Option<NaiveDate> {
    let ce_day = days.checked_add(UNIX_EPOCH_CE_DAY)?;
    NaiveDate::from_num_days_from_ce_opt(i32::try_from(ce_day).ok()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn maps_epoch_and_pre_epoch_dates() {
        assert_eq!(to_epoch_day(date(1970, 1, 1)), 0);
        assert_eq!(to_epoch_day(date(1969, 12, 31)), -1);
        assert_eq!(from_epoch_day(1), Some(date(1970, 1, 2)));
        assert_eq!(from_epoch_day(-1), Some(date(1969, 12, 31)));
        assert_eq!(to_epoch_day(date(2000, 1, 1)), 10_957);
    }

    #[test]
    fn round_trips_leap_days_and_supported_limits() {
        for value in [date(2024, 2, 29), NaiveDate::MIN, NaiveDate::MAX] {
            assert_eq!(from_epoch_day(to_epoch_day(value)), Some(value));
        }
    }

    #[test]
    fn rejects_unrepresentable_day_counts() {
        assert_eq!(from_epoch_day(i64::MIN), None);
        assert_eq!(from_epoch_day(i64::MAX), None);
        assert_eq!(from_epoch_day(to_epoch_day(NaiveDate::MAX) + 1), None);
        assert_eq!(from_epoch_day(to_epoch_day(NaiveDate::MIN) - 1), None);
    }
}
