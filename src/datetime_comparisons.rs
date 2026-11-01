//! Calendar and clock comparisons of naive date-times.

use chrono::{Datelike, NaiveDateTime, Timelike};

/// Whether two values occur on the same calendar date.
pub fn is_same_day_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    left.date() == right.date()
}

/// Whether two values are in the same calendar month.
pub fn is_same_month_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    left.year() == right.year() && left.month() == right.month()
}

/// Whether two values are in the same calendar year.
pub fn is_same_year_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    left.year() == right.year()
}

/// Whether the values fall in the same calendar hour.
pub fn is_same_hour_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    is_same_day_datetime(left, right) && left.hour() == right.hour()
}

/// Whether the values fall in the same calendar minute.
pub fn is_same_minute_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    is_same_hour_datetime(left, right) && left.minute() == right.minute()
}

/// Whether the values fall in the same calendar second.
pub fn is_same_second_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    is_same_minute_datetime(left, right) && left.second() == right.second()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn ignores_smaller_units_but_not_larger_ones() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let first = date.and_hms_milli_opt(23, 59, 59, 1).unwrap();
        let second = date.and_hms_milli_opt(23, 59, 59, 999).unwrap();
        assert!(is_same_second_datetime(first, second));
        assert!(is_same_minute_datetime(first, second));
        assert!(is_same_hour_datetime(first, second));
        assert!(is_same_day_datetime(first, second));
        assert!(is_same_month_datetime(first, second));
        assert!(is_same_year_datetime(first, second));

        let next_day = NaiveDate::from_ymd_opt(2024, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(!is_same_day_datetime(first, next_day));
        assert!(!is_same_month_datetime(first, next_day));
        assert!(is_same_year_datetime(first, next_day));
        assert!(!is_same_second_datetime(first, next_day));
    }
}
