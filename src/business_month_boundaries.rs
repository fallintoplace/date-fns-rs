//! First and last Monday-Friday dates in calendar months.

use chrono::NaiveDate;

use crate::boundaries::{end_of_month, start_of_month};
use crate::business_days::is_business_day;
use crate::business_holidays::is_business_day_with_holidays;

/// Return the first Monday-Friday date of the month containing date.
pub fn first_business_day_of_month(date: NaiveDate) -> NaiveDate {
    let mut current = start_of_month(date);
    while !is_business_day(current) {
        current = current
            .succ_opt()
            .expect("every month has a weekday after its start");
    }
    current
}

/// Return the last Monday-Friday date of the month containing date.
pub fn last_business_day_of_month(date: NaiveDate) -> NaiveDate {
    let mut current = end_of_month(date);
    while !is_business_day(current) {
        current = current
            .pred_opt()
            .expect("every month has a weekday before its end");
    }
    current
}

/// Find the first weekday that is not a supplied holiday in the date's month.
///
/// Returns None when every weekday of the month is excluded.
pub fn first_business_day_of_month_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    let mut current = start_of_month(date);
    let last = end_of_month(date);
    loop {
        if is_business_day_with_holidays(current, holidays) {
            return Some(current);
        }
        if current == last {
            return None;
        }
        current = current.succ_opt()?;
    }
}

/// Find the last weekday that is not a supplied holiday in the date's month.
///
/// Returns None when every weekday of the month is excluded.
pub fn last_business_day_of_month_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    let mut current = end_of_month(date);
    let first = start_of_month(date);
    loop {
        if is_business_day_with_holidays(current, holidays) {
            return Some(current);
        }
        if current == first {
            return None;
        }
        current = current.pred_opt()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn skips_weekends_at_both_month_boundaries() {
        let june = date(2024, 6, 15);
        assert_eq!(first_business_day_of_month(june), date(2024, 6, 3));
        assert_eq!(last_business_day_of_month(june), date(2024, 6, 28));
        let february = date(2024, 2, 29);
        assert_eq!(first_business_day_of_month(february), date(2024, 2, 1));
        assert_eq!(last_business_day_of_month(february), date(2024, 2, 29));
    }

    #[test]
    fn stays_within_supported_dates_at_chrono_limits() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!(first_business_day_of_month(edge) >= start_of_month(edge));
            assert!(last_business_day_of_month(edge) <= end_of_month(edge));
            assert!(is_business_day(first_business_day_of_month(edge)));
            assert!(is_business_day(last_business_day_of_month(edge)));
        }
    }
    #[test]
    fn holiday_aware_boundaries_can_skip_or_exhaust_the_month() {
        let march = date(2024, 3, 15);
        let holidays = [
            date(2024, 3, 1),
            date(2024, 3, 4),
            date(2024, 3, 28),
            date(2024, 3, 29),
            date(2024, 3, 29),
        ];
        assert_eq!(
            first_business_day_of_month_with_holidays(march, &holidays),
            Some(date(2024, 3, 5))
        );
        assert_eq!(
            last_business_day_of_month_with_holidays(march, &holidays),
            Some(date(2024, 3, 27))
        );

        let all_weekdays: Vec<_> =
            crate::each_business_day_of_interval(date(2024, 3, 1), date(2024, 3, 31))
                .unwrap()
                .collect();
        assert_eq!(first_business_day_of_month_with_holidays(march, &all_weekdays), None);
        assert_eq!(last_business_day_of_month_with_holidays(march, &all_weekdays), None);
        assert_eq!(
            first_business_day_of_month_with_holidays(march, &[]),
            Some(first_business_day_of_month(march))
        );
        assert_eq!(
            last_business_day_of_month_with_holidays(march, &[]),
            Some(last_business_day_of_month(march))
        );
    }

}
