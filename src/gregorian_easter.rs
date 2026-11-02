//! Computus for Gregorian Easter Sunday in the civil calendar.

use chrono::NaiveDate;

/// Calculate Easter Sunday using the Gregorian computus.
///
/// Supports Gregorian calendar years from 1583 onward. Returns None for
/// earlier years and years outside Chrono's representable date range.
pub fn gregorian_easter_sunday(year: i32) -> Option<NaiveDate> {
    if year < 1583 {
        return None;
    }
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k).rem_euclid(7);
    let m = (a + 11 * h + 22 * l) / 451;
    let day_index = h + l - 7 * m + 114;
    let month = u32::try_from(day_index / 31).ok()?;
    let day = u32::try_from(day_index % 31 + 1).ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Weekday};

    #[test]
    fn matches_historic_and_modern_gregorian_easters() {
        for (year, month, day) in [
            (1818, 3, 22),
            (1954, 4, 18),
            (2000, 4, 23),
            (2024, 3, 31),
            (2025, 4, 20),
            (2038, 4, 25),
        ] {
            let easter = gregorian_easter_sunday(year).unwrap();
            assert_eq!((easter.year(), easter.month(), easter.day()), (year, month, day));
            assert_eq!(easter.weekday(), Weekday::Sun);
        }
    }

    #[test]
    fn rejects_julian_era_and_out_of_range_years() {
        assert_eq!(gregorian_easter_sunday(1582), None);
        assert_eq!(gregorian_easter_sunday(0), None);
        assert_eq!(gregorian_easter_sunday(i32::MAX), None);
    }
}
