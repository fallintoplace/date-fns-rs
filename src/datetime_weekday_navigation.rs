//! Next and previous weekday navigation preserving naive clock precision.
use chrono::{NaiveDateTime, Weekday};

/// Return the next `weekday` strictly after `value`.
///
/// Matching weekdays advance seven days. Clock and nanoseconds are preserved.
pub fn next_day_datetime(value: NaiveDateTime, weekday: Weekday) -> Option<NaiveDateTime> {
    Some(crate::next_weekday(value.date(), weekday)?.and_time(value.time()))
}

/// Return the previous `weekday` strictly before `value`.
///
/// Matching weekdays retreat seven days. Returns None on date overflow.
pub fn previous_day_datetime(value: NaiveDateTime, weekday: Weekday) -> Option<NaiveDateTime> {
    Some(crate::previous_weekday(value.date(), weekday)?.and_time(value.time()))
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
    fn follows_date_fns_weekday_navigation_examples() {
        assert_eq!(
            next_day_datetime(at(2020, 3, 20), Weekday::Mon),
            Some(at(2020, 3, 23))
        );
        assert_eq!(
            previous_day_datetime(at(2020, 3, 23), Weekday::Fri),
            Some(at(2020, 3, 20))
        );
        assert_eq!(
            next_day_datetime(at(2020, 3, 23), Weekday::Mon),
            Some(at(2020, 3, 30))
        );
    }

    #[test]
    fn retains_nanoseconds_across_leap_days() {
        let start = at(2024, 2, 28);
        assert_eq!(
            next_day_datetime(start, Weekday::Thu),
            Some(at(2024, 2, 29)
        );
        assert_eq!(
            previous_day_datetime(start, Weekday::Tue),
            Some(at(2024, 2, 27)
        );
    }

    #[test]
    fn rejects_steps_beyond_supported_calendar_bounds() {
        let max = NaiveDateTime::MAX;
        let min = NaiveDateTime::MIN;
        assert_eq!(next_day_datetime(max, max.date().weekday()), None);
        assert_eq!(previous_day_datetime(min, min.date().weekday()), None);
    }
}
