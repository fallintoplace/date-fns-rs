//! Italian relative calendar dates and approximate date-distance wording.

use chrono::{Datelike, NaiveDate, Weekday};

const WEEKDAYS: [&str; 7] = [
    "domenica", "lunedì", "martedì", "mercoledì", "giovedì", "venerdì", "sabato",
];

/// Format exact calendar-day distance in Italian, optionally with direction.
pub fn format_date_distance_strict_days_it(
    target: NaiveDate,
    reference: NaiveDate,
    with_suffix: bool,
) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    let count = delta.abs();
    let noun = if count == 1 { "giorno" } else { "giorni" };
    let label = format!("{count} {noun}");
    if !with_suffix || delta == 0 {
        label
    } else if delta > 0 {
        format!("tra {label}")
    } else {
        format!("{label} fa")
    }
}

/// Format an approximate Italian distance with rounded calendar-length units.
pub fn format_date_distance_it(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => return "oggi".to_owned(),
        1 => return "domani".to_owned(),
        -1 => return "ieri".to_owned(),
        _ => {}
    }
    let days = delta.abs();
    let (count, singular, plural) = if days < 7 {
        (days, "giorno", "giorni")
    } else if days < 28 {
        ((days + 3) / 7, "settimana", "settimane")
    } else if days < 365 {
        ((days + 15) / 30, "mese", "mesi")
    } else {
        ((days + 182) / 365, "anno", "anni")
    };
    let noun = if count == 1 { singular } else { plural };
    let label = format!("{count} {noun}");
    if delta > 0 {
        format!("tra {label}")
    } else {
        format!("{label} fa")
    }
}

/// Label nearby dates in Italian with gender-aware Sunday wording.
pub fn format_relative_calendar_date_it(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    let sunday = target.weekday() == Weekday::Sun;
    match delta {
        0 => "oggi".to_owned(),
        1 => "domani".to_owned(),
        -1 => "ieri".to_owned(),
        2..=6 => {
            let day = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            let suffix = if sunday { "prossima" } else { "prossimo" };
            format!("{day} {suffix}")
        }
        -6..=-2 => {
            let day = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            let suffix = if sunday { "scorsa" } else { "scorso" };
            format!("{day} {suffix}")
        }
        _ => crate::format_iso_date(target).unwrap_or_else(|| target.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn formats_exact_and_approximate_distances() {
        let start = date(2024, 2, 28);
        let end = date(2024, 3, 1);
        assert_eq!(
            format_date_distance_strict_days_it(end, start, true),
            "tra 2 giorni"
        );
        assert_eq!(
            format_date_distance_strict_days_it(start, end, true),
            "2 giorni fa"
        );
        assert_eq!(
            format_date_distance_strict_days_it(start, start, false),
            "0 giorni"
        );
        assert_eq!(
            format_date_distance_it(date(2024, 1, 8), date(2024, 1, 1)),
            "tra 1 settimana"
        );
        assert_eq!(
            format_date_distance_it(date(2024, 1, 31), date(2024, 1, 1)),
            "tra 1 mese"
        );
        assert_eq!(
            format_date_distance_it(date(2025, 1, 1), date(2024, 1, 1)),
            "tra 1 anno"
        );
    }

    #[test]
    fn formats_masculine_and_feminine_weekdays() {
        let monday = date(2024, 12, 30);
        assert_eq!(format_relative_calendar_date_it(monday, monday), "oggi");
        assert_eq!(
            format_relative_calendar_date_it(date(2024, 12, 31), monday),
            "domani"
        );
        assert_eq!(
            format_relative_calendar_date_it(date(2024, 12, 29), monday),
            "ieri"
        );
        assert_eq!(
            format_relative_calendar_date_it(date(2025, 1, 3), monday),
            "venerdì prossimo"
        );
        assert_eq!(
            format_relative_calendar_date_it(date(2024, 12, 27), monday),
            "venerdì scorso"
        );
        assert_eq!(
            format_relative_calendar_date_it(date(2025, 1, 5), monday),
            "domenica prossima"
        );
        assert_eq!(
            format_relative_calendar_date_it(date(2025, 1, 20), monday),
            "2025-01-20"
        );
    }
}
