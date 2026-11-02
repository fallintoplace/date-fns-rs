//! Easter Monday dates derived from the supported Easter computus methods.

use chrono::NaiveDate;

/// Calculate Western Easter Monday in the Gregorian civil calendar.
///
/// Supported years match the Gregorian Easter Sunday helper.
pub fn gregorian_easter_monday(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::gregorian_easter_sunday(year)?, 1)
}

/// Calculate Orthodox Easter Monday in the Gregorian civil calendar.
///
/// Uses the Julian ecclesiastical Easter rule before adding one day.
pub fn orthodox_easter_monday(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::orthodox_easter_sunday(year)?, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn distinguishes_western_and_orthodox_holidays() {
        assert_eq!(gregorian_easter_monday(2024), Some(date(2024, 4, 1)));
        assert_eq!(orthodox_easter_monday(2024), Some(date(2024, 5, 6)));
        assert_eq!(gregorian_easter_monday(2025), Some(date(2025, 4, 21)));
        assert_eq!(orthodox_easter_monday(2025), Some(date(2025, 4, 21)));
    }

    #[test]
    fn reuses_supported_years_and_checked_date_arithmetic() {
        assert_eq!(gregorian_easter_monday(1582), None);
        assert_eq!(orthodox_easter_monday(1582), None);
        assert_eq!(gregorian_easter_monday(i32::MAX), None);
        assert_eq!(orthodox_easter_monday(i32::MAX), None);
    }
}
