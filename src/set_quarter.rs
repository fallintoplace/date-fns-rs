//! Checked calendar-quarter replacement preserving month position.

use chrono::{Datelike, NaiveDate};

use crate::set_month::set_month;

/// Set the calendar quarter while keeping the month position in the quarter.
///
/// For example, January 31 set to quarter two becomes April 30.
/// The day clamps in shorter months. Invalid quarters return None.
pub fn set_quarter(date: NaiveDate, quarter: u32) -> Option<NaiveDate> {
    if !(1..=4).contains(&quarter) {
        return None;
    }
    let month_in_quarter = (date.month() - 1) % 3;
    let new_month = (quarter - 1) * 3 + month_in_quarter + 1;
    set_month(date, new_month)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn preserves_month_position_and_clamps_shorter_months() {
        assert_eq!(set_quarter(date(2024, 1, 31), 2), Some(date(2024, 4, 30)));
        assert_eq!(set_quarter(date(2024, 2, 29), 4), Some(date(2024, 11, 29)));
        assert_eq!(set_quarter(date(2023, 11, 30), 1), Some(date(2023, 2, 28)));
        assert_eq!(set_quarter(date(2024, 3, 31), 2), Some(date(2024, 6, 30)));
    }

    #[test]
    fn rejects_invalid_quarters_and_preserves_same_quarter() {
        let start = date(2024, 5, 31);
        assert_eq!(set_quarter(start, 2), Some(start));
        assert_eq!(set_quarter(start, 0), None);
        assert_eq!(set_quarter(start, 5), None);
        assert_eq!(set_quarter(start, u32::MAX), None);
        assert_eq!(set_quarter(NaiveDate::MAX, 4), Some(NaiveDate::MAX));
    }
}
