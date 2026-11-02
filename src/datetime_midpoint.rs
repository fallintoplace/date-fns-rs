//! Overflow-safe midpoints of inclusive naive datetime ranges.

use chrono::{DateTime, NaiveDateTime, Utc};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

fn to_nanos(value: NaiveDateTime) -> i128 {
    i128::from(value.and_utc().timestamp()) * NANOS_PER_SECOND
        + i128::from(value.and_utc().timestamp_subsec_nanos())
}

/// Find the midpoint, rounding down to the earlier nanosecond.
///
/// The full Chrono datetime range is supported without i64 nanosecond
/// overflow. Reversed bounds return None.
pub fn datetime_interval_midpoint(
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Option<NaiveDateTime> {
    if start > end {
        return None;
    }
    let left = to_nanos(start);
    let midpoint = left + (to_nanos(end) - left) / 2;
    let seconds = i64::try_from(midpoint.div_euclid(NANOS_PER_SECOND)).ok()?;
    let nanos = u32::try_from(midpoint.rem_euclid(NANOS_PER_SECOND)).ok()?;
    DateTime::<Utc>::from_timestamp(seconds, nanos).map(|value| value.naive_utc())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn rounds_toward_the_start_at_nanosecond_precision() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, 0)
            .unwrap();
        let end = start.checked_add_signed(chrono::TimeDelta::nanoseconds(3)).unwrap();
        assert_eq!(
            datetime_interval_midpoint(start, end),
            start.checked_add_signed(chrono::TimeDelta::nanoseconds(1))
        );
        assert_eq!(datetime_interval_midpoint(start, start), Some(start));
        assert_eq!(datetime_interval_midpoint(end, start), None);
    }

    #[test]
    fn handles_full_datetime_range_without_overflow() {
        let midpoint = datetime_interval_midpoint(
            NaiveDateTime::MIN,
            NaiveDateTime::MAX,
        ).unwrap();
        assert!(midpoint > NaiveDateTime::MIN);
        assert!(midpoint < NaiveDateTime::MAX);
    }
}
