//! German date-distance wording without implicit clock or locale state.

use chrono::NaiveDate;

/// Format an exact signed number of calendar days in German.
///
/// Without a suffix this returns "1 Tag" or "2 Tage"; with a suffix it
/// returns "in 2 Tagen" or "vor 2 Tagen".
pub fn format_date_distance_strict_days_de(
    target: NaiveDate,
    reference: NaiveDate,
    with_suffix: bool,
) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    let days = delta.abs();
    if !with_suffix || delta == 0 {
        return format!("{days} {}", if days == 1 { "Tag" } else { "Tage" });
    }
    let label = format!("{days} {}", if days == 1 { "Tag" } else { "Tagen" });
    if delta > 0 {
        format!("in {label}")
    } else {
        format!("vor {label}")
    }
}

/// Format approximate distance using fixed 7-, 30- and 365-day units.
///
/// Calendar labels are intentionally approximate. Use the strict variant
/// for exact day counts. This function never reads the current time.
pub fn format_date_distance_de(target: NaiveDate, reference: NaiveDate) -> String {
    let delta = crate::difference_in_calendar_days(target, reference);
    match delta {
        0 => return "heute".to_owned(),
        1 => return "morgen".to_owned(),
        -1 => return "gestern".to_owned(),
        _ => {}
    }
    let days = delta.abs();
    let (count, singular, plural) = if days < 7 {
        (days, "Tag", "Tagen")
    } else if days < 28 {
        ((days + 3) / 7, "Woche", "Wochen")
    } else if days < 365 {
        ((days + 15) / 30, "Monat", "Monaten")
    } else {
        ((days + 182) / 365, "Jahr", "Jahren")
    };
    let label = format!("{count} {}", if count == 1 { singular } else { plural });
    if delta > 0 {
        format!("in {label}")
    } else {
        format!("vor {label}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn distinguishes_today_tomorrow_and_yesterday() {
        let origin = date(2024, 2, 29);
        assert_eq!(format_date_distance_de(origin, origin), "heute");
        assert_eq!(format_date_distance_de(date(2024, 3, 1), origin), "morgen");
        assert_eq!(format_date_distance_de(date(2024, 2, 28), origin), "gestern");
    }

    #[test]
    fn uses_german_suffix_cases_for_exact_days() {
        let origin = date(2024, 2, 28);
        let target = date(2024, 3, 1);
        assert_eq!(format_date_distance_strict_days_de(target, origin, false), "2 Tage");
        assert_eq!(format_date_distance_strict_days_de(target, origin, true), "in 2 Tagen");
        assert_eq!(format_date_distance_strict_days_de(origin, target, true), "vor 2 Tagen");
        assert_eq!(format_date_distance_strict_days_de(origin, origin, true), "0 Tage");
    }

    #[test]
    fn formats_week_month_and_year_distances() {
        let day = date(2024, 1, 1);
        assert_eq!(format_date_distance_de(date(2024, 1, 8), day), "in 1 Woche");
        assert_eq!(format_date_distance_de(date(2024, 1, 31), day), "in 1 Monat");
        assert_eq!(format_date_distance_de(date(2025, 1, 1), day), "in 1 Jahr");
        assert_eq!(format_date_distance_de(day, date(2024, 1, 15)), "vor 2 Wochen");
    }
}
