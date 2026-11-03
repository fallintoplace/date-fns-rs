//! Half-up rounding to configurable whole-hour increments.
use chrono::{NaiveDateTime, TimeDelta};

/// Round a naive datetime to the nearest multiple of `step_hours`.
///
/// The step is 1 through 12. Ties round upward and the step is anchored
/// at local midnight. Returns None for invalid steps or datetime overflow.
pub fn round_to_nearest_hours_datetime(
    value: NaiveDateTime,
    step_hours: u32,
) -> Option<NaiveDateTime> {
    if !(1..=12).contains(&step_hours) {
        return None;
    }
    let midnight = value.date().and_hms_opt(0, 0, 0)?;
    let elapsed = value.signed_duration_since(midnight).num_nanoseconds()?;
    let step = i64::from(step_hours) * 3_600_000_000_000;
    let rounded = ((elapsed + step / 2) / step) * step;
    midnight.checked_add_signed(TimeDelta::nanoseconds(rounded))
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
    fn handles_date_fns_hour_step_examples() {
        let value = at(2014, 7, 10, 12, 35);
        assert_eq!(
            round_to_nearest_hours_datetime(value, 1),
            Some(at(2014, 7, 10, 13, 0))
        );
        assert_eq!(
            round_to_nearest_hours_datetime(value, 6),
            Some(at(2014, 7, 10, 12, 0))
        );
        assert_eq!(
            round_to_nearest_hours_datetime(value, 8),
            Some(at(2014, 7, 10, 16, 0))
        );
    }

    #[test]
    fn rounds_ties_up_and_carries_across_dates() {
        assert_eq!(
            round_to_nearest_hours_datetime(at(2024, 2, 29, 6, 0), 12),
            Some(at(2024, 2, 29, 12, 0))
        );
        assert_eq!(
            round_to_nearest_hours_datetime(at(2024, 2, 29, 23, 59), 1),
            Some(at(2024, 3, 1, 0, 0))
        );
    }

    #[test]
    fn validates_steps_and_detects_overflow() {
        let instant = at(2024, 1, 1, 12, 0);
        assert_eq!(round_to_nearest_hours_datetime(instant, 0), None);
        assert_eq!(round_to_nearest_hours_datetime(instant, 13), None);
        assert_eq!(round_to_nearest_hours_datetime(NaiveDateTime::MAX, 1), None);
    }
}
