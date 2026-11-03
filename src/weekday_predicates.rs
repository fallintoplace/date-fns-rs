//! Named weekday predicates for calendar dates.

use chrono::{Datelike, NaiveDate, Weekday};

/// Return whether the date falls on Monday.
pub fn is_monday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Mon
}

/// Return whether the date falls on Tuesday.
pub fn is_tuesday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Tue
}

/// Return whether the date falls on Wednesday.
pub fn is_wednesday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Wed
}

/// Return whether the date falls on Thursday.
pub fn is_thursday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Thu
}

/// Return whether the date falls on Friday.
pub fn is_friday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Fri
}

/// Return whether the date falls on Saturday.
pub fn is_saturday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Sat
}

/// Return whether the date falls on Sunday.
pub fn is_sunday(date: NaiveDate) -> bool {
    date.weekday() == Weekday::Sun
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_one_predicate_matches_each_day_of_a_week() {
        for day in 4..=10 {
            let date = NaiveDate::from_ymd_opt(2024, 3, day).unwrap();
            let matches = [
                is_monday(date),
                is_tuesday(date),
                is_wednesday(date),
                is_thursday(date),
                is_friday(date),
                is_saturday(date),
                is_sunday(date),
            ];
            assert_eq!(matches.iter().filter(|value| **value).count(), 1);
            assert!(matches[(day - 4) as usize]);
        }
    }

    #[test]
    fn weekday_predicates_work_at_chrono_limits() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            let matches = [
                is_monday(date),
                is_tuesday(date),
                is_wednesday(date),
                is_thursday(date),
                is_friday(date),
                is_saturday(date),
                is_sunday(date),
            ];
            assert_eq!(matches.iter().filter(|value| **value).count(), 1);
        }
    }
}
