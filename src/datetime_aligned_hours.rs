//! Lazy iteration of hour starts within inclusive datetime ranges.
use chrono::{NaiveDateTime, Timelike};

use crate::HoursInDatetimeInterval;

/// Iterate hour boundaries at a positive `step_hours` increment.
///
/// Unlike `each_nth_hour_of_datetime_interval`, this floors the first
/// sample to the start of its hour. A partially covered first hour counts.
pub fn each_hour_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
    step_hours: u32,
) -> Option<HoursInDatetimeInterval> {
    if start > end {
        return None;
    }
    let aligned = start.date().and_hms_opt(start.hour(), 0, 0)?;
    crate::each_nth_hour_of_datetime_interval(aligned, end, step_hours)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn floors_partial_hours_and_crosses_leap_midnight() {
        let start = at(2024, 2, 29, 23, 30);
        let end = at(2024, 3, 1, 2, 0);
        let items: Vec<_> = each_hour_of_datetime_interval(start, end, 1)
            .unwrap()
            .collect();
        assert_eq!(
            items,
            vec![
                at(2024, 2, 29, 23, 0),
                at(2024, 3, 1, 0, 0),
                at(2024, 3, 1, 1, 0),
                end,
            ]
        );
    }

    #[test]
    fn supports_steps_and_validates_reversed_intervals() {
        let start = at(2024, 2, 29, 23, 30);
        let end = at(2024, 3, 1, 2, 0);
        let items: Vec<_> = each_hour_of_datetime_interval(start, end, 2)
            .unwrap()
            .collect();
        assert_eq!(items, vec![at(2024, 2, 29, 23, 0), at(2024, 3, 1, 1, 0)]);
        assert!(each_hour_of_datetime_interval(end, start, 1).is_none());
        assert!(each_hour_of_datetime_interval(start, end, 0).is_none());
    }

    #[test]
    fn exhausts_cleanly_at_maximum_datetime() {
        let mut values =
            each_hour_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, 1).unwrap();
        assert!(values.next().is_some());
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }
}
