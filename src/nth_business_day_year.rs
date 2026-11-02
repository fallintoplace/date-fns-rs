//! One-based working-day positions within a calendar year.

use chrono::{Datelike, NaiveDate};

/// Return the nth Monday-Friday date of the year containing `date`.
///
/// Ordinals are 1-based. Returns None for zero or values beyond the year.
pub fn nth_business_day_of_year(date: NaiveDate, ordinal: u32) -> Option<NaiveDate> {
    nth_business_day_of_year_with_holidays(date, ordinal, &[])
}

/// Find the nth weekday of a year excluding caller-supplied holidays.
///
/// Weekend and duplicate holiday entries do not affect the result.
/// Counting includes January 1 when it is a working day. Queries use the
/// indexed holiday calendar, avoiding one-day-at-a-time scans.
pub fn nth_business_day_of_year_with_holidays(
    date: NaiveDate,
    ordinal: u32,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if ordinal == 0 {
        return None;
    }
    let first = crate::start_of_year(date);
    let calendar = crate::IndexedHolidayCalendar::new(holidays);
    let first_count = if calendar.is_business_day(first) {
        1_i64
    } else {
        0
    };
    let steps = i64::from(ordinal) - first_count;
    let target = if steps == 0 {
        first
    } else {
        calendar.add_business_days(first, steps)?
    };
    (target.year() == date.year()).then_some(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn counts_weekdays_from_first_day_of_calendar_year() {
        let year = date(2024, 7, 4);
        assert_eq!(nth_business_day_of_year(year, 1), Some(date(2024, 1, 1)));
        assert_eq!(nth_business_day_of_year(year, 5), Some(date(2024, 1, 5)));
        assert_eq!(nth_business_day_of_year(year, 6), Some(date(2024, 1, 8)));
        assert_eq!(nth_business_day_of_year(date(2023, 7, 4), 1), Some(date(2023, 1, 2)));
        assert_eq!(nth_business_day_of_year(year, 0), None);
        assert_eq!(nth_business_day_of_year(year, 366), None);
    }

    #[test]
    fn excludes_unique_weekday_holidays_including_new_years_day() {
        let holidays = [
            date(2024, 1, 1),
            date(2024, 1, 2),
            date(2024, 1, 2),
            date(2024, 1, 6),
        ];
        let year = date(2024, 9, 1);
        assert_eq!(
            nth_business_day_of_year_with_holidays(year, 1, &holidays),
            Some(date(2024, 1, 3))
        );
        assert_eq!(
            nth_business_day_of_year_with_holidays(year, 5, &holidays),
            Some(date(2024, 1, 9))
        );
        assert_eq!(
            nth_business_day_of_year_with_holidays(year, 366, &holidays),
            None
        );
    }

    #[test]
    fn bounds_calendar_limits_and_large_ordinals() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let first = nth_business_day_of_year(edge, 1).unwrap();
            assert_eq!(first.year(), edge.year());
            assert_eq!(nth_business_day_of_year(edge, u32::MAX), None);
        }
    }
}
