//! Atomic field replacement for naive datetimes.

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

/// Date and clock fields to replace together without intermediate mutations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateTimeFieldPatch {
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub second: Option<u32>,
    pub nanosecond: Option<u32>,
}

/// Replace selected datetime fields and validate the complete timestamp once.
///
/// Omitted components are copied from the original value. Leap seconds are
/// not supported. Returns None for invalid dates, clock values, or ranges.
pub fn set_datetime_fields(
    value: NaiveDateTime,
    patch: DateTimeFieldPatch,
) -> Option<NaiveDateTime> {
    let date = value.date();
    let time = value.time();
    NaiveDate::from_ymd_opt(
        patch.year.unwrap_or(date.year()),
        patch.month.unwrap_or(date.month()),
        patch.day.unwrap_or(date.day()),
    )?
    .and_hms_nano_opt(
        patch.hour.unwrap_or(time.hour()),
        patch.minute.unwrap_or(time.minute()),
        patch.second.unwrap_or(time.second()),
        patch.nanosecond.unwrap_or(time.nanosecond()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_leap_days_and_replaces_clock_fields() {
        let original = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, 456_000_000)
            .unwrap();
        let patch = DateTimeFieldPatch {
            year: Some(2023),
            month: Some(3),
            day: Some(1),
            hour: Some(9),
            ..DateTimeFieldPatch::default()
        };
        assert_eq!(
            set_datetime_fields(original, patch),
            Some(
                NaiveDate::from_ymd_opt(2023, 3, 1)
                    .unwrap()
                    .and_hms_nano_opt(9, 59, 59, 456_000_000)
                    .unwrap()
            )
        );
        assert_eq!(
            set_datetime_fields(original, DateTimeFieldPatch::default()),
            Some(original)
        );
    }

    #[test]
    fn rejects_invalid_dates_and_clock_values() {
        let original = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            set_datetime_fields(
                original,
                DateTimeFieldPatch {
                    year: Some(2023),
                    ..DateTimeFieldPatch::default()
                }
            ),
            None
        );
        assert_eq!(
            set_datetime_fields(
                original,
                DateTimeFieldPatch {
                    hour: Some(24),
                    ..DateTimeFieldPatch::default()
                }
            ),
            None
        );
        assert_eq!(
            set_datetime_fields(
                original,
                DateTimeFieldPatch {
                    nanosecond: Some(1_000_000_000),
                    ..DateTimeFieldPatch::default()
                }
            ),
            None
        );
    }
}
