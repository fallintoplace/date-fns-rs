//! Inclusive and nearest weekday selection for naive datetimes.

use chrono::{NaiveDateTime, Weekday};

/// Find the next weekday, including the starting date if it matches.
pub fn next_or_same_day_datetime(value: NaiveDateTime, weekday: Weekday) -> Option<NaiveDateTime> {
    Some(crate::next_or_same_weekday(value.date(), weekday)?.and_time(value.time()))
}

/// Find the previous weekday, including the starting date if it matches.
pub fn previous_or_same_day_datetime(
    value: NaiveDateTime,
    weekday: Weekday,
) -> Option<NaiveDateTime> {
    Some(crate::previous_or_same_weekday(value.date(), weekday)?.and_time(value.time()))
}

/// Find the nearest occurrence of a weekday, keeping the clock time.
pub fn nearest_day_datetime(value: NaiveDateTime, weekday: Weekday) -> Option<NaiveDateTime> {
    Some(crate::nearest_weekday(value.date(), weekday)?.and_time(value.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};

    fn at(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_456_789)
            .unwrap()
    }

    #[test]
    fn navigates_inclusively_and_preserves_nanoseconds() {
        let friday = at(2024, 3, 1);
        assert_eq!(
            next_or_same_day_datetime(friday, Weekday::Fri),
            Some(friday)
        );
        assert_eq!(
            previous_or_same_day_datetime(friday, Weekday::Fri),
            Some(friday)
        );
        assert_eq!(
            next_or_same_day_datetime(friday, Weekday::Mon),
            Some(at(2024, 3, 4))
        );
        assert_eq!(
            previous_or_same_day_datetime(friday, Weekday::Mon),
            Some(at(2024, 2, 26))
        );
        assert_eq!(
            nearest_day_datetime(friday, Weekday::Mon),
            Some(at(2024, 3, 4))
        );
    }

    #[test]
    fn returns_identity_at_chrono_limits() {
        for edge in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            let weekday = edge.date().weekday();
            assert_eq!(next_or_same_day_datetime(edge, weekday), Some(edge));
            assert_eq!(previous_or_same_day_datetime(edge, weekday), Some(edge));
            assert_eq!(nearest_day_datetime(edge, weekday), Some(edge));
        }
    }
}
