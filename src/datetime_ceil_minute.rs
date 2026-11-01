//! Ceil naive datetime values to minute buckets anchored at midnight.

use chrono::{NaiveDateTime, TimeDelta};

/// Ceil a timestamp to a positive minute step measured from midnight.
///
/// Exact boundaries remain unchanged. Accepts 1-1440 minute steps and returns
/// None on invalid steps or date-range overflow.
pub fn ceil_to_minute_step_datetime(
    value: NaiveDateTime,
    step_minutes: u32,
) -> Option<NaiveDateTime> {
    if !(1..=1_440).contains(&step_minutes) {
        return None;
    }
    let midnight = value.date().and_hms_opt(0, 0, 0)?;
    let elapsed = value.signed_duration_since(midnight).num_nanoseconds()?;
    let step = i64::from(step_minutes) * 60 * 1_000_000_000;
    let rounded = if elapsed % step == 0 {
        elapsed
    } else {
        (elapsed / step + 1).checked_mul(step)?
    };
    midnight.checked_add_signed(TimeDelta::nanoseconds(rounded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn carries_across_midnight_and_preserves_exact_boundaries() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let next = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let late = date.and_hms_opt(23, 55, 1).unwrap();
        assert_eq!(
            ceil_to_minute_step_datetime(late, 15),
            next.and_hms_opt(0, 0, 0)
        );
        let exact = date.and_hms_opt(12, 30, 0).unwrap();
        assert_eq!(ceil_to_minute_step_datetime(exact, 15), Some(exact));
        let fractional = date.and_hms_nano_opt(0, 0, 0, 1).unwrap();
        assert_eq!(
            ceil_to_minute_step_datetime(fractional, 15),
            date.and_hms_opt(0, 15, 0)
        );
    }

    #[test]
    fn validates_steps_and_checks_datetime_overflow() {
        assert_eq!(ceil_to_minute_step_datetime(NaiveDateTime::MIN, 0), None);
        assert_eq!(
            ceil_to_minute_step_datetime(NaiveDateTime::MIN, 1_441),
            None
        );
        assert_eq!(
            ceil_to_minute_step_datetime(NaiveDateTime::MIN, 15),
            Some(NaiveDateTime::MIN)
        );
        assert_eq!(ceil_to_minute_step_datetime(NaiveDateTime::MAX, 15), None);
    }
}
