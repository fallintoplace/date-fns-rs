//! English date-distance wording. These functions never read the system clock.

use chrono::NaiveDate;

/// Return a precise signed day distance in English.
/// Zero returns "0 days", and nonzero values optionally include direction.
pub fn format_date_distance_strict_days_en(
    target: NaiveDate,
    reference: NaiveDate,
    with_suffix: bool,
) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    let magnitude = delta.abs();
    let plural = if magnitude == 1 { "day" } else { "days" };
    let label = format!("{magnitude} {plural}");
    if !with_suffix || delta == 0 {
        label
    } else if delta > 0 {
        format!("in {label}")
    } else {
        format!("{label} ago")
    }
}

/// Return concise approximate English distance wording.
/// Week, month and year labels use rounded 7-, 30- and 365-day lengths.
/// Use the strict function when exact calendar-day counts are needed.
pub fn format_date_distance_en(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    if delta == 0 {
        return "today".to_owned();
    }
    if delta == 1 {
        return "tomorrow".to_owned();
    }
    if delta == -1 {
        return "yesterday".to_owned();
    }
    let days = delta.abs();
    let (amount, unit) = if days < 7 {
        (days, "day")
    } else if days < 28 {
        ((days + 3) / 7, "week")
    } else if days < 365 {
        ((days + 15) / 30, "month")
    } else {
        ((days + 182) / 365, "year")
    };
    let noun = if amount == 1 {
        unit.to_owned()
    } else {
        format!("{unit}s")
    };
    let label = format!("{amount} {noun}");
    if delta > 0 {
        format!("in {label}")
    } else {
        format!("{label} ago")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishes_exact_and_humanized_differences() {
        let origin = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        assert_eq!(format_date_distance_en(origin, origin), "today");
        assert_eq!(
            format_date_distance_en(origin.succ_opt().unwrap(), origin),
            "tomorrow"
        );
        assert_eq!(
            format_date_distance_en(origin.pred_opt().unwrap(), origin),
            "yesterday"
        );
        let leap_span = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        assert_eq!(
            format_date_distance_strict_days_en(leap_span, origin, true),
            "in 2 days"
        );
        assert_eq!(
            format_date_distance_strict_days_en(origin, leap_span, true),
            "2 days ago"
        );
        assert_eq!(
            format_date_distance_strict_days_en(origin, leap_span, false),
            "2 days"
        );
        assert_eq!(
            format_date_distance_en(crate::add_days(origin, 7).unwrap(), origin),
            "in 1 week"
        );
        assert_eq!(
            format_date_distance_en(crate::add_days(origin, 30).unwrap(), origin),
            "in 1 month"
        );
    }
}
