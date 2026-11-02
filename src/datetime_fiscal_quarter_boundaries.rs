//! Fixed-calendar fiscal-quarter boundaries for naive date-times.

use chrono::NaiveDateTime;

/// Midnight of the fiscal quarter containing value.
///
/// The fiscal year begins in first_month, which must be 1 through 12.
pub fn start_of_fiscal_quarter_datetime(
    value: NaiveDateTime,
    first_month: u32,
) -> Option<NaiveDateTime> {
    crate::start_of_fiscal_quarter(value.date(), first_month)?.and_hms_opt(0, 0, 0)
}

/// Final nanosecond of the fiscal quarter containing value.
pub fn end_of_fiscal_quarter_datetime(
    value: NaiveDateTime,
    first_month: u32,
) -> Option<NaiveDateTime> {
    crate::end_of_fiscal_quarter(value.date(), first_month)?.and_hms_nano_opt(
        23,
        59,
        59,
        999_999_999,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn handles_february_fiscal_year_across_new_year() {
        let value = date(2024, 1, 15).and_hms_opt(12, 0, 0).unwrap();
        assert_eq!(
            start_of_fiscal_quarter_datetime(value, 2),
            date(2023, 11, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_fiscal_quarter_datetime(value, 2),
            date(2024, 1, 31).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
    }

    #[test]
    fn validates_fiscal_start_and_supports_calendar_limits() {
        let value = date(2024, 2, 29).and_hms_opt(10, 0, 0).unwrap();
        assert_eq!(start_of_fiscal_quarter_datetime(value, 0), None);
        assert_eq!(end_of_fiscal_quarter_datetime(value, 13), None);
        assert_eq!(
            start_of_fiscal_quarter_datetime(value, 4),
            date(2024, 1, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_fiscal_quarter_datetime(value, 4),
            date(2024, 3, 31).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        assert_eq!(
            end_of_fiscal_quarter_datetime(NaiveDateTime::MAX, 1),
            Some(NaiveDateTime::MAX)
        );
    }
}
