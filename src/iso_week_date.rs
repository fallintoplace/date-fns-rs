//! Strict ISO 8601 week dates in the form YYYY-Www-d.

use chrono::{Datelike, NaiveDate, Weekday};

/// Parse an exactly ten-byte ISO week date, such as 2020-W53-5.
///
/// Week days run from Monday (1) to Sunday (7). Invalid week 53 dates,
/// expanded years, and noncanonical separators return None.
pub fn parse_iso_week_date(input: &str) -> Option<NaiveDate> {
    let bytes = input.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[5] != b'W'
        || bytes[8] != b'-'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[6..8].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let weekday = match bytes[9] {
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
    let week = input[6..8].parse::<u32>().ok()?;
    NaiveDate::from_isoywd_opt(year, week, weekday)
}

/// Format a date as YYYY-Www-d if its ISO week year has four digits.
///
/// The ISO week year can differ from the calendar year around New Year.
pub fn format_iso_week_date(date: NaiveDate) -> Option<String> {
    let week = date.iso_week();
    if !(0..=9999).contains(&week.year()) {
        return None;
    }
    Some(format!(
        "{:04}-W{:02}-{}",
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
    fn handles_iso_week_year_boundaries_and_week_53() {
        assert_eq!(format_iso_week_date(date(2021, 1, 1)).as_deref(), Some("2020-W53-5"));
        assert_eq!(parse_iso_week_date("2020-W53-5"), Some(date(2021, 1, 1)));
        assert_eq!(format_iso_week_date(date(2018, 12, 31)).as_deref(), Some("2019-W01-1"));
        assert_eq!(parse_iso_week_date("2020-W53-7"), Some(date(2021, 1, 3)));
    }

    #[test]
    fn rejects_bad_week_numbers_and_noncanonical_input() {
        for value in [
            "2021-W53-1",
            "2024-W00-1",
            "2024-W01-0",
            "2024-W01-8",
            "2024-w01-1",
            "2024-W1-1",
            "2024-W01-1 ",
            "+10000-W01-1",
        ] {
            assert_eq!(parse_iso_week_date(value), None, "{value}");
        }
    }

    #[test]
    fn supports_year_zero_but_not_expanded_year_output() {
        let value = date(0, 6, 1);
        let formatted = format_iso_week_date(value).unwrap();
        assert_eq!(parse_iso_week_date(&formatted), Some(value));
        assert_eq!(format_iso_week_date(NaiveDate::MIN), None);
        assert_eq!(format_iso_week_date(NaiveDate::MAX), None);
    }
}
