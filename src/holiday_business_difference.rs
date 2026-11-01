//! Signed business-day differences with a user-defined holiday calendar.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::business_days::is_business_day;
use crate::business_difference::difference_in_business_days;

/// Count business days in (right, left] excluding supplied holidays.
///
/// Negative differences use the reverse half-open range. Repeated holidays
/// count once, weekend holidays have no effect, and the right endpoint is
/// excluded. Work scales with the number of supplied holiday dates.
pub fn difference_in_business_days_excluding_holidays(
    left: NaiveDate,
    right: NaiveDate,
    holidays: &[NaiveDate],
) -> i64 {
    let base = difference_in_business_days(left, right);
    if holidays.is_empty() || left == right {
        return base;
    }

    let (start, end, sign) = if left > right {
        (right, left, 1)
    } else {
        (left, right, -1)
    };
    let holidays_in_range = holidays
        .iter()
        .copied()
        .filter(|day| *day > start && *day <= end && is_business_day(*day))
        .collect::<BTreeSet<_>>()
        .len() as i64;
    base - sign * holidays_in_range
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn excludes_unique_weekday_holidays_but_not_weekends() {
        let start = date(2024, 3, 1);
        let end = date(2024, 3, 8);
        let holidays = [
            date(2024, 3, 1),
            date(2024, 3, 2),
            date(2024, 3, 4),
            date(2024, 3, 4),
            date(2024, 3, 8),
        ];
        assert_eq!(
            difference_in_business_days_excluding_holidays(end, start, &holidays),
            3
        );
        assert_eq!(
            difference_in_business_days_excluding_holidays(start, end, &holidays),
            -3
        );
    }

    #[test]
    fn handles_empty_holidays_equal_dates_and_calendar_limits() {
        let start = date(2024, 2, 28);
        let end = date(2024, 3, 1);
        assert_eq!(
            difference_in_business_days_excluding_holidays(end, start, &[]),
            difference_in_business_days(end, start)
        );
        assert_eq!(
            difference_in_business_days_excluding_holidays(start, start, &[start]),
            0
        );
        assert_eq!(
            difference_in_business_days_excluding_holidays(NaiveDate::MAX, NaiveDate::MIN, &[]),
            difference_in_business_days(NaiveDate::MAX, NaiveDate::MIN)
        );
    }
}
