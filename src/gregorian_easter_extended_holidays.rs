//! Gregorian Easter-derived Ascension and Pentecost dates.

use chrono::NaiveDate;

/// Ascension Day, 39 days after Gregorian Easter Sunday.
pub fn gregorian_ascension_day(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::gregorian_easter_sunday(year)?, 39)
}

/// Pentecost Sunday, 49 days after Gregorian Easter Sunday.
pub fn gregorian_pentecost_sunday(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::gregorian_easter_sunday(year)?, 49)
}

/// Pentecost Monday, 50 days after Gregorian Easter Sunday.
pub fn gregorian_pentecost_monday(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::gregorian_easter_sunday(year)?, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_movable_holidays_in_2024_and_2025() {
        assert_eq!(gregorian_ascension_day(2024), Some(date(2024, 5, 9)));
        assert_eq!(gregorian_pentecost_sunday(2024), Some(date(2024, 5, 19)));
        assert_eq!(gregorian_pentecost_monday(2024), Some(date(2024, 5, 20)));
        assert_eq!(gregorian_ascension_day(2025), Some(date(2025, 5, 29)));
        assert_eq!(gregorian_pentecost_sunday(2025), Some(date(2025, 6, 8)));
        assert_eq!(gregorian_pentecost_monday(2025), Some(date(2025, 6, 9)));
    }

    #[test]
    fn rejects_unsupported_years() {
        for year in [0, 1582, i32::MAX] {
            assert_eq!(gregorian_ascension_day(year), None);
            assert_eq!(gregorian_pentecost_sunday(year), None);
            assert_eq!(gregorian_pentecost_monday(year), None);
        }
    }
}
