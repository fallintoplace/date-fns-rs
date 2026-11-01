//! Date-field setters for naive date-times with preserved clock precision.

use chrono::{NaiveDate, NaiveDateTime, Weekday};

fn with_date(value: NaiveDateTime, date: Option<NaiveDate>) -> Option<NaiveDateTime> {
    Some(date?.and_time(value.time()))
}

/// Replace the year, clamping February 29 if the new year is not a leap year.
pub fn set_year_datetime(value: NaiveDateTime, year: i32) -> Option<NaiveDateTime> {
    with_date(value, crate::set_year(value.date(), year))
}

/// Replace the 1-based month, clamping a day that is too large.
pub fn set_month_datetime(value: NaiveDateTime, month: u32) -> Option<NaiveDateTime> {
    with_date(value, crate::set_month(value.date(), month))
}

/// Replace the 1-based calendar quarter, retaining month position.
pub fn set_quarter_datetime(value: NaiveDateTime, quarter: u32) -> Option<NaiveDateTime> {
    with_date(value, crate::set_quarter(value.date(), quarter))
}

/// Replace the day of month; invalid day numbers return None.
pub fn set_day_of_month_datetime(value: NaiveDateTime, day: u32) -> Option<NaiveDateTime> {
    with_date(value, crate::set_day_of_month(value.date(), day))
}

/// Replace the 1-based day of the year; invalid ordinals return None.
pub fn set_day_of_year_datetime(
    value: NaiveDateTime,
    ordinal: u32,
) -> Option<NaiveDateTime> {
    with_date(value, crate::set_day_of_year(value.date(), ordinal))
}

/// Replace the weekday index (0 to 6) within an explicitly configured week.
pub fn set_day_of_week_datetime(
    value: NaiveDateTime,
    index: u32,
    week_starts_on: Weekday,
) -> Option<NaiveDateTime> {
    with_date(value, crate::set_day_of_week(value.date(), index, week_starts_on))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn at(day: NaiveDate) -> NaiveDateTime {
        day.and_hms_nano_opt(20, 15, 30, 123_456_789).unwrap()
    }

    #[test]
    fn clamps_dates_but_preserves_clock_and_nanoseconds() {
        let leap = at(date(2024, 2, 29));
        assert_eq!(set_year_datetime(leap, 2023), Some(at(date(2023, 2, 28))));
        assert_eq!(set_month_datetime(at(date(2024, 1, 31)), 2), Some(leap));
        assert_eq!(
            set_quarter_datetime(at(date(2024, 1, 31)), 2),
            Some(at(date(2024, 4, 30)))
        );
        assert_eq!(set_day_of_month_datetime(leap, 1), Some(at(date(2024, 2, 1))));
        assert_eq!(set_day_of_year_datetime(leap, 366), Some(at(date(2024, 12, 31))));
        assert_eq!(
            set_day_of_week_datetime(at(date(2024, 1, 1)), 0, Weekday::Sun),
            Some(at(date(2023, 12, 31)))
        );
    }

    #[test]
    fn rejects_invalid_components_and_out_of_range_week_shifts() {
        let value = at(date(2024, 2, 29));
        assert_eq!(set_month_datetime(value, 13), None);
        assert_eq!(set_quarter_datetime(value, 0), None);
        assert_eq!(set_day_of_month_datetime(value, 30), None);
        assert_eq!(set_day_of_year_datetime(value, 367), None);
        assert_eq!(set_day_of_week_datetime(value, 7, Weekday::Mon), None);
        assert_eq!(set_year_datetime(value, i32::MAX), None);
        assert_eq!(
            set_day_of_month_datetime(NaiveDateTime::MAX, NaiveDateTime::MAX.date().day()),
            Some(NaiveDateTime::MAX)
        );
    }
}
