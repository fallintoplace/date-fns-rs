//! Comma-separated RFC 5545 DATE values for RDATE and EXDATE properties.

use chrono::NaiveDate;

/// Parse a non-empty comma-separated list of basic calendar DATE values.
///
/// Input order and duplicates are preserved. Property parameters and names
/// are handled by the caller.
pub fn parse_ical_date_list(input: &str) -> Option<Vec<NaiveDate>> {
    input.split(',').map(crate::parse_ical_date).collect()
}

/// Format non-empty DATE lists without changing order or duplicates.
///
/// Returns None for an empty list or a date outside the four-digit year range.
pub fn format_ical_date_list(dates: &[NaiveDate]) -> Option<String> {
    if dates.is_empty() {
        return None;
    }
    let parts: Vec<_> = dates
        .iter()
        .copied()
        .map(crate::format_ical_date)
        .collect::<Option<Vec<_>>>()?;
    Some(parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn round_trips_rdate_and_exdate_date_values() {
        let dates = vec![date(2024, 2, 29), date(2024, 3, 1), date(2024, 2, 29)];
        let input = "20240229,20240301,20240229";
        assert_eq!(parse_ical_date_list(input), Some(dates.clone()));
        assert_eq!(format_ical_date_list(&dates).as_deref(), Some(input));
    }

    #[test]
    fn rejects_empty_elements_invalid_dates_and_out_of_range_years() {
        for input in [
            "",
            "20240229,",
            ",20240229",
            "20240229,,20240301",
            "20240230",
            "2024-02-29",
            "20240229,20240301 ",
        ] {
            assert!(parse_ical_date_list(input).is_none(), "{input}");
        }
        assert_eq!(format_ical_date_list(&[]), None);
        assert_eq!(format_ical_date_list(&[NaiveDate::MAX]), None);
    }
}
