//! Constant-time counts of weekdays and weekend days in date intervals.

use chrono::{Datelike, NaiveDate, Weekday};

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
        .filter(|offset| (monday_index + *offset) % 7 < 5)
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

/// Count occurrences of one weekday in an inclusive calendar interval.
///
/// Returns None for reversed intervals. The calculation is constant time.
pub fn count_weekday_occurrences_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    weekday: Weekday,
) -> Option<i64> {
    if start > end {
        return None;
    }
    let days = difference_in_calendar_days(end, start) + 1;
    let first_offset = (i64::from(weekday.num_days_from_monday())
        - i64::from(start.weekday().num_days_from_monday()))
    .rem_euclid(7);
    Some((days + 6 - first_offset) / 7)
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
            let weekends = crate::each_weekend_of_interval(start, end).unwrap().count() as i64;
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
        assert_eq!(
            count_business_days_in_interval(date(2024, 3, 2), friday),
            None
        );
        let expected = if crate::is_weekend(NaiveDate::MAX) {
            1
        } else {
            0
        };
        assert_eq!(
            count_weekend_days_in_interval(NaiveDate::MAX, NaiveDate::MAX),
            Some(expected)
        );
        let span = difference_in_calendar_days(NaiveDate::MAX, NaiveDate::MIN) + 1;
        let business = count_business_days_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        let weekends = count_weekend_days_in_interval(NaiveDate::MIN, NaiveDate::MAX).unwrap();
        assert_eq!(business + weekends, span);
    }
    #[test]
    fn weekday_counts_agree_with_lazy_iteration() {
        let start = date(2024, 2, 25);
        let weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        for days in 0..90 {
            let end = start + chrono::Duration::days(days);
            let total: i64 = weekdays
                .iter()
                .map(|weekday| {
                    let expected = crate::each_weekday_of_interval(start, end, *weekday)
                        .unwrap()
                        .count() as i64;
                    assert_eq!(
                        count_weekday_occurrences_in_interval(start, end, *weekday),
                        Some(expected)
                    );
                    expected
                })
                .sum();
            assert_eq!(total, days + 1);
        }
    }

    #[test]
    fn weekday_counts_validate_range_and_handle_extreme_dates() {
        let monday = date(2024, 1, 1);
        assert_eq!(
            count_weekday_occurrences_in_interval(monday, monday, Weekday::Mon),
            Some(1)
        );
        assert_eq!(
            count_weekday_occurrences_in_interval(monday, monday, Weekday::Tue),
            Some(0)
        );
        assert_eq!(
            count_weekday_occurrences_in_interval(NaiveDate::MAX, NaiveDate::MIN, Weekday::Mon),
            None
        );
        let whole_span = difference_in_calendar_days(NaiveDate::MAX, NaiveDate::MIN) + 1;
        let sum: i64 = weekdays_count_full_range().iter().sum();
        assert_eq!(sum, whole_span);
    }

    fn weekdays_count_full_range() -> [i64; 7] {
        let mut counts = [0; 7];
        for (i, weekday) in [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ]
        .iter()
        .enumerate()
        {
            counts[i] = count_weekday_occurrences_in_interval(
                NaiveDate::MIN,
                NaiveDate::MAX,
                *weekday,
            )
            .unwrap();
        }
        counts
    }

}
