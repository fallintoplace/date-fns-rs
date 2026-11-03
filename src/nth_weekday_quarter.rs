//! Select a numbered weekday within a calendar quarter.

use chrono::{Datelike, Days, NaiveDate, Weekday};

/// Find a numbered occurrence of a weekday in a calendar quarter.
///
/// Positive occurrences count from the beginning, and negative ones
/// count backward from the end. Zero and missing occurrences return None.
pub fn nth_weekday_of_quarter(
    year: i32,
    quarter: u32,
    weekday: Weekday,
    occurrence: i16,
) -> Option<NaiveDate> {
    if !(1..=4).contains(&quarter) || occurrence == 0 {
        return None;
    }

    let first = NaiveDate::from_ymd_opt(year, (quarter - 1) * 3 + 1, 1)?;
    let last = crate::end_of_quarter(first);
    let target = i64::from(weekday.num_days_from_monday());
    let weeks = i64::from(occurrence).abs() - 1;

    let date = if occurrence > 0 {
        let current = i64::from(first.weekday().num_days_from_monday());
        let offset = (target - current).rem_euclid(7) + weeks * 7;
        first.checked_add_days(Days::new(u64::try_from(offset).ok()?))?
    } else {
        let current = i64::from(last.weekday().num_days_from_monday());
        let offset = (current - target).rem_euclid(7) + weeks * 7;
        last.checked_sub_days(Days::new(u64::try_from(offset).ok()?))?
    };

    (first..=last).contains(&date).then_some(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn selects_first_and_last_occurrences() {
        assert_eq!(
            nth_weekday_of_quarter(2024, 1, Weekday::Thu, 9),
            Some(day(2024, 2, 29))
        );
        assert_eq!(
            nth_weekday_of_quarter(2024, 1, Weekday::Sun, -1),
            Some(day(2024, 3, 31))
        );
        assert_eq!(
            nth_weekday_of_quarter(2024, 4, Weekday::Tue, -1),
            Some(day(2024, 12, 31))
        );
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Thu, 14), None);
    }

    #[test]
    fn rejects_invalid_periods_and_unrepresentable_dates() {
        assert_eq!(nth_weekday_of_quarter(2024, 0, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_quarter(2024, 5, Weekday::Mon, 1), None);
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Mon, 0), None);
        assert_eq!(nth_weekday_of_quarter(2024, 1, Weekday::Mon, i16::MIN), None);
        assert_eq!(nth_weekday_of_quarter(i32::MAX, 1, Weekday::Mon, 1), None);
    }
}
