//! Spanish relative calendar dates and approximate date-distance wording.

use chrono::{Datelike, NaiveDate};

const WEEKDAYS: [&str; 7] = [
    "domingo", "lunes", "martes", "miércoles", "jueves", "viernes", "sábado",
];

/// Format exact calendar-day distance in Spanish, optionally with direction.
pub fn format_date_distance_strict_days_es(
    target: NaiveDate,
    reference: NaiveDate,
    with_suffix: bool,
) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    let count = delta.abs();
    let noun = if count == 1 { "día" } else { "días" };
    let label = format!("{count} {noun}");
    if !with_suffix || delta == 0 {
        label
    } else if delta > 0 {
        format!("en {label}")
    } else {
        format!("hace {label}")
    }
}

/// Format a Spanish approximate distance with 7-, 30- and 365-day units.
pub fn format_date_distance_es(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => return "hoy".to_owned(),
        1 => return "mañana".to_owned(),
        -1 => return "ayer".to_owned(),
        _ => {}
    }
    let days = delta.abs();
    let (count, singular, plural) = if days < 7 {
        (days, "día", "días")
    } else if days < 28 {
        ((days + 3) / 7, "semana", "semanas")
    } else if days < 365 {
        ((days + 15) / 30, "mes", "meses")
    } else {
        ((days + 182) / 365, "año", "años")
    };
    let noun = if count == 1 { singular } else { plural };
    let label = format!("{count} {noun}");
    if delta > 0 {
        format!("en {label}")
    } else {
        format!("hace {label}")
    }
}

/// Label nearby dates in Spanish, falling back to ISO for distant dates.
pub fn format_relative_calendar_date_es(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => "hoy".to_owned(),
        1 => "mañana".to_owned(),
        -1 => "ayer".to_owned(),
        2..=6 => {
            let day = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            format!("el próximo {day}")
        }
        -6..=-2 => {
            let day = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            format!("el {day} pasado")
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
            format_date_distance_strict_days_es(end, start, true),
            "en 2 días"
        );
        assert_eq!(
            format_date_distance_strict_days_es(start, end, true),
            "hace 2 días"
        );
        assert_eq!(
            format_date_distance_strict_days_es(start, start, false),
            "0 días"
        );
        assert_eq!(
            format_date_distance_es(date(2024, 1, 8), date(2024, 1, 1)),
            "en 1 semana"
        );
        assert_eq!(
            format_date_distance_es(date(2024, 1, 31), date(2024, 1, 1)),
            "en 1 mes"
        );
        assert_eq!(
            format_date_distance_es(date(2025, 1, 1), date(2024, 1, 1)),
            "en 1 año"
        );
    }

    #[test]
    fn formats_relative_labels_and_weekday_accents() {
        let monday = date(2024, 12, 30);
        assert_eq!(format_relative_calendar_date_es(monday, monday), "hoy");
        assert_eq!(
            format_relative_calendar_date_es(date(2024, 12, 31), monday),
            "mañana"
        );
        assert_eq!(
            format_relative_calendar_date_es(date(2024, 12, 29), monday),
            "ayer"
        );
        assert_eq!(
            format_relative_calendar_date_es(date(2025, 1, 3), monday),
            "el próximo viernes"
        );
        assert_eq!(
            format_relative_calendar_date_es(date(2024, 12, 27), monday),
            "el viernes pasado"
        );
        assert_eq!(
            format_relative_calendar_date_es(date(2025, 1, 20), monday),
            "2025-01-20"
        );
    }
}
