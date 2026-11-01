//! First and last Monday-Friday dates in calendar months.

use chrono::NaiveDate;

use crate::boundaries::{end_of_month, start_of_month};
use crate::business_days::is_business_day;

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
}
