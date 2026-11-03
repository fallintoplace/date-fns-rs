//! Relative civil holidays around Orthodox Easter Sunday.

use chrono::{Days, NaiveDate};

/// Return the Friday two days before Orthodox Easter Sunday.
///
/// Returns None when the Easter date is unsupported or subtraction overflows.
pub fn orthodox_good_friday(year: i32) -> Option<NaiveDate> {
    crate::orthodox_easter_sunday(year)?.checked_sub_days(Days::new(2))
}

/// Return the Monday after Orthodox Easter Sunday.
///
/// Returns None when the Easter date is unsupported or addition overflows.
pub fn orthodox_easter_monday(year: i32) -> Option<NaiveDate> {
    crate::orthodox_easter_sunday(year)?.checked_add_days(Days::new(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Weekday};

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn matches_known_holiday_dates_and_weekdays() {
        for (year, friday_month, friday_day, monday_month, monday_day) in [
            (2024, 5, 3, 5, 6),
            (2025, 4, 18, 4, 21),
            (2026, 4, 10, 4, 13),
        ] {
            let friday = orthodox_good_friday(year).unwrap();
            let monday = orthodox_easter_monday(year).unwrap();
            assert_eq!(friday, date(year, friday_month, friday_day));
            assert_eq!(monday, date(year, monday_month, monday_day));
            assert_eq!(friday.weekday(), Weekday::Fri);
            assert_eq!(monday.weekday(), Weekday::Mon);
            assert_eq!(
                friday.succ_opt().and_then(|d| d.succ_opt()),
                crate::orthodox_easter_sunday(year)
            );
        }
    }

    #[test]
    fn preserves_the_base_computus_range_contract() {
        for year in [0, 1582, i32::MAX] {
            assert_eq!(orthodox_good_friday(year), None);
            assert_eq!(orthodox_easter_monday(year), None);
        }
    }
}
