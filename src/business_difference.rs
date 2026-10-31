//! Signed counts of business days in half-open calendar intervals.

use chrono::{Datelike, NaiveDate};

use crate::difference::difference_in_calendar_days;

/// Count weekdays in the interval (right, left] for left >= right.
///
/// For reversed dates, negate the count in (left, right]. Saturdays and
/// Sundays are excluded. This function does not account for public holidays.
/// The complexity is constant even for dates thousands of years apart.
pub fn difference_in_business_days(left: NaiveDate, right: NaiveDate) -> i64 {
    let (start, end, sign) = if left >= right {
        (right, left, 1)
    } else {
        (left, right, -1)
    };

    let calendar_days = difference_in_calendar_days(end, start);
    let mut count = (calendar_days / 7) * 5;
    let start_weekday = i64::from(start.weekday().num_days_from_monday());

    for offset in 1..=(calendar_days % 7) {
        if (start_weekday + offset) % 7 < 5 {
            count += 1;
        }
    }

    count * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_weekdays_and_excludes_start() {
        assert_eq!(
            difference_in_business_days(date(2024, 3, 4), date(2024, 3, 1)),
            1
        );
        assert_eq!(
            difference_in_business_days(date(2024, 3, 3), date(2024, 3, 1)),
            0
        );
        assert_eq!(
            difference_in_business_days(date(2024, 3, 8), date(2024, 3, 1)),
            5
        );
        assert_eq!(
            difference_in_business_days(date(2024, 3, 1), date(2024, 3, 1)),
            0
        );
    }

    #[test]
    fn counts_partial_and_complete_weeks() {
        assert_eq!(
            difference_in_business_days(date(2024, 3, 12), date(2024, 3, 1)),
            7
        );
        assert_eq!(
            difference_in_business_days(date(2024, 3, 7), date(2024, 3, 3)),
            4
        );
        assert_eq!(
            difference_in_business_days(date(2024, 2, 29), date(2024, 2, 26)),
            3
        );
    }

    #[test]
    fn signed_difference_is_antisymmetric_even_at_limits() {
        let first = NaiveDate::MIN;
        let last = NaiveDate::MAX;
        let forward = difference_in_business_days(last, first);
        assert!(forward > 0);
        assert_eq!(difference_in_business_days(first, last), -forward);
    }
}
