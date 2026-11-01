//! Strict parsing of date-only calendar patterns.
//!
//! Supported tokens: yyyy, MM, M, dd and d. English names and time parts are not parsed.

use chrono::NaiveDate;

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

/// Parse a complete date using the documented numeric date-fns style tokens.
/// Unrecognized alphabetic tokens, duplicate fields and incomplete input return None.
pub fn parse_date_tokens(input: &str, pattern: &str) -> Option<NaiveDate> {
    let mut pi = 0;
    let mut ii = 0;
    let mut year = None;
    let mut month = None;
    let mut day = None;
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
        if rest.starts_with("yyyy") {
            if year.replace(digits(input, &mut ii, 4, 4)? as i32).is_some() {
                return None;
            }
            pi += 4;
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
    NaiveDate::from_ymd_opt(year?, month?, day?)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parses_doubled_apostrophes_outside_quoted_literals() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            parse_date_tokens("2024'02'29", "yyyy''MM''dd"),
            Some(date)
        );
        assert_eq!(
            parse_date_tokens("'' 2024-02-29", "'''' yyyy-MM-dd"),
            Some(date)
        );
        assert_eq!(parse_date_tokens("2024-02-29", "yyyy''MM''dd"), None);
    }

}
