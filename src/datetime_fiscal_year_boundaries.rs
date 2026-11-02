//! Fiscal-year boundaries on naive timestamps with an explicit first month.

use chrono::NaiveDateTime;

/// Return midnight on the opening day of the containing fiscal year.
///
/// The first fiscal month must be between January (1) and December (12).
pub fn start_of_fiscal_year_datetime(
    value: NaiveDateTime,
    first_month: u32,
) -> Option<NaiveDateTime> {
    crate::start_of_fiscal_year(value.date(), first_month)?.and_hms_opt(0, 0, 0)
}

/// Return the last nanosecond of the containing fiscal year.
pub fn end_of_fiscal_year_datetime(
    value: NaiveDateTime,
    first_month: u32,
) -> Option<NaiveDateTime> {
    crate::end_of_fiscal_year(value.date(), first_month)?.and_hms_nano_opt(23, 59, 59, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn retains_april_to_march_fiscal_year_boundaries() {
        let value = date(2024, 2, 29).and_hms_opt(10, 45, 15).unwrap();
        assert_eq!(
            start_of_fiscal_year_datetime(value, 4),
            date(2023, 4, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_fiscal_year_datetime(value, 4),
            date(2024, 3, 31).and_hms_nano_opt(23, 59, 59, 999_999_999)
        );
    }

    #[test]
    fn checks_invalid_months_and_date_range_boundaries() {
        let value = date(2024, 1, 1).and_hms_opt(0, 0, 0).unwrap();
        assert_eq!(start_of_fiscal_year_datetime(value, 0), None);
        assert_eq!(end_of_fiscal_year_datetime(value, 13), None);
        assert_eq!(
            start_of_fiscal_year_datetime(value, 1),
            date(2024, 1, 1).and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            end_of_fiscal_year_datetime(NaiveDateTime::MAX, 1),
            Some(NaiveDateTime::MAX)
        );
        assert_eq!(end_of_fiscal_year_datetime(NaiveDateTime::MAX, 2), None);
    }
}
