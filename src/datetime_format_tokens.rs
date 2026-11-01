//! Date-fns style numeric datetime formatting without implicit time zones.

use chrono::{Datelike, NaiveDateTime, Timelike};

/// Format a naive datetime with numeric date and clock tokens.
///
/// Supported: yyyy, MM, M, dd, d, HH, H, mm, m, ss, s, SSS.
/// SSS truncates nanoseconds to whole milliseconds. Literal text uses single
/// quotes, doubled to escape an apostrophe. Unsupported alphabetic tokens,
/// unclosed quotes and yyyy years outside 0000-9999 return None.
pub fn format_datetime_tokens(value: NaiveDateTime, pattern: &str) -> Option<String> {
    let mut result = String::new();
    let bytes = pattern.as_bytes();
    let mut i = 0;

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
        let token = [
            "yyyy", "SSS", "MM", "dd", "HH", "mm", "ss", "M", "d", "H", "m", "s",
        ]
        .iter()
        .find(|token| rest.starts_with(**token))
        .copied();

        if let Some(token) = token {
            let formatted = match token {
                "yyyy" => {
                    if !(0..=9999).contains(&value.year()) {
                        return None;
                    }
                    format!("{:04}", value.year())
                }
                "MM" => format!("{:02}", value.month()),
                "M" => value.month().to_string(),
                "dd" => format!("{:02}", value.day()),
                "d" => value.day().to_string(),
                "HH" => format!("{:02}", value.hour()),
                "H" => value.hour().to_string(),
                "mm" => format!("{:02}", value.minute()),
                "m" => value.minute().to_string(),
                "ss" => format!("{:02}", value.second()),
                "s" => value.second().to_string(),
                "SSS" => format!("{:03}", value.nanosecond() / 1_000_000),
                _ => return None,
            };
            result.push_str(&formatted);
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
    use chrono::NaiveDate;

    #[test]
    fn prints_fixed_and_variable_width_fields() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_nano_opt(9, 5, 6, 7_123_456)
            .unwrap();
        assert_eq!(
            format_datetime_tokens(value, "yyyy-MM-dd'T'HH:mm:ss.SSS").as_deref(),
            Some("2024-02-29T09:05:06.007")
        );
        assert_eq!(
            format_datetime_tokens(value, "M/d/yyyy H:m:s").as_deref(),
            Some("2/29/2024 9:5:6")
        );
        assert_eq!(
            format_datetime_tokens(value, "''HH 'o''clock'").as_deref(),
            Some("'09 o'clock")
        );
    }

    #[test]
    fn rejects_unsupported_fields_and_unfinished_literals() {
        let value = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(format_datetime_tokens(value, "yyyy-QQ"), None);
        assert_eq!(format_datetime_tokens(value, "'unfinished"), None);
        assert_eq!(format_datetime_tokens(NaiveDateTime::MAX, "yyyy"), None);
        assert_eq!(format_datetime_tokens(value, "HH:mm:ss"), Some("00:00:00".to_owned()));
    }
}
