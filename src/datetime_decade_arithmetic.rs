//! Checked Gregorian decade shifts for naive datetimes.

use chrono::NaiveDateTime;

/// Add signed decades with leap-day clamping and unchanged clock precision.
pub fn add_decades_datetime(value: NaiveDateTime, decades: i32) -> Option<NaiveDateTime> {
    Some(crate::add_decades(value.date(), decades)?.and_time(value.time()))
}

/// Subtract signed decades without changing the clock components.
pub fn sub_decades_datetime(value: NaiveDateTime, decades: i32) -> Option<NaiveDateTime> {
    Some(crate::sub_decades(value.date(), decades)?.and_time(value.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(13, 14, 15, 987_654_321)
            .unwrap()
    }

    #[test]
    fn shifts_decades_and_retains_nanoseconds() {
        assert_eq!(
            add_decades_datetime(at(2024, 2, 29), 1),
            Some(at(2034, 2, 28))
        );
        assert_eq!(
            sub_decades_datetime(at(2024, 2, 29), 1),
            Some(at(2014, 2, 28))
        );
        assert_eq!(
            add_decades_datetime(at(2024, 1, 1), -2),
            Some(at(2004, 1, 1))
        );
        assert_eq!(
            sub_decades_datetime(at(2024, 1, 1), -2),
            Some(at(2044, 1, 1))
        );
    }

    #[test]
    fn refuses_invalid_offsets_and_date_overflow() {
        assert_eq!(add_decades_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_decades_datetime(NaiveDateTime::MIN, 1), None);
        assert_eq!(add_decades_datetime(at(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_decades_datetime(at(2024, 1, 1), i32::MIN), None);
        assert_eq!(
            add_decades_datetime(NaiveDateTime::MAX, 0),
            Some(NaiveDateTime::MAX)
        );
    }
}
