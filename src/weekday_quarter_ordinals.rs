//! Ordinals of a date's weekday occurrence in its Gregorian quarter.

use chrono::NaiveDate;

/// The 1-based occurrence of the date's weekday from the quarter start.
pub fn get_weekday_occurrence_in_quarter(date: NaiveDate) -> u32 {
    let first = crate::start_of_quarter(date);
    (date.signed_duration_since(first).num_days() / 7 + 1) as u32
}

/// Negative occurrence from the quarter end, with -1 meaning the last.
pub fn get_weekday_occurrence_from_end_of_quarter(date: NaiveDate) -> i8 {
    let last = crate::end_of_quarter(date);
    -((last.signed_duration_since(date).num_days() / 7 + 1) as i8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_weekdays_across_months_in_a_quarter() {
        assert_eq!(get_weekday_occurrence_in_quarter(date(2024, 1, 1)), 1);
        assert_eq!(get_weekday_occurrence_in_quarter(date(2024, 1, 8)), 2);
        assert_eq!(get_weekday_occurrence_in_quarter(date(2024, 3, 25)), 13);
        assert_eq!(get_weekday_occurrence_from_end_of_quarter(date(2024, 3, 25)), -1);
        assert_eq!(get_weekday_occurrence_from_end_of_quarter(date(2024, 3, 18)), -2);
        assert_eq!(get_weekday_occurrence_in_quarter(date(2024, 4, 1)), 1);
    }

    #[test]
    fn handles_leap_days_and_calendar_bounds() {
        assert_eq!(get_weekday_occurrence_in_quarter(date(2024, 2, 29)), 9);
        assert_eq!(get_weekday_occurrence_from_end_of_quarter(date(2024, 2, 29)), -5);
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!((1..=14).contains(&get_weekday_occurrence_in_quarter(edge)));
            assert!((-14..=-1).contains(&get_weekday_occurrence_from_end_of_quarter(edge)));
        }
    }
}
