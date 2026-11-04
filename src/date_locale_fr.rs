//! French relative calendar dates and approximate date-distance wording.

use chrono::{Datelike, NaiveDate};

const WEEKDAYS: [&str; 7] = [
    "dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi",
];

/// Format exact calendar-day distance in French, optionally with direction.
pub fn format_date_distance_strict_days_fr(
    target: NaiveDate,
    reference: NaiveDate,
    with_suffix: bool,
) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    let count = delta.abs();
    let noun = if count == 1 { "jour" } else { "jours" };
    let label = format!("{count} {noun}");
    if !with_suffix || delta == 0 {
        label
    } else if delta > 0 {
        format!("dans {label}")
    } else {
        format!("il y a {label}")
    }
}

/// Format an approximate French distance using rounded week, month and year units.
///
/// Uses 7, 30 and 365 days rather than exact calendar periods. No clock or
/// locale is read from the environment.
pub fn format_date_distance_fr(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => return "aujourd'hui".to_owned(),
        1 => return "demain".to_owned(),
        -1 => return "hier".to_owned(),
        _ => {}
    }

    let days = delta.abs();
    let (count, singular, plural) = if days < 7 {
        (days, "jour", "jours")
    } else if days < 28 {
        ((days + 3) / 7, "semaine", "semaines")
    } else if days < 365 {
        ((days + 15) / 30, "mois", "mois")
    } else {
        ((days + 182) / 365, "an", "ans")
    };
    let noun = if count == 1 { singular } else { plural };
    let label = format!("{count} {noun}");
    if delta > 0 {
        format!("dans {label}")
    } else {
        format!("il y a {label}")
    }
}

/// Format near dates as French relative labels and distant dates as ISO dates.
pub fn format_relative_calendar_date_fr(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => "aujourd'hui".to_owned(),
        1 => "demain".to_owned(),
        -1 => "hier".to_owned(),
        2..=6 => {
            let day = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            format!("{day} prochain")
        }
        -6..=-2 => {
            let day = WEEKDAYS[target.weekday().num_days_from_sunday() as usize];
            format!("{day} dernier")
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
    fn formats_direction_and_french_plural_words() {
        let start = date(2024, 2, 28);
        let end = date(2024, 3, 1);
        assert_eq!(
            format_date_distance_strict_days_fr(end, start, true),
            "dans 2 jours"
        );
        assert_eq!(
            format_date_distance_strict_days_fr(start, end, true),
            "il y a 2 jours"
        );
        assert_eq!(
            format_date_distance_strict_days_fr(start, start, true),
            "0 jours"
        );
        assert_eq!(
            format_date_distance_fr(date(2024, 1, 8), date(2024, 1, 1)),
            "dans 1 semaine"
        );
        assert_eq!(
            format_date_distance_fr(date(2024, 1, 31), date(2024, 1, 1)),
            "dans 1 mois"
        );
        assert_eq!(
            format_date_distance_fr(date(2025, 1, 1), date(2024, 1, 1)),
            "dans 1 an"
        );
    }

    #[test]
    fn labels_today_adjacent_days_and_weekdays() {
        let monday = date(2024, 12, 30);
        assert_eq!(
            format_relative_calendar_date_fr(monday, monday),
            "aujourd'hui"
        );
        assert_eq!(
            format_relative_calendar_date_fr(date(2024, 12, 31), monday),
            "demain"
        );
        assert_eq!(
            format_relative_calendar_date_fr(date(2024, 12, 29), monday),
            "hier"
        );
        assert_eq!(
            format_relative_calendar_date_fr(date(2025, 1, 3), monday),
            "vendredi prochain"
        );
        assert_eq!(
            format_relative_calendar_date_fr(date(2024, 12, 27), monday),
            "vendredi dernier"
        );
        assert_eq!(
            format_relative_calendar_date_fr(date(2025, 1, 20), monday),
            "2025-01-20"
        );
    }
}
