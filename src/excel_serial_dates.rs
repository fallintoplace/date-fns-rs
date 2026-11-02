//! Whole-day serial date conversions for the Excel 1900 and 1904 systems.

use chrono::{Days, NaiveDate};

fn epoch_1900() -> NaiveDate {
    NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
}

fn epoch_1904() -> NaiveDate {
    NaiveDate::from_ymd_opt(1904, 1, 1).unwrap()
}

/// Convert a date to an Excel 1900-system serial.
/// Serial 1 is 1900-01-01. Excel incorrectly assigns serial 60 to 1900-02-29;
/// serial 61 is 1900-03-01. Dates before 1900-01-01 are not supported.
pub fn to_excel_1900_serial(date: NaiveDate) -> Option<i64> {
    let days = date.signed_duration_since(epoch_1900()).num_days();
    if days < 0 {
        return None;
    }
    let first = days.checked_add(1)?;
    if date >= NaiveDate::from_ymd_opt(1900, 3, 1).unwrap() {
        first.checked_add(1)
    } else {
        Some(first)
    }
}

/// Convert an Excel 1900-system serial to a real Gregorian date.
/// The nonexistent 1900-02-29 (serial 60) is rejected.
pub fn from_excel_1900_serial(serial: i64) -> Option<NaiveDate> {
    if serial <= 0 || serial == 60 {
        return None;
    }
    let offset = if serial < 60 { serial - 1 } else { serial - 2 };
    epoch_1900().checked_add_days(Days::new(u64::try_from(offset).ok()?))
}

/// Convert a date to an Excel 1904-system serial.
/// Serial 0 is 1904-01-01. Negative serials are not supported.
pub fn to_excel_1904_serial(date: NaiveDate) -> Option<i64> {
    let days = date.signed_duration_since(epoch_1904()).num_days();
    (days >= 0).then_some(days)
}

/// Convert a nonnegative Excel 1904-system serial to a date.
pub fn from_excel_1904_serial(serial: i64) -> Option<NaiveDate> {
    epoch_1904().checked_add_days(Days::new(u64::try_from(serial).ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_the_1900_leap_year_bug_without_inventing_a_date() {
        assert_eq!(to_excel_1900_serial(date(1900, 1, 1)), Some(1));
        assert_eq!(to_excel_1900_serial(date(1900, 2, 28)), Some(59));
        assert_eq!(to_excel_1900_serial(date(1900, 3, 1)), Some(61));
        assert_eq!(from_excel_1900_serial(59), Some(date(1900, 2, 28)));
        assert_eq!(from_excel_1900_serial(60), None);
        assert_eq!(from_excel_1900_serial(61), Some(date(1900, 3, 1)));
        assert_eq!(from_excel_1900_serial(0), None);
        assert_eq!(from_excel_1900_serial(-1), None);
        assert_eq!(to_excel_1900_serial(date(1899, 12, 31)), None);
    }

    #[test]
    fn supports_both_epochs_and_their_1462_day_offset() {
        assert_eq!(to_excel_1904_serial(date(1904, 1, 1)), Some(0));
        assert_eq!(to_excel_1900_serial(date(1904, 1, 1)), Some(1462));
        assert_eq!(from_excel_1904_serial(0), Some(date(1904, 1, 1)));
        assert_eq!(to_excel_1904_serial(date(1903, 12, 31)), None);
        assert_eq!(from_excel_1904_serial(-1), None);
    }

    #[test]
    fn round_trips_leap_dates_and_checks_overflow() {
        for day in [date(1900, 1, 1), date(1900, 3, 1), date(2000, 2, 29), NaiveDate::MAX] {
            assert_eq!(from_excel_1900_serial(to_excel_1900_serial(day).unwrap()), Some(day));
        }
        for day in [date(1904, 1, 1), date(2024, 2, 29), NaiveDate::MAX] {
            assert_eq!(from_excel_1904_serial(to_excel_1904_serial(day).unwrap()), Some(day));
        }
        assert_eq!(from_excel_1900_serial(i64::MAX), None);
        assert_eq!(from_excel_1904_serial(i64::MAX), None);
    }
}
