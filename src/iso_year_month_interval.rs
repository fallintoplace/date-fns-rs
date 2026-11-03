//! Calendar month ranges represented by two YYYY-MM labels.

use chrono::NaiveDate;

/// Parse a month-label range as inclusive first and last calendar dates.
///
/// The ending month includes its last day, even during a leap year.
pub fn parse_iso_year_month_interval(input: &str) -> Option<(NaiveDate, NaiveDate)> {
    let (left, right) = input.split_once('/')?;
    let start = crate::parse_iso_year_month(left)?;
    let end = crate::end_of_month(crate::parse_iso_year_month(right)?);
    (start <= end).then_some((start, end))
}

/// Format a range only when both endpoints enclose complete months.
pub fn format_iso_year_month_interval(start: NaiveDate, end: NaiveDate) -> Option<String> {
    if start > end || start != crate::start_of_month(start) || end != crate::end_of_month(end) {
        return None;
    }
    let left = crate::format_iso_year_month(start)?;
    let right = crate::format_iso_year_month(end)?;
    Some(format!("{left}/{right}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn includes_leap_days_and_cross_year_months() {
        let label = "2024-02/2024-03";
        let start = date(2024, 2, 1);
        let end = date(2024, 3, 31);
        assert_eq!(parse_iso_year_month_interval(label), Some((start, end)));
        assert_eq!(
            format_iso_year_month_interval(start, end).as_deref(),
            Some(label)
        );
        assert_eq!(
            parse_iso_year_month_interval("2023-12/2024-01"),
            Some((date(2023, 12, 1), date(2024, 1, 31)))
        );
        assert_eq!(
            parse_iso_year_month_interval("2024-02/2024-02"),
            Some((date(2024, 2, 1), date(2024, 2, 29)))
        );
    }

    #[test]
    fn rejects_reversed_partial_and_noncanonical_ranges() {
        for input in [
            "2024-03/2024-02",
            "2024-00/2024-02",
            "2024-02/2024-13",
            "2024-2/2024-03",
            "2024-02 /2024-03",
            "2024-02/2024-03/2024-04",
        ] {
            assert_eq!(parse_iso_year_month_interval(input), None, "{input}");
        }
        assert_eq!(
            format_iso_year_month_interval(date(2024, 2, 2), date(2024, 2, 29)),
            None
        );
        assert_eq!(
            format_iso_year_month_interval(date(2024, 2, 1), date(2024, 2, 28)),
            None
        );
        assert_eq!(
            format_iso_year_month_interval(NaiveDate::MIN, NaiveDate::MAX),
            None
        );
    }
}
