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

/// Format date-only calendar values using common date-fns tokens.
/// Supported: yyyy, yy, MMMM, MMM, MM, M, dd, d, EEEE, EEE, E.
/// Use single quotes to escape literal text, doubling a quote inside it.
/// Unsupported tokens, unclosed quotes and years outside 0000-9999 return None.
pub fn format_date_tokens(date: NaiveDate, pattern: &str) -> Option<String> {
    let mut result = String::new();
    let mut i = 0;
    let bytes = pattern.as_bytes();
    while i < bytes.len() {
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
        let token = [
            "yyyy", "MMMM", "EEEE", "MMM", "EEE", "yy", "MM", "dd", "M", "d", "E",
        ]
        .iter()
        .find(|token| rest.starts_with(**token))
        .copied();
        if let Some(token) = token {
            let month = (date.month() - 1) as usize;
            let weekday = date.weekday().num_days_from_sunday() as usize;
            let value = match token {
                "yyyy" => {
                    if !(0..=9999).contains(&date.year()) {
                        return None;
                    }
                    format!("{:04}", date.year())
                }
                "yy" => format!("{:02}", date.year().rem_euclid(100)),
                "MMMM" => MONTHS[month].to_owned(),
                "MMM" => MONTHS[month][..3].to_owned(),
                "MM" => format!("{:02}", date.month()),
                "M" => date.month().to_string(),
                "dd" => format!("{:02}", date.day()),
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
        assert_eq!(format_date_tokens(day, "QQ"), None);
        assert_eq!(format_date_tokens(day, "'unfinished"), None);
        assert_eq!(format_date_tokens(NaiveDate::MAX, "yyyy"), None);
    }
}
