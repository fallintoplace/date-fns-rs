//! Checked decade and century boundaries at nanosecond precision.

use chrono::NaiveDateTime;

/// First midnight of the Gregorian decade containing the timestamp.
pub fn start_of_decade_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::start_of_decade(value.date())?.and_hms_opt(0, 0, 0)
}

/// Last nanosecond of the Gregorian decade containing the timestamp.
pub fn end_of_decade_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::end_of_decade(value.date())?.and_hms_nano_opt(23, 59, 59, 999_999_999)
}

/// First midnight of the one-based century containing the timestamp.
pub fn start_of_century_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::start_of_century(value.date())?.and_hms_opt(0, 0, 0)
}

/// Last nanosecond of the one-based century containing the timestamp.
pub fn end_of_century_datetime(value: NaiveDateTime) -> Option<NaiveDateTime> {
    crate::end_of_century(value.date())?.and_hms_nano_opt(23, 59, 59, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn datetime(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(12, 34, 56, 123_456_789)
            .unwrap()
    }

    #[test]
    fn spans_full_decades_and_centuries() {
        let value = datetime(2024, 2, 29);
        assert_eq!(
            start_of_decade_datetime(value),
            NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_decade_datetime(value),
            NaiveDate::from_ymd_opt(2029, 12, 31)
                .unwrap()
                .and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(
            start_of_century_datetime(value),
            NaiveDate::from_ymd_opt(2001, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_century_datetime(value),
            NaiveDate::from_ymd_opt(2100, 12, 31)
                .unwrap()
                .and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
    }

    #[test]
    fn century_boundary_uses_one_based_years() {
        assert_eq!(
            end_of_century_datetime(datetime(2000, 12, 31))
                .unwrap()
                .date(),
            NaiveDate::from_ymd_opt(2000, 12, 31).unwrap()
        );
        assert_eq!(
            start_of_century_datetime(datetime(2001, 1, 1))
                .unwrap()
                .date(),
            NaiveDate::from_ymd_opt(2001, 1, 1).unwrap()
        );
    }

    #[test]
    fn unsupported_boundaries_return_none_without_panicking() {
        for edge in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            let _ = start_of_decade_datetime(edge);
            let _ = end_of_decade_datetime(edge);
            let _ = start_of_century_datetime(edge);
            let _ = end_of_century_datetime(edge);
        }
    }
}
