//! Strict ISO 8601 basic week dates in YYYYWwwd form.

use chrono::{Datelike, NaiveDate, Weekday};

/// Parse an exactly eight-byte ISO basic week date, such as 2020W535.
///
/// ISO weekdays are Monday (1) through Sunday (7). Invalid week 53
/// values, expanded years, spaces and noncanonical separators return None.
pub fn parse_iso_basic_week_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 8
        || bytes[4] != b'W'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..7].iter().all(u8::is_ascii_digit)
    {
        return None;
    }

    let weekday = match bytes[7] {
        b'1' => Weekday::Mon,
        b'2' => Weekday::Tue,
        b'3' => Weekday::Wed,
        b'4' => Weekday::Thu,
        b'5' => Weekday::Fri,
        b'6' => Weekday::Sat,
        b'7' => Weekday::Sun,
        _ => return None,
    };
    let year = input[..4].parse::<i32>().ok()?;
    let week = input[5..7].parse::<u32>().ok()?;
    NaiveDate::from_isoywd_opt(year, week, weekday)
}

/// Format a date as YYYYWwwd using its ISO week-numbering year.
///
/// Returns None if the week year is outside the four-digit range 0000..=9999.
pub fn format_iso_basic_week_date(date: NaiveDate) -> Option<String> {
    let week = date.iso_week();
    if !(0..=9999).contains(&week.year()) {
        return None;
    }
    Some(format!(
        "{:04}W{:02}{}",
        week.year(),
        week.week(),
        date.weekday().number_from_monday()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn round_trips_year_transitions_and_week_53() {
        let new_year = date(2021, 1, 1);
        assert_eq!(format_iso_basic_week_date(new_year).as_deref(), Some("2020W535"));
        assert_eq!(parse_iso_basic_week_date("2020W535"), Some(new_year));
        assert_eq!(parse_iso_basic_week_date("2020W537"), Some(date(2021, 1, 3)));
        assert_eq!(
            format_iso_basic_week_date(date(2018, 12, 31)).as_deref(),
            Some("2019W011")
        );
    }

    #[test]
    fn rejects_invalid_and_noncanonical_week_dates() {
        for value in [
            "2021W531",
            "2024W001",
            "2024W010",
            "2024W018",
            "2024w011",
            "2024-W01-1",
            "2024W011 ",
            "2024W01",
            "+2024W011",
            "2024W0a1",
        ] {
            assert_eq!(parse_iso_basic_week_date(value), None, "{value}");
        }
    }

    #[test]
    fn supports_zero_year_and_rejects_expanded_output() {
        let day = date(0, 6, 1);
        let text = format_iso_basic_week_date(day).unwrap();
        assert_eq!(parse_iso_basic_week_date(&text), Some(day));
        assert_eq!(format_iso_basic_week_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_basic_week_date(NaiveDate::MAX), None);
    }
}
