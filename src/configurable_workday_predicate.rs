//! Holiday-aware workday checks for configurable working weeks.

use chrono::NaiveDate;

use crate::Workweek;

/// Return whether a date is a configured workday and not a holiday.
///
/// Holidays may be unsorted or duplicated. Holiday dates on non-working
/// weekdays do not change the answer.
pub fn is_workday_with_holidays(week: Workweek, date: NaiveDate, holidays: &[NaiveDate]) -> bool {
    week.is_workday(date) && !holidays.contains(&date)
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use super::*;

    fn day(month: u32, date: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, month, date).unwrap()
    }

    #[test]
    fn follows_non_western_workweeks_and_skips_holidays() {
        let week = Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap();
        let monday = day(3, 4);
        let holidays = [monday, monday, day(3, 2)];
        assert!(is_workday_with_holidays(week, day(3, 3), &holidays));
        assert!(!is_workday_with_holidays(week, monday, &holidays));
        assert!(!is_workday_with_holidays(week, day(3, 1), &holidays));
        assert!(!is_workday_with_holidays(week, day(3, 2), &holidays));
        assert!(is_workday_with_holidays(week, monday, &[]));
    }

    #[test]
    fn supports_full_week_calendars_and_date_limits() {
        let week = Workweek::from_mask(0x7f).unwrap();
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            assert!(is_workday_with_holidays(week, edge, &[]));
            assert!(!is_workday_with_holidays(week, edge, &[edge, edge]));
        }
    }
}
