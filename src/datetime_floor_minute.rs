//! Floor naive datetime values to minute buckets anchored at midnight.

use chrono::{NaiveDateTime, TimeDelta};

/// Floor a timestamp to a positive minute step measured from midnight.
///
/// Accepts steps from 1 to 1440 minutes and preserves the date. Returns None
/// for invalid steps or an unsupported datetime boundary.
pub fn floor_to_minute_step_datetime(
    value: NaiveDateTime,
    step_minutes: u32,
) -> Option<NaiveDateTime> {
    if !(1..=1_440).contains(&step_minutes) {
        return None;
    }
    let midnight = value.date().and_hms_opt(0, 0, 0)?;
    let elapsed = value.signed_duration_since(midnight).num_nanoseconds()?;
    let step = i64::from(step_minutes) * 60 * 1_000_000_000;
    let rounded = (elapsed / step) * step;
    midnight.checked_add_signed(TimeDelta::nanoseconds(rounded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn floors_fractional_times_with_midnight_alignment() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let value = date.and_hms_nano_opt(12, 29, 59, 999_999_999).unwrap();
        let floored = date.and_hms_opt(12, 15, 0).unwrap();
        assert_eq!(floor_to_minute_step_datetime(value, 15), Some(floored));
        assert_eq!(
            floor_to_minute_step_datetime(date.and_hms_opt(12, 30, 0).unwrap(), 15),
            date.and_hms_opt(12, 30, 0)
        );
        assert_eq!(
            floor_to_minute_step_datetime(date.and_hms_opt(23, 59, 0).unwrap(), 1_440),
            date.and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_invalid_steps_and_supports_datetime_limits() {
        assert_eq!(floor_to_minute_step_datetime(NaiveDateTime::MAX, 0), None);
        assert_eq!(
            floor_to_minute_step_datetime(NaiveDateTime::MIN, 1_441),
            None
        );
        assert_eq!(
            floor_to_minute_step_datetime(NaiveDateTime::MIN, 1),
            Some(NaiveDateTime::MIN)
        );
    }
}
