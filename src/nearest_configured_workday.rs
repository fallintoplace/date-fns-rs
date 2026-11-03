//! Find the nearest allowed workday without scanning calendar days.

use chrono::NaiveDate;

use crate::{add_workdays_with_holidays, Workweek};

/// Return the closest workday including the input when it is allowed.
///
/// The configured workweek may have any nonempty weekday mask. Duplicate
/// holidays do not matter. Equidistant candidates prefer the earlier date.
/// Returns None if neither side contains a representable workday.
pub fn nearest_workday_with_holidays(
    week: Workweek,
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if week.is_workday(date) && !holidays.contains(&date) {
        return Some(date);
    }

    let previous = add_workdays_with_holidays(week, date, -1, holidays);
    let next = add_workdays_with_holidays(week, date, 1, holidays);

    match (previous, next) {
        (Some(before), Some(after)) => {
            let backward = date.signed_duration_since(before).num_days();
            let forward = after.signed_duration_since(date).num_days();
            Some(if backward <= forward { before } else { after })
        }
        (Some(before), None) => Some(before),
        (None, Some(after)) => Some(after),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn chooses_nearest_and_earlier_on_equal_distances() {
        let week = Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap();
        let holidays = [day(2024, 3, 3), day(2024, 3, 3)];
        assert_eq!(
            nearest_workday_with_holidays(week, day(2024, 3, 1), &holidays),
            Some(day(2024, 2, 29))
        );
        assert_eq!(
            nearest_workday_with_holidays(week, day(2024, 3, 2), &holidays),
            Some(day(2024, 2, 29))
        );
        assert_eq!(
            nearest_workday_with_holidays(week, day(2024, 3, 3), &holidays),
            Some(day(2024, 3, 4))
        );
        assert_eq!(
            nearest_workday_with_holidays(week, day(2024, 3, 4), &holidays),
            Some(day(2024, 3, 4))
        );
    }

    #[test]
    fn navigates_single_weekday_calendars_at_date_limits() {
        let week = Workweek::new(&[Weekday::Mon]).unwrap();
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let nearest = nearest_workday_with_holidays(week, edge, &[]).unwrap();
            assert!(week.is_workday(nearest));
        }
        assert_eq!(
            nearest_workday_with_holidays(
                Workweek::from_mask(0x7f).unwrap(),
                NaiveDate::MIN,
                &[NaiveDate::MIN]
            ),
            NaiveDate::MIN.succ_opt()
        );
    }
}
