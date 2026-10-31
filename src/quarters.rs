//! Quarter numbering and inclusive calendar boundaries.

use chrono::{Datelike, NaiveDate};

use crate::boundaries::end_of_month;

/// Return the quarter number in 1..=4.
pub fn get_quarter(date: NaiveDate) -> u32 {
    (date.month() - 1) / 3 + 1
}

/// Return the first day of the quarter containing the date.
pub fn start_of_quarter(date: NaiveDate) -> NaiveDate {
    let month = (get_quarter(date) - 1) * 3 + 1;
    NaiveDate::from_ymd_opt(date.year(), month, 1)
        .expect("quarter start of a valid date must exist")
}

/// Return the last day of the quarter containing the date.
pub fn end_of_quarter(date: NaiveDate) -> NaiveDate {
    let month = get_quarter(date) * 3;
    let first = NaiveDate::from_ymd_opt(date.year(), month, 1)
        .expect("quarter end month of a valid date must exist");
    end_of_month(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn quarter_labels_follow_calendar_months() {
        for month in 1..=12 {
            assert_eq!(get_quarter(date(2024, month, 1)), (month - 1) / 3 + 1);
        }
    }

    #[test]
    fn boundaries_include_the_complete_quarter() {
        assert_eq!(start_of_quarter(date(2024, 3, 31)), date(2024, 1, 1));
        assert_eq!(end_of_quarter(date(2024, 2, 29)), date(2024, 3, 31));
        assert_eq!(start_of_quarter(date(2024, 5, 31)), date(2024, 4, 1));
        assert_eq!(end_of_quarter(date(2024, 5, 31)), date(2024, 6, 30));
        assert_eq!(end_of_quarter(date(2024, 8, 15)), date(2024, 9, 30));
        assert_eq!(end_of_quarter(date(2024, 12, 31)), date(2024, 12, 31));
    }

    #[test]
    fn quarter_boundaries_work_at_date_limits() {
        assert!(start_of_quarter(NaiveDate::MIN) <= NaiveDate::MIN);
        assert!(end_of_quarter(NaiveDate::MAX) >= NaiveDate::MAX);
    }
}
