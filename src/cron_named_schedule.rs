//! Month and weekday aliases for strict five-field cron schedules.

use crate::CronSchedule;

const MONTHS: [(&str, &str); 12] = [
    ("JAN", "1"),
    ("FEB", "2"),
    ("MAR", "3"),
    ("APR", "4"),
    ("MAY", "5"),
    ("JUN", "6"),
    ("JUL", "7"),
    ("AUG", "8"),
    ("SEP", "9"),
    ("OCT", "10"),
    ("NOV", "11"),
    ("DEC", "12"),
];

const WEEKDAYS: [(&str, &str); 7] = [
    ("SUN", "7"),
    ("MON", "1"),
    ("TUE", "2"),
    ("WED", "3"),
    ("THU", "4"),
    ("FRI", "5"),
    ("SAT", "6"),
];

fn normalize_field(input: &str, names: &[(&str, &str)]) -> Option<String> {
    if !input.is_ascii() {
        return None;
    }
    let bytes = input.as_bytes();
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_alphabetic() {
            if cursor > 0 && bytes[cursor - 1].is_ascii_digit() {
                return None;
            }
            let start = cursor;
            while cursor < bytes.len() && bytes[cursor].is_ascii_alphabetic() {
                cursor += 1;
            }
            if cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
                return None;
            }
            let token = &input[start..cursor];
            let (_, number) = names
                .iter()
                .find(|(alias, _)| alias.eq_ignore_ascii_case(token))?;
            output.push_str(number);
        } else if bytes[cursor].is_ascii_digit()
            || matches!(bytes[cursor], b'*' | b'-' | b'/' | b',')
        {
            output.push(char::from(bytes[cursor]));
            cursor += 1;
        } else {
            return None;
        }
    }
    Some(output)
}

/// Parse a five-field cron expression with optional three-letter English names.
///
/// Only the month and weekday fields support names. Names are ASCII and
/// case-insensitive; they work in lists and non-wrapping ranges. The numeric
/// parser supplies the same steps, weekday OR rules and validation.
pub fn parse_cron_schedule_named(text: &str) -> Option<CronSchedule> {
    let fields: Vec<_> = text.split_whitespace().collect();
    if fields.len() != 5 {
        return None;
    }
    let months = normalize_field(fields[3], &MONTHS)?;
    let weekdays = normalize_field(fields[4], &WEEKDAYS)?;
    let numeric = format!("{} {} {} {} {}", fields[0], fields[1], fields[2], months, weekdays);
    crate::parse_cron_schedule(&numeric)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn stamp(y: i32, m: u32, d: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap()
    }

    #[test]
    fn parses_case_insensitive_month_and_weekday_ranges() {
        let named = parse_cron_schedule_named("0 9 * jan-Mar Mon-fri").unwrap();
        let numeric = crate::parse_cron_schedule("0 9 * 1-3 1-5").unwrap();
        assert_eq!(named, numeric);
        assert!(named.matches(stamp(2024, 1, 1)));
        assert!(!named.matches(stamp(2024, 1, 6)));
        assert!(!named.matches(stamp(2024, 4, 1)));
    }

    #[test]
    fn supports_sunday_as_a_list_item_or_range_endpoint() {
        let every_day = parse_cron_schedule_named("0 9 * * MON-SUN").unwrap();
        assert!(every_day.matches(stamp(2024, 1, 7)));
        assert!(every_day.matches(stamp(2024, 1, 8)));
        let sundays = parse_cron_schedule_named("0 9 * * SUN-SUN").unwrap();
        assert!(sundays.matches(stamp(2024, 1, 7)));
        assert!(!sundays.matches(stamp(2024, 1, 8)));
        let weekends = parse_cron_schedule_named("0 9 * * sat,sun").unwrap();
        assert!(weekends.matches(stamp(2024, 1, 6)));
        assert!(weekends.matches(stamp(2024, 1, 7)));
    }

    #[test]
    fn rejects_malformed_names_and_preserves_numeric_validation() {
        for input in [
            "0 9 * FOO MON",
            "0 9 * JAN MONDAY",
            "0 9 * JAN2 MON",
            "0 9 * JAN MON3",
            "0 9 * * SUN-MON",
            "0 9 * * MON-FOO",
            "0 9 * JAN",
            "@daily",
            "0 9 * * MÖN",
            "60 9 * JAN MON",
        ] {
            assert!(parse_cron_schedule_named(input).is_none(), "{input}");
        }
    }
}
