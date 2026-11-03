//! Lazy iteration of minute starts within inclusive datetime ranges.
use chrono::{NaiveDateTime, Timelike};

use crate::MinutesInDatetimeInterval;

/// Iterate minute boundaries at a positive `step_minutes` increment.
///
/// The first sample is floored to a minute boundary, with seconds and
/// subseconds cleared. Partial first and last minutes are included.
pub fn each_minute_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
    step_minutes: u32,
) -> Option<MinutesInDatetimeInterval> {
    if start > end {
        return None;
    }
    let aligned = start
        .date()
        .and_hms_opt(start.hour(), start.minute(), 0)?;
    crate::each_nth_minute_of_datetime_interval(aligned, end, step_minutes)
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
    fn floors_partial_minutes_and_crosses_midnight() {
        let start = at(2024, 2, 29, 23, 59)
            .with_nanosecond(0)
            .unwrap()
            .checked_add_signed(chrono::TimeDelta::seconds(30))
            .unwrap();
        let end = at(2024, 3, 1, 0, 1);
        let items: Vec<_> = each_minute_of_datetime_interval(start, end, 1)
            .unwrap()
            .collect();
        assert_eq!(
            items,
            vec![
                at(2024, 2, 29, 23, 59),
                at(2024, 3, 1, 0, 0),
                end,
            ]
        );
    }

    #[test]
    fn supports_steps_and_rejects_invalid_inputs() {
        let start = at(2024, 2, 29, 23, 59);
        let end = at(2024, 3, 1, 0, 1);
        let items: Vec<_> = each_minute_of_datetime_interval(start, end, 2)
            .unwrap()
            .collect();
        assert_eq!(items, vec![start, end]);
        assert!(each_minute_of_datetime_interval(end, start, 1).is_none());
        assert!(each_minute_of_datetime_interval(start, end, 0).is_none());
    }

    #[test]
    fn stays_fused_at_chrono_datetime_limits() {
        let mut items =
            each_minute_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, 1)
                .unwrap();
        assert!(items.next().is_some());
        assert_eq!(items.next(), None);
        assert_eq!(items.next(), None);
    }
}
