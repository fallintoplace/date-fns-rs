//! Strict numeric token parsing for naive datetimes without time zone guessing.

use chrono::{Datelike, NaiveDate, NaiveDateTime};

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

fn parse_english_name(input: &str, at: &mut usize, names: &[&str], full: bool) -> Option<u32> {
    let remaining = input.get(*at..)?;
    for (index, name) in names.iter().enumerate() {
        let token = if full { *name } else { &name[..3] };
        if remaining.starts_with(token) {
            *at += token.len();
            return Some(index as u32);
        }
    }
    None
}

fn digits(input: &str, index: &mut usize, min: usize, max: usize) -> Option<u32> {
    let bytes = input.as_bytes();
    let start = *index;
    while *index < bytes.len() && *index - start < max && bytes[*index].is_ascii_digit() {
        *index += 1;
    }
    if *index - start < min {
        return None;
    }
    input[start..*index].parse().ok()
}

/// Parse a full date and optional clock fields from numeric date-fns style tokens.
///
/// Supported: yyyy, MMMM, MMM, MM, M, dd, d, EEEE, EEE, HH, H,
/// hh, h, mm, m, ss, s, S through SSSSSSSSS, and uppercase AM/PM using a.
/// Date fields are required. A twelve-hour clock requires an AM/PM field.\n/// Unspecified clock fields default to zero.
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
    let mut nanosecond = None;
    let mut weekday = None;
    let mut pm = None;
    let mut twelve_hour = false;
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
        if rest.starts_with('S') {
            let width = rest.bytes().take_while(|c| *c == b'S').count();
            if width > 9 || nanosecond.is_some() {
                return None;
            }
            let fraction = digits(input, &mut ii, width, width)?;
            let scale = 10_u32.pow((9 - width) as u32);
            nanosecond = Some(fraction.checked_mul(scale)?);
            pi += width;
            continue;
        }
        if rest.starts_with("MMMM") || rest.starts_with("MMM") {
            let full = rest.starts_with("MMMM");
            let parsed = parse_english_name(input, &mut ii, &MONTHS, full)? + 1;
            if month.replace(parsed).is_some() {
                return None;
            }
            pi += if full { 4 } else { 3 };
            continue;
        }
        if rest.starts_with("EEEE") || rest.starts_with("EEE") {
            let full = rest.starts_with("EEEE");
            let parsed = parse_english_name(input, &mut ii, &WEEKDAYS, full)?;
            if weekday.replace(parsed).is_some() {
                return None;
            }
            pi += if full { 4 } else { 3 };
            continue;
        }
        if rest.starts_with('a') {
            if pm.is_some() {
                return None;
            }
            if input[ii..].starts_with("AM") {
                pm = Some(false);
                ii += 2;
            } else if input[ii..].starts_with("PM") {
                pm = Some(true);
                ii += 2;
            } else {
                return None;
            }
            pi += 1;
            continue;
        }
        let token = [
            "yyyy", "hh", "MM", "dd", "HH", "mm", "ss", "M", "d", "H", "h", "m", "s",
        ]
        .iter()
        .find(|token| rest.starts_with(**token))
        .copied();
        if let Some(token) = token {
            let (min, max) = match token {
                "yyyy" => (4, 4),
                "MM" | "dd" | "HH" | "hh" | "mm" | "ss" => (2, 2),
                _ => (1, 2),
            };
            let parsed = digits(input, &mut ii, min, max)?;
            let target = match token {
                "yyyy" => &mut year,
                "MM" | "M" => &mut month,
                "dd" | "d" => &mut day,
                "HH" | "H" | "hh" | "h" => &mut hour,
                "mm" | "m" => &mut minute,
                "ss" | "s" => &mut second,
                _ => return None,
            };
            if target.replace(parsed).is_some() {
                return None;
            }
            if matches!(token, "hh" | "h") {
                twelve_hour = true;
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
    if weekday.is_some_and(|name| date.weekday().num_days_from_sunday() != name) {
        return None;
    }
    let parsed_hour = if twelve_hour {
        let hour = hour?;
        if !(1..=12).contains(&hour) {
            return None;
        }
        (hour % 12) + if pm? { 12 } else { 0 }
    } else {
        if pm.is_some() {
            return None;
        }
        hour.unwrap_or(0)
    };
    date.and_hms_nano_opt(
        parsed_hour,
        minute.unwrap_or(0),
        second.unwrap_or(0),
        nanosecond.unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn parses_fractional_seconds_without_losing_nanoseconds() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        for (suffix, token, nanos) in [
            ("1", "S", 100_000_000),
            ("12", "SS", 120_000_000),
            ("123", "SSS", 123_000_000),
            ("123456", "SSSSSS", 123_456_000),
            ("123456789", "SSSSSSSSS", 123_456_789),
        ] {
            let input = format!("2024-02-29 12:34:56.{suffix}");
            let pattern = format!("yyyy-MM-dd HH:mm:ss.{token}");
            let result = parse_datetime_tokens(&input, &pattern).unwrap();
            assert_eq!(result.date(), date);
            assert_eq!(result.nanosecond(), nanos);
        }
        assert_eq!(
            parse_datetime_tokens("2024-02-29 12:34:56.1", "yyyy-MM-dd HH:mm:ss.SS"),
            None
        );
        assert_eq!(
            parse_datetime_tokens("2024-02-29 12:34:56.1234567890", "yyyy-MM-dd HH:mm:ss.SSSSSSSSSS"),
            None
        );
    }

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
        assert_eq!(
            parse_datetime_tokens("2024-02-29 02", "yyyy-MM-dd MM"),
            None
        );
        assert_eq!(parse_datetime_tokens("2024-02-29", "yyyy-MM-QQ"), None);
        assert_eq!(parse_datetime_tokens("23:59", "HH:mm"), None);
        assert_eq!(parse_datetime_tokens("2024-02-29", "'unfinished"), None);
    }
    #[test]
    fn parses_english_months_weekdays_and_meridiems() {
        let morning = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(9, 5, 0)
            .unwrap();
        assert_eq!(
            parse_datetime_tokens("Thu, February 29, 2024 9:05 AM", "EEE, MMMM d, yyyy h:mm a"),
            Some(morning)
        );
        assert_eq!(
            parse_datetime_tokens("Thursday Feb 29 2024 09:05 AM", "EEEE MMM dd yyyy hh:mm a"),
            Some(morning)
        );
        assert_eq!(
            parse_datetime_tokens("Feb 29, 2024 9:05 PM", "MMM dd, yyyy h:mm a"),
            Some(morning.with_hour(21).unwrap())
        );
        assert_eq!(
            parse_datetime_tokens("Feb 29, 2024 12:05 AM", "MMM dd, yyyy h:mm a"),
            Some(morning.with_hour(0).unwrap())
        );
        assert_eq!(
            parse_datetime_tokens("Feb 29, 2024 12:05 PM", "MMM dd, yyyy h:mm a"),
            Some(morning.with_hour(12).unwrap())
        );
    }

    #[test]
    fn rejects_conflicting_clock_fields_and_weekdays() {
        for input in [
            "Friday, February 29, 2024 9:05 AM",
            "Thu, February 29, 2024 0:05 AM",
            "Thu, February 29, 2024 13:05 PM",
            "Thu, February 29, 2024 9:05 am",
        ] {
            assert_eq!(
                parse_datetime_tokens(input, "EEE, MMMM d, yyyy h:mm a"),
                None
            );
        }
        assert_eq!(
            parse_datetime_tokens("2024-02-29 9:05", "yyyy-MM-dd h:mm"),
            None
        );
        assert_eq!(
            parse_datetime_tokens("2024-02-29 09:05 PM", "yyyy-MM-dd HH:mm a"),
            None
        );
        assert_eq!(
            parse_datetime_tokens("2024-02-29 09:05", "yyyy-MM-dd hh:mm"),
            None
        );
        assert_eq!(
            parse_datetime_tokens("2024-02-29 February", "yyyy-MM-dd MMMM"),
            None
        );
    }
}
