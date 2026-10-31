//! Constant-time counts of weekdays and weekend days in date intervals.

use chrono::{Datelike, NaiveDate};

use crate::difference::difference_in_calendar_days;

/// Count Monday through Friday inside an inclusive date interval.
///
/// Saturdays and Sundays are excluded. Public holidays are not considered.
/// Returns None for reversed intervals and runs in constant time.
pub fn count_business_days_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    if start > end {
        return None;
    }

    let days = difference_in_calendar_days(end, start) + 1;
    let full_weeks = days / 7;
    let monday_index = i64::from(start.weekday().num_days_from_monday());
    let partial = (0..days % 7)
        .filter(|offset| (monday_index + offset) % 7 < 5)
        .count() as i64;
    Some(full_weeks * 5 + partial)
}

/// Count Saturdays and Sundays inside an inclusive date interval.
///
/// Returns None for reversed intervals.
pub fn count_weekend_days_in_interval(start: NaiveDate, end: NaiveDate) -> Option<i64> {
    let business_days = count_business_days_in_interval(start, end)?;
    Some(difference_in_calendar_days(end, start) + 1 - business_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn agrees_with_lazy_iterators_for_many_ranges() {
        let start = date(2024, 2, 25);
        for days in 0..60 {
            let end = start + chrono::Duration::days(days);
            let business = crate::each_business_day_of_interval(start, end)
                .unwrap()
                .count() as i64;
            let weekends = crate::each_weekend_of_interval(start, end)
                .unwrap()
                .count() as i64;
            assert_eq!(count_business_days_in_interval(start, end), Some(business));
            assert_eq!(count_weekend_days_in_interval(start, end), Some(weekends));
            assert_eq!(business + weekends, days + 1);
        }
    }

    #[test]
    fn rejects_reversed_ranges_and_handles_extremes() {
        let friday = date(2024, 3, 1);
        assert_eq!(count_business_days_in_interval(friday, friday), Some(1));
        assert_eq!(count_weekend_days_in_interval(friday, friday), Some(0));
        assert_eq!(count_business_days_in_interval(date(2024, 3, 2), friday), None);
        assert_eq!(count_weekend_days_in_interval(NaiveDate::MAX, NaiveDate::MAX), Some(0)
            .filter(|_| crate::is_weekend(NaiveDate::MAX) == false)
            .or_else(|| Some(1)));
        let span = difference_in_calendar_days(NaiveDate::MAX, NaiveDate::MIN) + 1;
        let business = count_business_days_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        let weekends = count_weekend_days_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(business + weekends, span);
    }
}
