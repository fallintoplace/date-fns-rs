//! Orthodox Easter-derived Ascension and Pentecost dates in the civil calendar.

use chrono::NaiveDate;

/// Orthodox Ascension Day, 39 days after Orthodox Easter Sunday.
pub fn orthodox_ascension_day(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::orthodox_easter_sunday(year)?, 39)
}

/// Orthodox Pentecost Sunday, 49 days after Orthodox Easter Sunday.
pub fn orthodox_pentecost_sunday(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::orthodox_easter_sunday(year)?, 49)
}

/// Orthodox Pentecost Monday, 50 days after Orthodox Easter Sunday.
pub fn orthodox_pentecost_monday(year: i32) -> Option<NaiveDate> {
    crate::add_days(crate::orthodox_easter_sunday(year)?, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_known_orthodox_movable_holidays() {
        assert_eq!(orthodox_ascension_day(2024), Some(date(2024, 6, 13)));
        assert_eq!(orthodox_pentecost_sunday(2024), Some(date(2024, 6, 23)));
        assert_eq!(orthodox_pentecost_monday(2024), Some(date(2024, 6, 24)));
        assert_eq!(orthodox_ascension_day(2025), Some(date(2025, 5, 29)));
        assert_eq!(orthodox_pentecost_sunday(2025), Some(date(2025, 6, 8)));
        assert_eq!(orthodox_pentecost_monday(2025), Some(date(2025, 6, 9)));
    }

    #[test]
    fn rejects_unsupported_years() {
        for year in [0, 1582, i32::MAX] {
            assert_eq!(orthodox_ascension_day(year), None);
            assert_eq!(orthodox_pentecost_sunday(year), None);
            assert_eq!(orthodox_pentecost_monday(year), None);
        }
    }
}
