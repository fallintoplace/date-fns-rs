//! ISO 8601 date intervals with a calendar duration after the start date.

use chrono::NaiveDate;

use crate::IsoCalendarDuration;

/// Resolve a START/PERIOD interval to a start and exclusive end date.
///
/// Month and year components use calendar-aware clamping, not fixed-day
/// conversions. A period of P1D ends on the following calendar date.
/// Negative periods, overflowing dates, and malformed text are rejected.
pub fn parse_iso_date_duration_interval(input: &str) -> Option<(NaiveDate, NaiveDate)> {
    let (start, period) = input.split_once('/')?;
    let start = crate::parse_iso_date(start)?;
    let period = crate::parse_iso_calendar_duration(period)?;
    if period.negative {
        return None;
    }
    let end = crate::apply_iso_calendar_duration(start, period)?;
    (end >= start).then_some((start, end))
}

/// Format a START/PERIOD interval after validating its resolved end.
///
/// The duration remains calendar-relative and may therefore resolve
/// differently for different start dates.
pub fn format_iso_date_duration_interval(
    start: NaiveDate,
    period: IsoCalendarDuration,
) -> Option<String> {
    if period.negative {
        return None;
    }
    let first = crate::format_iso_date(start)?;
    let end = crate::apply_iso_calendar_duration(start, period)?;
    if end < start {
        return None;
    }
    // The resulting endpoint must be representable by the same ISO grammar.
    crate::format_iso_date(end)?;
    Some(format!(
        "{first}/{}",
        crate::format_iso_calendar_duration(period)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn parses_calendar_month_clamping_and_leap_years() {
        assert_eq!(
            parse_iso_date_duration_interval("2024-01-31/P1M"),
            Some((date(2024, 1, 31), date(2024, 2, 29)))
        );
        assert_eq!(
            parse_iso_date_duration_interval("2024-02-29/P1Y"),
            Some((date(2024, 2, 29), date(2025, 2, 28)))
        );
        assert_eq!(
            parse_iso_date_duration_interval("2024-02-28/P2W"),
            Some((date(2024, 2, 28), date(2024, 3, 13)))
        );
        assert_eq!(
            parse_iso_date_duration_interval("2024-02-29/P0D"),
            Some((date(2024, 2, 29), date(2024, 2, 29)))
        );
    }

    #[test]
    fn formats_nonnegative_periods_and_round_trips_dates() {
        let period = crate::parse_iso_calendar_duration("P1Y2M3D").unwrap();
        let start = date(2024, 1, 31);
        let encoded = format_iso_date_duration_interval(start, period).unwrap();
        assert_eq!(encoded, "2024-01-31/P1Y2M3D");
        assert_eq!(
            parse_iso_date_duration_interval(&encoded),
            Some((start, crate::apply_iso_calendar_duration(start, period).unwrap()))
        );
    }

    #[test]
    fn rejects_negative_malformed_and_unrepresentable_periods() {
        for text in [
            "2024-01-31/-P1D",
            "2024-01-31/P1W1D",
            "2024-01-31/P",
            "2024-01-31/P1D/extra",
            "2024-02-30/P1D",
            "2024-01-31/",
        ] {
            assert_eq!(parse_iso_date_duration_interval(text), None, "{text}");
        }
        let negative = crate::parse_iso_calendar_duration("-P1M").unwrap();
        assert_eq!(format_iso_date_duration_interval(date(2024, 3, 1), negative), None);
        let huge = crate::parse_iso_calendar_duration("P4294967295Y").unwrap();
        assert_eq!(format_iso_date_duration_interval(date(2024, 3, 1), huge), None);
        let day = crate::parse_iso_calendar_duration("P1D").unwrap();
        assert_eq!(format_iso_date_duration_interval(date(9999, 12, 31), day), None);
    }
}
