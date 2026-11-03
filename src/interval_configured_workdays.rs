//! Lazy workday iteration for arbitrary weekday masks and holiday calendars.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::Workweek;

/// Iterate allowed workdays in the inclusive interval [start, end].
///
/// The iterator owns its deduplicated holiday set. It does not materialize
/// the result dates. Reversed intervals return None; date overflow stops
/// iteration naturally.
pub fn each_workday_of_interval_with_holidays(
    week: Workweek,
    start: NaiveDate,
    end: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<impl Iterator<Item = NaiveDate>> {
    if start > end {
        return None;
    }
    let blocked: BTreeSet<_> = holidays.iter().copied().collect();
    Some(
        std::iter::successors(Some(start), |date| date.succ_opt())
            .take_while(move |date| *date <= end)
            .filter(move |date| week.is_workday(*date) && !blocked.contains(date)),
    )
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use super::*;

    fn day(month: u32, date: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, month, date).unwrap()
    }

    #[test]
    fn iterates_custom_workweeks_across_leap_day_and_holidays() {
        let week = Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap();
        let holidays = [day(3, 3), day(3, 3), day(3, 1)];
        let result: Vec<_> = each_workday_of_interval_with_holidays(
            week,
            day(2, 29),
            day(3, 5),
            &holidays,
        )
        .unwrap()
        .collect();
        assert_eq!(result, vec![day(2, 29), day(3, 4), day(3, 5)]);
    }

    #[test]
    fn rejects_reversed_ranges_and_handles_date_boundaries() {
        let week = Workweek::from_mask(0x7f).unwrap();
        assert!(each_workday_of_interval_with_holidays(
            week,
            day(3, 5),
            day(3, 4),
            &[]
        )
        .is_none());
        let days: Vec<_> =
            each_workday_of_interval_with_holidays(week, NaiveDate::MAX, NaiveDate::MAX, &[])
                .unwrap()
                .collect();
        assert_eq!(days, vec![NaiveDate::MAX]);
    }
}
