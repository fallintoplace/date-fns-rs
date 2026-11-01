//! Half-up rounding of naive date-times to clock increments.

use chrono::{NaiveDateTime, TimeDelta};

fn round_to_step(value: NaiveDateTime, seconds: i64) -> Option<NaiveDateTime> {
    let midnight = value.date().and_hms_opt(0, 0, 0)?;
    let nanos = value.signed_duration_since(midnight).num_nanoseconds()?;
    let step = seconds.checked_mul(1_000_000_000)?;
    let rounded = ((nanos + step / 2) / step) * step;
    midnight.checked_add_signed(TimeDelta::nanoseconds(rounded))
}

/// Round to the nearest positive number of minutes, from 1 through 30.
/// Exact halfway values round toward the next boundary.
pub fn round_to_nearest_minute_datetime(
    value: NaiveDateTime,
    nearest_to: u32,
) -> Option<NaiveDateTime> {
    if !(1..=30).contains(&nearest_to) {
        return None;
    }
    round_to_step(value, i64::from(nearest_to) * 60)
}

/// Round to the nearest whole second.
pub fn round_to_nearest_second_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    round_to_step(value, 1)
}

/// Round to the nearest whole hour.
pub fn round_to_nearest_hour_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    round_to_step(value, 3_600)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn rounds_up_on_ties_and_carries_across_midnight() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let after = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let tie = day.and_hms_opt(12, 7, 30).unwrap();
        assert_eq!(
            round_to_nearest_minute_datetime(tie, 15),
            Some(day.and_hms_opt(12, 15, 0).unwrap())
        );
        assert_eq!(
            round_to_nearest_hour_datetime(day.and_hms_opt(23, 31, 0).unwrap()),
            Some(after.and_hms_opt(0, 0, 0).unwrap())
        );
        assert_eq!(
            round_to_nearest_second_datetime(day.and_hms_milli_opt(1, 2, 3, 500).unwrap()),
            Some(day.and_hms_opt(1, 2, 4).unwrap())
        );
    }

    #[test]
    fn rejects_invalid_steps_and_datetime_overflow() {
        let value = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(round_to_nearest_minute_datetime(value, 0), None);
        assert_eq!(round_to_nearest_minute_datetime(value, 31), None);
        assert_eq!(round_to_nearest_hour_datetime(NaiveDateTime::MAX), None);
    }
}
