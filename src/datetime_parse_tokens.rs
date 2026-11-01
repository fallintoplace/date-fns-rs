//! Strict numeric token parsing for naive datetimes without time zone guessing.

use chrono::{NaiveDate, NaiveDateTime};

fn digits(input: &str, index: &mut usize, min: usize, max: usize) -> Option<u32> {
    let bytes = input.as_bytes();
    let start = *index;
    while *index < bytes.len()
        && *index - start < max
        && bytes[*index].is_ascii_digit()
    {
        *index += 1;
    }
    if *index - start < min {
        return None;
    }
    input[start..*index].parse().ok()
}

/// Parse a full date and optional clock fields from numeric date-fns style tokens.
///
/// Supported: yyyy, MM, M, dd, d, HH, H, mm, m, ss, s, SSS.
/// Date fields are required. Unspecified clock fields default to zero.
/// Doubled single quotes represent literal apostrophes. Unsupported tokens,
/// duplicate fields, invalid dates or clock values, and trailing data fail.
pub fn parse_datetime_tokens(input: &str, pattern: &str) -> Option<NaiveDateTime> {
    let mut pi = 0;
    let mut ii = 0;
    let mut year = None;
    let mut month = None;
    let mut day = None;
    let mut hour = None;
    let mut minute = None;
    let mut second = None;
    let mut millisecond = None;
    let bytes = pattern.as_bytes();

    while pi < bytes.len() {
        if bytes[pi] == b'\'' && bytes.get(pi + 1) == Some(&b'\'') {
            if !input[ii..].starts_with('\'') {
                return None;
            }
            ii += 1;
            pi += 2;
            continue;
        }
        if bytes[pi] == b'\'' {
            pi += 1;
            let mut closed = false;
            while pi < bytes.len() {
                if bytes[pi] == b'\'' {
                    if bytes.get(pi + 1) == Some(&b'\'') {
                        if !input[ii..].starts_with('\'') {
                            return None;
                        }
                        ii += 1;
                        pi += 2;
                    } else {
                        pi += 1;
                        closed = true;
                        break;
                    }
                } else {
                    let ch = pattern[pi..].chars().next()?;
                    if !input[ii..].starts_with(ch) {
                        return None;
                    }
                    ii += ch.len_utf8();
                    pi += ch.len_utf8();
                }
            }
            if !closed {
                return None;
            }
            continue;
        }

        let rest = &pattern[pi..];
        let token = [
            "yyyy", "SSS", "MM", "dd", "HH", "mm", "ss", "M", "d", "H", "m", "s",
        ]
        .iter()
        .find(|token| rest.starts_with(**token))
        .copied();
        if let Some(token) = token {
            let (min, max) = match token {
                "yyyy" => (4, 4),
                "SSS" => (3, 3),
                "MM" | "dd" | "HH" | "mm" | "ss" => (2, 2),
                _ => (1, 2),
            };
            let parsed = digits(input, &mut ii, min, max)?;
            let target = match token {
                "yyyy" => &mut year,
                "MM" | "M" => &mut month,
                "dd" | "d" => &mut day,
                "HH" | "H" => &mut hour,
                "mm" | "m" => &mut minute,
                "ss" | "s" => &mut second,
                "SSS" => &mut millisecond,
                _ => return None,
            };
            if target.replace(parsed).is_some() {
                return None;
            }
            pi += token.len();
        } else {
            let ch = rest.chars().next()?;
            if ch.is_ascii_alphabetic() || !input[ii..].starts_with(ch) {
                return None;
            }
            ii += ch.len_utf8();
            pi += ch.len_utf8();
        }
    }

    if ii != input.len() {
        return None;
    }
    let date = NaiveDate::from_ymd_opt(year? as i32, month?, day?)?;
    date.and_hms_milli_opt(
        hour.unwrap_or(0),
        minute.unwrap_or(0),
        second.unwrap_or(0),
        millisecond.unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_leap_days_and_fractional_clock_values() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let expected = date.and_hms_milli_opt(9, 5, 6, 7).unwrap();
        assert_eq!(
            parse_datetime_tokens("2024-02-29T09:05:06.007", "yyyy-MM-dd'T'HH:mm:ss.SSS"),
            Some(expected)
        );
        assert_eq!(
            parse_datetime_tokens("2/29/2024 9:5:6", "M/d/yyyy H:m:s"),
            Some(date.and_hms_opt(9, 5, 6).unwrap())
        );
        assert_eq!(
            parse_datetime_tokens("2024-02-29", "yyyy-MM-dd"),
            date.and_hms_opt(0, 0, 0)
        );
        assert_eq!(
            parse_datetime_tokens("2024'02'29", "yyyy''MM''dd"),
            date.and_hms_opt(0, 0, 0)
        );
    }

    #[test]
    fn rejects_invalid_or_incomplete_inputs_and_duplicate_fields() {
        for invalid in [
            "2023-02-29T09:05:06.007",
            "2024-02-29T24:05:06.007",
            "2024-02-29T09:60:06.007",
            "2024-02-29T09:05:61.007",
            "2024-02-29T09:05:06.999Z",
        ] {
            assert_eq!(
                parse_datetime_tokens(invalid, "yyyy-MM-dd'T'HH:mm:ss.SSS"),
                None
            );
        }
        assert_eq!(parse_datetime_tokens("2024-02-29 02", "yyyy-MM-dd MM"), None);
        assert_eq!(parse_datetime_tokens("2024-02-29", "yyyy-MM-QQ"), None);
        assert_eq!(parse_datetime_tokens("23:59", "HH:mm"), None);
        assert_eq!(parse_datetime_tokens("2024-02-29", "'unfinished"), None);
    }
}
