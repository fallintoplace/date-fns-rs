//! Western Gregorian Easter Sunday for civil holiday calendars.

use chrono::NaiveDate;

/// Calculate Western Easter Sunday using the Gregorian computus.
///
/// Valid for Gregorian Easter years 1583 through 4099. Returns None for
/// unsupported years rather than silently applying the Julian algorithm.
pub fn gregorian_easter_sunday(year: i32) -> Option<NaiveDate> {
    if !(1583..=4099).contains(&year) {
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
    let value = h + l - 7 * m + 114;
    let month = u32::try_from(value / 31).ok()?;
    let day = u32::try_from(value % 31 + 1).ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_known_early_and_late_easters() {
        assert_eq!(gregorian_easter_sunday(2024), Some(date(2024, 3, 31)));
        assert_eq!(gregorian_easter_sunday(2025), Some(date(2025, 4, 20)));
        assert_eq!(gregorian_easter_sunday(2000), Some(date(2000, 4, 23)));
        assert_eq!(gregorian_easter_sunday(2038), Some(date(2038, 4, 25)));
        assert_eq!(gregorian_easter_sunday(1818), Some(date(1818, 3, 22)));
    }

    #[test]
    fn rejects_years_outside_supported_gregorian_range() {
        assert_eq!(gregorian_easter_sunday(1582), None);
        assert_eq!(gregorian_easter_sunday(4100), None);
        assert_eq!(gregorian_easter_sunday(i32::MAX), None);
    }
}
