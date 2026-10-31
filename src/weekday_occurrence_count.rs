//! Constant-time counts of a specified weekday in inclusive date intervals.

use chrono::{Datelike, NaiveDate, Weekday};

use crate::difference::difference_in_calendar_days;

/// Count the occurrences of one weekday between two dates, inclusive.
///
/// A reversed interval returns None. Works in constant time regardless
/// of how many years lie between the endpoints.
pub fn count_weekday_occurrences(
    start: NaiveDate,
    end: NaiveDate,
    weekday: Weekday,
) -> Option<i64> {
    if start > end {
        return None;
    }
    let length = difference_in_calendar_days(end, start) + 1;
    let offset = (i64::from(weekday.num_days_from_monday())
        - i64::from(start.weekday().num_days_from_monday()))
    .rem_euclid(7);

    Some(if offset >= length {
        0
    } else {
        1 + (length - offset - 1) / 7
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn agrees_with_weekday_iterator_over_short_ranges() {
        let start = date(2024, 2, 27);
        for weekday in [Weekday::Mon, Weekday::Tue, Weekday::Sat] {
            for days in 0..35 {
                let end = start + chrono::Duration::days(days);
                let count = crate::each_weekday_of_interval(start, end, weekday)
                    .unwrap()
                    .count() as i64;
                assert_eq!(count_weekday_occurrences(start, end, weekday), Some(count));
            }
        }
    }

    #[test]
    fn handles_empty_matches_reversed_ranges_and_extremes() {
        let mon = date(2024, 3, 4);
        assert_eq!(count_weekday_occurrences(mon, mon, Weekday::Tue), Some(0));
        assert_eq!(count_weekday_occurrences(mon, mon, Weekday::Mon), Some(1));
        assert_eq!(
            count_weekday_occurrences(date(2024, 3, 5), mon, Weekday::Mon),
            None
        );
        assert_eq!(
            count_weekday_occurrences(NaiveDate::MIN, NaiveDate::MIN, NaiveDate::MIN.weekday()),
            Some(1)
        );
        assert!(
            count_weekday_occurrences(NaiveDate::MIN, NaiveDate::MAX, Weekday::Fri).unwrap()
                > 1_000_000
        );
    }
}
