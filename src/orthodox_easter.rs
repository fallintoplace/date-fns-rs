//! Orthodox Easter based on the Julian ecclesiastical computus.

use chrono::NaiveDate;

/// Calculate Julian-calendar Easter Sunday as a Gregorian civil date.
///
/// Uses the traditional Julian paschalion, not the Gregorian computus.
/// Supports years from 1583 onward within Chrono's Gregorian date range.
pub fn orthodox_easter_sunday(year: i32) -> Option<NaiveDate> {
    if year < 1583 {
        return None;
    }
    let year = i64::from(year);
    let a = year % 4;
    let b = year % 7;
    let c = year % 19;
    let d = (19 * c + 15) % 30;
    let e = (2 * a + 4 * b - d + 34).rem_euclid(7);
    let day_index = d + e + 114;
    let julian_month = day_index / 31;
    let julian_day = day_index % 31 + 1;

    // Convert a Julian calendar civil date to its absolute Julian Day Number.
    let adjustment = (14 - julian_month) / 12;
    let y = year + 4800 - adjustment;
    let m = julian_month + 12 * adjustment - 3;
    let jdn = julian_day + (153 * m + 2) / 5 + 365 * y + y / 4 - 32_083;
    crate::from_julian_day(jdn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Weekday};

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_known_orthodox_easter_dates() {
        for (year, month, day) in [
            (1900, 4, 22),
            (2000, 4, 30),
            (2024, 5, 5),
            (2025, 4, 20),
            (2026, 4, 12),
            (2027, 5, 2),
        ] {
            let result = orthodox_easter_sunday(year).unwrap();
            assert_eq!(result, date(year, month, day));
            assert_eq!(result.weekday(), Weekday::Sun);
        }
    }

    #[test]
    fn differs_from_gregorian_easter_in_2024_but_agrees_in_2025() {
        assert_ne!(orthodox_easter_sunday(2024), crate::gregorian_easter_sunday(2024));
        assert_eq!(orthodox_easter_sunday(2025), crate::gregorian_easter_sunday(2025));
    }

    #[test]
    fn rejects_unsupported_eras_and_year_ranges() {
        assert_eq!(orthodox_easter_sunday(1582), None);
        assert_eq!(orthodox_easter_sunday(0), None);
        assert_eq!(orthodox_easter_sunday(i32::MAX), None);
    }
}
