//! A strict subset of date-fns style calendar formatting tokens.
//!
//! English names are fixed; unsupported alphabetic tokens return None.

use chrono::{Datelike, NaiveDate};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

fn english_ordinal(number: u32) -> String {
    let suffix = if (11..=13).contains(&(number % 100)) {
        "th"
    } else {
        match number % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("{number}{suffix}")
}

/// Format date-only calendar values using common date-fns tokens.
/// Supported: yyyy, yy, QQQQ, QQQ, QQ, Qo, Q, MMMM, MMM, MM, M, Mo,
/// dd, d, do, EEEE, EEE, E.
/// Use single quotes to escape literal text, doubling a quote inside it.
/// Unsupported tokens, unclosed quotes and years outside 0000-9999 return None.
pub fn format_date_tokens(date: NaiveDate, pattern: &str) -> Option<String> {
    let mut result = String::new();
    let mut i = 0;
    let bytes = pattern.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'\'' && bytes.get(i + 1) == Some(&b'\'') {
            result.push('\'');
            i += 2;
            continue;
        }
        if bytes[i] == b'\'' {
            i += 1;
            let mut closed = false;
            while i < bytes.len() {
                if bytes[i] == b'\'' {
                    if bytes.get(i + 1) == Some(&b'\'') {
                        result.push('\'');
                        i += 2;
                    } else {
                        i += 1;
                        closed = true;
                        break;
                    }
                } else {
                    let ch = pattern[i..].chars().next()?;
                    result.push(ch);
                    i += ch.len_utf8();
                }
            }
            if !closed {
                return None;
            }
            continue;
        }
        let rest = &pattern[i..];
        if rest.starts_with("QQQQQ") {
            return None;
        }
        let token = [
            "yyyy", "QQQQ", "MMMM", "EEEE", "QQQ", "MMM", "EEE", "yy", "QQ", "Qo", "MM", "dd",
            "Mo", "do", "Q", "M", "d", "E",
        ]
        .iter()
        .find(|token| rest.starts_with(**token))
        .copied();
        if let Some(token) = token {
            let month = (date.month() - 1) as usize;
            let quarter = (date.month() - 1) / 3 + 1;
            let weekday = date.weekday().num_days_from_sunday() as usize;
            let value = match token {
                "yyyy" => {
                    if !(0..=9999).contains(&date.year()) {
                        return None;
                    }
                    format!("{:04}", date.year())
                }
                "yy" => format!("{:02}", date.year().rem_euclid(100)),
                "Q" => quarter.to_string(),
                "QQ" => format!("{quarter:02}"),
                "Qo" => english_ordinal(quarter),
                "QQQ" => format!("Q{quarter}"),
                "QQQQ" => format!("{} quarter", english_ordinal(quarter)),
                "MMMM" => MONTHS[month].to_owned(),
                "MMM" => MONTHS[month][..3].to_owned(),
                "MM" => format!("{:02}", date.month()),
                "Mo" => english_ordinal(date.month()),
                "M" => date.month().to_string(),
                "dd" => format!("{:02}", date.day()),
                "do" => english_ordinal(date.day()),
                "d" => date.day().to_string(),
                "EEEE" => WEEKDAYS[weekday].to_owned(),
                "EEE" | "E" => WEEKDAYS[weekday][..3].to_owned(),
                _ => return None,
            };
            result.push_str(&value);
            i += token.len();
        } else {
            let ch = rest.chars().next()?;
            if ch.is_ascii_alphabetic() {
                return None;
            }
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_quarters_with_date_fns_tokens() {
        let first = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let last = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        assert_eq!(
            format_date_tokens(first, "Q QQ Qo QQQ QQQQ").as_deref(),
            Some("1 01 1st Q1 1st quarter")
        );
        assert_eq!(
            format_date_tokens(last, "Q QQ Qo QQQ QQQQ").as_deref(),
            Some("4 04 4th Q4 4th quarter")
        );
        assert_eq!(format_date_tokens(first, "'Q' Q"), Some("Q 1".to_string()));
        assert_eq!(format_date_tokens(first, "QQQQQ"), None);
    }

    #[test]
    fn formats_common_date_tokens_and_quoted_text() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            format_date_tokens(day, "yyyy-MM-dd").as_deref(),
            Some("2024-02-29")
        );
        assert_eq!(
            format_date_tokens(day, "EEEE, MMM d, yyyy").as_deref(),
            Some("Thursday, Feb 29, 2024")
        );
        assert_eq!(
            format_date_tokens(day, "'day' dd 'o''clock'").as_deref(),
            Some("day 29 o'clock")
        );
        assert_eq!(
            format_date_tokens(day, "M/d/yy").as_deref(),
            Some("2/29/24")
        );
    }

    #[test]
    fn rejects_unsupported_tokens_and_unclosed_literals() {
        let day = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(format_date_tokens(day, "PP"), None);
        assert_eq!(format_date_tokens(day, "'unfinished"), None);
        assert_eq!(format_date_tokens(NaiveDate::MAX, "yyyy"), None);
    }

    #[test]
    fn formats_english_day_and_month_ordinals() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            format_date_tokens(date, "MMM do, yyyy").as_deref(),
            Some("Feb 29th, 2024")
        );
        assert_eq!(
            format_date_tokens(date, "'month' Mo, 'day' do").as_deref(),
            Some("month 2nd, day 29th")
        );
        for (day, label) in [
            (1, "1st"),
            (2, "2nd"),
            (3, "3rd"),
            (4, "4th"),
            (11, "11th"),
            (12, "12th"),
            (13, "13th"),
            (21, "21st"),
            (22, "22nd"),
            (23, "23rd"),
            (31, "31st"),
        ] {
            let day = NaiveDate::from_ymd_opt(2024, 1, day).unwrap();
            assert_eq!(format_date_tokens(day, "do").as_deref(), Some(label));
        }
    }

    #[test]
    fn formats_doubled_apostrophes_outside_quoted_literals() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            format_date_tokens(date, "yyyy''MM''dd").as_deref(),
            Some("2024'02'29")
        );
        assert_eq!(
            format_date_tokens(date, "'''' yyyy").as_deref(),
            Some("'' 2024")
        );
    }

    #[test]
    fn doubled_quote_formats_a_literal_apostrophe() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            format_date_tokens(date, "yyyy''MM''dd").as_deref(),
            Some("2024'02'29")
        );
        assert_eq!(format_date_tokens(date, "''").as_deref(), Some("'"));
        assert_eq!(format_date_tokens(date, "'it''s'").as_deref(), Some("it's"));
    }
}
