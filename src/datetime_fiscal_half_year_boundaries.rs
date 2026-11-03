//! Fiscal half-year boundaries on naive date-times.

use chrono::NaiveDateTime;

/// Return midnight at the start of the fiscal half-year containing `value`.
///
/// The fiscal year starts in `first_month` (1 through 12).
/// Invalid months and unrepresentable boundaries return None.
pub fn start_of_fiscal_half_year_datetime(
    value: NaiveDateTime,
    first_month: u32,
) -> Option<NaiveDateTime> {
    crate::start_of_fiscal_half_year(value.date(), first_month)?.and_hms_opt(0, 0, 0)
}

/// Return the final nanosecond of the fiscal half-year containing `value`.
pub fn end_of_fiscal_half_year_datetime(
    value: NaiveDateTime,
    first_month: u32,
) -> Option<NaiveDateTime> {
    crate::end_of_fiscal_half_year(value.date(), first_month)?.and_hms_nano_opt(
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

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn supports_april_to_march_fiscal_halves() {
        let january = day(2024, 1, 15).and_hms_opt(12, 34, 56).unwrap();
        assert_eq!(
            start_of_fiscal_half_year_datetime(january, 4),
            day(2023, 10, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_fiscal_half_year_datetime(january, 4),
            day(2024, 3, 31).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
        let october = day(2024, 10, 1).and_hms_opt(0, 0, 0).unwrap();
        assert_eq!(
            start_of_fiscal_half_year_datetime(october, 4),
            Some(october)
        );
    }

    #[test]
    fn validates_months_and_chrono_limits() {
        let leap_day = day(2024, 2, 29).and_hms_opt(18, 0, 0).unwrap();
        assert_eq!(start_of_fiscal_half_year_datetime(leap_day, 0), None);
        assert_eq!(end_of_fiscal_half_year_datetime(leap_day, 13), None);
        assert_eq!(
            end_of_fiscal_half_year_datetime(NaiveDateTime::MAX, 1),
            Some(NaiveDateTime::MAX)
        );
        assert_eq!(end_of_fiscal_half_year_datetime(NaiveDateTime::MAX, 2), None);
    }
}
