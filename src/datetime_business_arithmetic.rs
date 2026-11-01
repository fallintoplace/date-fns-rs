//! Checked business-day arithmetic that preserves local naive clock time.

use chrono::{NaiveDate, NaiveDateTime};

/// Add signed Monday-Friday days while preserving the exact time of day.
pub fn add_business_days_datetime(
    value: NaiveDateTime,
    days: i64,
) -> Option<NaiveDateTime> {
    Some(crate::add_business_days(value.date(), days)?.and_time(value.time()))
}

/// Subtract signed Monday-Friday days while preserving the exact time of day.
pub fn sub_business_days_datetime(
    value: NaiveDateTime,
    days: i64,
) -> Option<NaiveDateTime> {
    Some(crate::sub_business_days(value.date(), days)?.and_time(value.time()))
}

/// Add business days while skipping caller-supplied holiday dates.
pub fn add_business_days_with_holidays_datetime(
    value: NaiveDateTime,
    days: i64,
    holidays: &[NaiveDate],
) -> Option<NaiveDateTime> {
    Some(
        crate::add_business_days_with_holidays(value.date(), days, holidays)?
            .and_time(value.time()),
    )
}

/// Subtract business days while skipping caller-supplied holiday dates.
pub fn sub_business_days_with_holidays_datetime(
    value: NaiveDateTime,
    days: i64,
    holidays: &[NaiveDate],
) -> Option<NaiveDateTime> {
    Some(
        crate::sub_business_days_with_holidays(value.date(), days, holidays)?
            .and_time(value.time()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn at(date: NaiveDate) -> NaiveDateTime {
        date.and_hms_nano_opt(23, 59, 59, 123_456_789).unwrap()
    }

    #[test]
    fn retains_nanoseconds_while_crossing_weekends_and_holidays() {
        let friday = at(date(2024, 3, 1));
        let monday = at(date(2024, 3, 4));
        let tuesday = at(date(2024, 3, 5));

        assert_eq!(add_business_days_datetime(friday, 1), Some(monday));
        assert_eq!(sub_business_days_datetime(monday, 1), Some(friday));
        assert_eq!(add_business_days_datetime(friday, -1), Some(at(date(2024, 2, 29))));
        assert_eq!(add_business_days_datetime(friday, 0), Some(friday));
        assert_eq!(
            add_business_days_with_holidays_datetime(friday, 1, &[date(2024, 3, 4)]),
            Some(tuesday)
        );
        assert_eq!(
            sub_business_days_with_holidays_datetime(tuesday, 1, &[date(2024, 3, 4)]),
            Some(friday)
        );
    }

    #[test]
    fn validates_large_offsets_without_losing_clock_time() {
        assert_eq!(add_business_days_datetime(NaiveDateTime::MAX, 1), None);
        assert_eq!(sub_business_days_datetime(NaiveDateTime::MIN, 1), None);
        assert_eq!(sub_business_days_datetime(at(date(2024, 3, 1)), i64::MIN), None);
        assert_eq!(
            add_business_days_with_holidays_datetime(NaiveDateTime::MAX, 1, &[]),
            None
        );
    }
}
