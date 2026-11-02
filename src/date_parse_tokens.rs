//! Strict parsing of date-only calendar patterns.
//!
//! Supported tokens: yyyy, QQQQ, QQQ, QQ, Qo, Q, MMMM, MMM, MM, M, Mo,
//! dd, d, do, EEEE, EEE, E.
//! English names are case-sensitive and time parts are not parsed.

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

fn parse_month_name(input: &str, at: &mut usize, full: bool) -> Option<u32> {
    let remaining = input.get(*at..)?;
    for (index, name) in MONTHS.iter().enumerate() {
        let token = if full { *name } else { &name[..3] };
        if remaining.starts_with(token) {
            *at += token.len();
            return Some(index as u32 + 1);
        }
    }
    None
}

fn parse_weekday_name(input: &str, at: &mut usize, full: bool) -> Option<u32> {
    let remaining = input.get(*at..)?;
    for (index, name) in WEEKDAYS.iter().enumerate() {
        let token = if full { *name } else { &name[..3] };
        if remaining.starts_with(token) {
            *at += token.len();
            return Some(index as u32);
        }
    }
    None
}

fn digits(input: &str, at: &mut usize, min: usize, max: usize) -> Option<u32> {
    let b = input.as_bytes();
    let begin = *at;
    while *at < b.len() && *at - begin < max && b[*at].is_ascii_digit() {
        *at += 1;
    }
    if *at - begin < min {
        return None;
    }
    input[begin..*at].parse().ok()
}

fn english_ordinal(input: &str, at: &mut usize) -> Option<u32> {
    let number = digits(input, at, 1, 2)?;
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
    if !input.get(*at..)?.starts_with(suffix) {
        return None;
    }
    *at += suffix.len();
    Some(number)
}

/// Parse a complete date using the documented numeric date-fns style tokens.
/// Unrecognized alphabetic tokens, duplicate fields and incomplete input return None.
pub fn parse_date_tokens(input: &str, pattern: &str) -> Option<NaiveDate> {
    let mut pi = 0;
    let mut ii = 0;
    let mut year = None;
    let mut month = None;
    let mut quarter = None;
    let mut day = None;
    let mut weekday = None;
    let pb = pattern.as_bytes();
    while pi < pb.len() {
        if pb[pi] == b'\'' && pb.get(pi + 1) == Some(&b'\'') {
            if !input[ii..].starts_with('\'') {
                return None;
            }
            pi += 2;
            ii += 1;
            continue;
        }
        if pb[pi] == b'\'' {
            pi += 1;
            let mut closed = false;
            while pi < pb.len() {
                if pb[pi] == b'\'' {
                    if pb.get(pi + 1) == Some(&b'\'') {
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
        if rest.starts_with("QQQQ") {
            let parsed = english_ordinal(input, &mut ii)?;
            if !input.get(ii..)?.starts_with(" quarter") {
                return None;
            }
            ii += " quarter".len();
            if quarter.replace(parsed).is_some() {
                return None;
            }
            pi += 4;
        } else if rest.starts_with("QQQ") {
            if !input.get(ii..)?.starts_with('Q') {
                return None;
            }
            ii += 1;
            if quarter.replace(digits(input, &mut ii, 1, 1)?).is_some() {
                return None;
            }
            pi += 3;
        } else if rest.starts_with("QQ") {
            if quarter.replace(digits(input, &mut ii, 2, 2)?).is_some() {
                return None;
            }
            pi += 2;
        } else if rest.starts_with("Qo") {
            if quarter.replace(english_ordinal(input, &mut ii)?).is_some() {
                return None;
            }
            pi += 2;
        } else if rest.starts_with('Q') {
            if quarter.replace(digits(input, &mut ii, 1, 1)?).is_some() {
                return None;
            }
            pi += 1;
        } else if rest.starts_with("MMMM") {
            if month
                .replace(parse_month_name(input, &mut ii, true)?)
                .is_some()
            {
                return None;
            }
            pi += 4;
        } else if rest.starts_with("MMM") {
            if month
                .replace(parse_month_name(input, &mut ii, false)?)
                .is_some()
            {
                return None;
            }
            pi += 3;
        } else if rest.starts_with("EEEE") {
            if weekday
                .replace(parse_weekday_name(input, &mut ii, true)?)
                .is_some()
            {
                return None;
            }
            pi += 4;
        } else if rest.starts_with("EEE") {
            if weekday
                .replace(parse_weekday_name(input, &mut ii, false)?)
                .is_some()
            {
                return None;
            }
            pi += 3;
        } else if rest.starts_with('E') {
            if weekday
                .replace(parse_weekday_name(input, &mut ii, false)?)
                .is_some()
            {
                return None;
            }
            pi += 1;
        } else if rest.starts_with("yyyy") {
            if year.replace(digits(input, &mut ii, 4, 4)? as i32).is_some() {
                return None;
            }
            pi += 4;
        } else if rest.starts_with("Mo") {
            if month.replace(english_ordinal(input, &mut ii)?).is_some() {
                return None;
            }
            pi += 2;
        } else if rest.starts_with("MM") {
            if month.replace(digits(input, &mut ii, 2, 2)?).is_some() {
                return None;
            }
            pi += 2;
        } else if rest.starts_with('M') {
            if month.replace(digits(input, &mut ii, 1, 2)?).is_some() {
                return None;
            }
            pi += 1;
        } else if rest.starts_with("do") {
            if day.replace(english_ordinal(input, &mut ii)?).is_some() {
                return None;
            }
            pi += 2;
        } else if rest.starts_with("dd") {
            if day.replace(digits(input, &mut ii, 2, 2)?).is_some() {
                return None;
            }
            pi += 2;
        } else if rest.starts_with('d') {
            if day.replace(digits(input, &mut ii, 1, 2)?).is_some() {
                return None;
            }
            pi += 1;
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
    if quarter.is_some_and(|q| !(1..=4).contains(&q)) {
        return None;
    }
    // A quarter alone selects its first day; a month still requires a day.
    let month = match (month, quarter, day) {
        (Some(m), _, Some(_)) => m,
        (None, Some(q), None) => (q - 1) * 3 + 1,
        _ => return None,
    };
    let day = day.unwrap_or(1);
    if quarter.is_some_and(|q| (month - 1) / 3 + 1 != q) {
        return None;
    }
    let parsed = NaiveDate::from_ymd_opt(year?, month, day)?;
    if weekday.is_some_and(|day| parsed.weekday().num_days_from_sunday() != day) {
        return None;
    }
    Some(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quarter_tokens_and_checks_month_consistency() {
        let april = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        for (input, pattern) in [
            ("2024 Q2", "yyyy QQQ"),
            ("2024-02", "yyyy-QQ"),
            ("2024 2nd", "yyyy Qo"),
            ("2024 2nd quarter", "yyyy QQQQ"),
            ("2024 2", "yyyy Q"),
        ] {
            assert_eq!(parse_date_tokens(input, pattern), Some(april));
        }
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            parse_date_tokens("2024-02-29 Q1", "yyyy-MM-dd QQQ"),
            Some(leap)
        );
        assert_eq!(parse_date_tokens("2024-02-29 Q2", "yyyy-MM-dd QQQ"), None);
    }

    #[test]
    fn rejects_invalid_quarters_and_partial_dates() {
        for text in ["2024 Q0", "2024 Q5", "2024 Q22"] {
            assert_eq!(parse_date_tokens(text, "yyyy QQQ"), None);
        }
        assert_eq!(parse_date_tokens("2024 2th quarter", "yyyy QQQQ"), None);
        assert_eq!(parse_date_tokens("2024-02-29", "yyyy-QQ-dd"), None);
        assert_eq!(parse_date_tokens("2024 Q1 Q2", "yyyy QQQ QQQ"), None);
    }

    #[test]
    fn parses_leap_days_and_multiple_field_orders() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(parse_date_tokens("2024-02-29", "yyyy-MM-dd"), Some(date));
        assert_eq!(parse_date_tokens("2/29/2024", "M/d/yyyy"), Some(date));
        assert_eq!(parse_date_tokens("29 Feb? 2024", "dd 'Feb?' yyyy"), None);
        assert_eq!(
            parse_date_tokens("29 day 02 2024", "dd 'day' MM yyyy"),
            Some(date)
        );
    }

    #[test]
    fn rejects_invalid_dates_unfinished_inputs_and_unsupported_tokens() {
        for input in ["2023-02-29", "2024-13-01", "2024-02-30", "2024-02-29 "] {
            assert_eq!(parse_date_tokens(input, "yyyy-MM-dd"), None);
        }
        assert_eq!(parse_date_tokens("2024-02", "yyyy-MM-dd"), None);
        assert_eq!(parse_date_tokens("2024-02-29", "yyyy-QQ-dd"), None);
        assert_eq!(parse_date_tokens("2024-02-29", "yyyy-MM-MM"), None);
    }

    #[test]
    fn parses_english_months_and_verifies_weekday_names() {
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            parse_date_tokens("Thursday, February 29, 2024", "EEEE, MMMM d, yyyy"),
            Some(leap)
        );
        assert_eq!(
            parse_date_tokens("Thu, Feb 29 2024", "EEE, MMM dd yyyy"),
            Some(leap)
        );
        assert_eq!(
            parse_date_tokens("Thu Feb 29 2024", "E MMM dd yyyy"),
            Some(leap)
        );
        assert_eq!(parse_date_tokens("Feb 29 2024", "MMM dd yyyy"), Some(leap));
    }

    #[test]
    fn rejects_mismatched_weekdays_duplicate_fields_and_nonenglish_names() {
        assert_eq!(
            parse_date_tokens("Friday, February 29, 2024", "EEEE, MMMM d, yyyy"),
            None
        );
        assert_eq!(parse_date_tokens("FEB 29 2024", "MMM dd yyyy"), None);
        assert_eq!(parse_date_tokens("Feb 29 2023", "MMM dd yyyy"), None);
        assert_eq!(parse_date_tokens("Feb 29 02 2024", "MMM dd MM yyyy"), None);
        assert_eq!(
            parse_date_tokens("Thu Thursday Feb 29 2024", "EEE EEEE MMM dd yyyy"),
            None
        );
    }

    #[test]
    fn parses_ordinal_day_and_month_tokens_emitted_by_formatter() {
        let leap = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            parse_date_tokens("February 29th, 2024", "MMMM do, yyyy"),
            Some(leap)
        );
        assert_eq!(parse_date_tokens("2nd/29th/2024", "Mo/do/yyyy"), Some(leap));
        for (day, ordinal) in [
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
            let date = NaiveDate::from_ymd_opt(2024, 1, day).unwrap();
            assert_eq!(
                parse_date_tokens(&format!("Jan {ordinal}, 2024"), "MMM do, yyyy"),
                Some(date)
            );
            let formatted = crate::format_date_tokens(date, "Mo/do/yyyy").unwrap();
            assert_eq!(parse_date_tokens(&formatted, "Mo/do/yyyy"), Some(date));
        }
    }

    #[test]
    fn ordinal_parser_rejects_wrong_suffixes_and_duplicate_fields() {
        for text in [
            "Feb 11st 2024",
            "Feb 2th 2024",
            "Feb 30th 2024",
            "Feb 29TH 2024",
        ] {
            assert_eq!(parse_date_tokens(text, "MMM do yyyy"), None, "{text}");
        }
        assert_eq!(parse_date_tokens("2rd/29th/2024", "Mo/do/yyyy"), None);
        assert_eq!(
            parse_date_tokens("Feb 29th 29 2024", "MMM do dd yyyy"),
            None
        );
    }

    #[test]
    fn parses_doubled_apostrophes_outside_quoted_literals() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(parse_date_tokens("2024'02'29", "yyyy''MM''dd"), Some(date));
        assert_eq!(
            parse_date_tokens("'' 2024-02-29", "'''' yyyy-MM-dd"),
            Some(date)
        );
        assert_eq!(parse_date_tokens("2024-02-29", "yyyy''MM''dd"), None);
    }

    #[test]
    fn doubled_quote_matches_a_literal_apostrophe() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(parse_date_tokens("2024'02'29", "yyyy''MM''dd"), Some(date));
        assert_eq!(parse_date_tokens("2024-02'29", "yyyy''MM''dd"), None);
        assert_eq!(
            parse_date_tokens("2024 it's 02 29", "yyyy 'it''s' MM dd"),
            Some(date)
        );
    }
}
