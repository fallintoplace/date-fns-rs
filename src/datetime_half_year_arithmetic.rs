//! Checked six-month calendar shifts for naive datetimes.

use chrono::NaiveDateTime;

/// Add calendar half-years while preserving the original clock time.
///
/// Missing target days clamp to the last day of the target month.
pub fn add_half_years_datetime(value: NaiveDateTime, half_years: i32) -> Option<NaiveDateTime> {
    Some(crate::add_half_years(value.date(), half_years)?.and_time(value.time()))
}

/// Subtract calendar half-years while preserving clock precision.
pub fn sub_half_years_datetime(value: NaiveDateTime, half_years: i32) -> Option<NaiveDateTime> {
    Some(crate::sub_half_years(value.date(), half_years)?.and_time(value.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(23, 59, 58, 123_456_789)
            .unwrap()
    }

    #[test]
    fn preserves_time_across_clamped_dates() {
        assert_eq!(
            add_half_years_datetime(at(2023, 8, 31), 1),
            Some(at(2024, 2, 29))
        );
        assert_eq!(
            sub_half_years_datetime(at(2024, 8, 31), 1),
            Some(at(2024, 2, 29))
        );
        assert_eq!(
            add_half_years_datetime(at(2024, 8, 31), -1),
            Some(at(2024, 2, 29))
        );
        assert_eq!(
            sub_half_years_datetime(at(2024, 8, 31), -1),
            Some(at(2025, 2, 28))
        );
    }

    #[test]
    fn rejects_overflow_and_accepts_zero_offsets() {
        assert_eq!(add_half_years_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_half_years_datetime(NaiveDateTime::MIN, 1), None);
        assert_eq!(add_half_years_datetime(at(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_half_years_datetime(at(2024, 1, 1), i32::MIN), None);
        assert_eq!(
            add_half_years_datetime(NaiveDateTime::MAX, 0),
            Some(NaiveDateTime::MAX)
        );
    }
}
