//! Checked calendar-unit shifts for naive date-times.

use chrono::{NaiveDate, NaiveDateTime};

fn at_same_time(value: NaiveDateTime, date: Option<NaiveDate>) -> Option<NaiveDateTime> {
    Some(date?.and_time(value.time()))
}

/// Shift calendar months, clamping the day to the target month's end.
pub fn add_months_datetime(value: NaiveDateTime, months: i32) -> Option<NaiveDateTime> {
    at_same_time(value, crate::add_months(value.date(), months))
}

/// Subtract calendar months, preserving the local time component.
pub fn sub_months_datetime(value: NaiveDateTime, months: i32) -> Option<NaiveDateTime> {
    at_same_time(value, crate::sub_months(value.date(), months))
}

/// Shift calendar years with leap-day clamping.
pub fn add_years_datetime(value: NaiveDateTime, years: i32) -> Option<NaiveDateTime> {
    at_same_time(value, crate::add_years(value.date(), years))
}

/// Subtract calendar years with leap-day clamping.
pub fn sub_years_datetime(value: NaiveDateTime, years: i32) -> Option<NaiveDateTime> {
    at_same_time(value, crate::sub_years(value.date(), years))
}

/// Shift calendar quarters.
pub fn add_quarters_datetime(value: NaiveDateTime, quarters: i32) -> Option<NaiveDateTime> {
    at_same_time(value, crate::add_quarters(value.date(), quarters))
}

/// Subtract calendar quarters.
pub fn sub_quarters_datetime(value: NaiveDateTime, quarters: i32) -> Option<NaiveDateTime> {
    at_same_time(value, crate::sub_quarters(value.date(), quarters))
}

/// Add months while keeping month-end datetimes at the target month's end.
///
/// Non-month-end days use normal date clamping. Clock precision is unchanged.
pub fn add_months_preserving_end_of_month_datetime(
    value: NaiveDateTime,
    months: i32,
) -> Option<NaiveDateTime> {
    at_same_time(value, crate::add_months_preserving_end_of_month(value.date(), months))
}

/// Add years while retaining the source's end-of-month position.
///
/// For example, February 28, 2023 becomes February 29, 2024.
pub fn add_years_preserving_end_of_month_datetime(
    value: NaiveDateTime,
    years: i32,
) -> Option<NaiveDateTime> {
    at_same_time(value, crate::add_years_preserving_end_of_month(value.date(), years))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_milli_opt(23, 45, 56, 789)
            .unwrap()
    }

    #[test]
    fn clamps_dates_and_preserves_time() {
        assert_eq!(
            add_months_datetime(dt(2024, 1, 31), 1),
            Some(dt(2024, 2, 29))
        );
        assert_eq!(
            sub_months_datetime(dt(2024, 3, 31), 1),
            Some(dt(2024, 2, 29))
        );
        assert_eq!(
            add_years_datetime(dt(2024, 2, 29), 1),
            Some(dt(2025, 2, 28))
        );
        assert_eq!(
            sub_years_datetime(dt(2024, 2, 29), 1),
            Some(dt(2023, 2, 28))
        );
        assert_eq!(
            add_quarters_datetime(dt(2024, 1, 31), 1),
            Some(dt(2024, 4, 30))
        );
        assert_eq!(
            sub_quarters_datetime(dt(2024, 4, 30), 1),
            Some(dt(2024, 1, 30))
        );
    }

    #[test]
    fn returns_none_for_overflow() {
        assert_eq!(add_years_datetime(dt(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_quarters_datetime(dt(2024, 1, 1), i32::MIN), None);
        assert_eq!(add_months_datetime(NaiveDateTime::MAX, 1), None);
    }
    #[test]
    fn preserves_month_ends_and_subsecond_time_in_calendar_shifts() {
        assert_eq!(
            add_months_preserving_end_of_month_datetime(dt(2024, 2, 29), 1),
            Some(dt(2024, 3, 31))
        );
        assert_eq!(
            add_months_preserving_end_of_month_datetime(dt(2024, 1, 30), 1),
            Some(dt(2024, 2, 29))
        );
        assert_eq!(
            add_years_preserving_end_of_month_datetime(dt(2023, 2, 28), 1),
            Some(dt(2024, 2, 29))
        );
        assert_eq!(
            add_years_preserving_end_of_month_datetime(dt(2024, 2, 28), 1),
            Some(dt(2025, 2, 28))
        );
        assert_eq!(
            add_months_preserving_end_of_month_datetime(NaiveDateTime::MAX, 1),
            None
        );
    }

}
