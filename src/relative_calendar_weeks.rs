//! Adjacent week comparisons with an explicit first weekday.

use chrono::{Datelike, NaiveDate, Weekday};

fn week_index(date: NaiveDate, first_day: Weekday) -> i64 {
    let offset = (date.weekday().num_days_from_monday() + 7
        - first_day.num_days_from_monday())
        % 7;
    (i64::from(date.num_days_from_ce()) - i64::from(offset)).div_euclid(7)
}

/// Return true when target is in the week after reference's week.
///
/// Week boundaries use the supplied weekday, including across year changes.
pub fn is_next_calendar_week(
    target: NaiveDate,
    reference: NaiveDate,
    first_day: Weekday,
) -> bool {
    week_index(target, first_day) - week_index(reference, first_day) == 1
}

/// Return true when target is in the week before reference's week.
pub fn is_previous_calendar_week(
    target: NaiveDate,
    reference: NaiveDate,
    first_day: Weekday,
) -> bool {
    week_index(target, first_day) - week_index(reference, first_day) == -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn respects_configured_week_start() {
        let saturday = date(2024, 3, 9);
        let sunday = date(2024, 3, 10);
        let monday = date(2024, 3, 11);
        assert!(is_next_calendar_week(sunday, saturday, Weekday::Sun));
        assert!(!is_next_calendar_week(sunday, saturday, Weekday::Mon));
        assert!(is_next_calendar_week(monday, sunday, Weekday::Mon));
        assert!(is_previous_calendar_week(saturday, sunday, Weekday::Sun));
    }

    #[test]
    fn handles_new_year_and_chrono_boundaries() {
        assert!(is_next_calendar_week(
            date(2024, 1, 1),
            date(2023, 12, 31),
            Weekday::Mon
        ));
        assert!(!is_next_calendar_week(
            date(2024, 1, 1),
            date(2023, 12, 31),
            Weekday::Sun
        ));
        assert!(!is_next_calendar_week(
            NaiveDate::MIN,
            NaiveDate::MAX,
            Weekday::Mon
        ));
    }
}
