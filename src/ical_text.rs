//! RFC 5545 TEXT value escaping for calendar properties.

/// Escape a plain text property value for iCalendar serialization.
///
/// Line endings are emitted as \\n. Bare CR and control characters are rejected.
pub fn escape_ical_text(input: &str) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => result.push_str("\\\\"),
            ';' => result.push_str("\\;"),
            ',' => result.push_str("\\,"),
            '\n' => result.push_str("\\n"),
            '\r' => {
                if chars.next() != Some('\n') {
                    return None;
                }
                result.push_str("\\n");
            }
            _ if ch.is_control() && ch != '\t' => return None,
            _ => result.push(ch),
        }
    }
    Some(result)
}

/// Decode iCalendar TEXT escapes into a Unicode string.
///
/// Only the escape forms in RFC 5545 are accepted; raw line breaks must be
/// unfolded by the caller before this function is used.
pub fn unescape_ical_text(input: &str) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => match chars.next()? {
                'n' | 'N' => result.push('\n'),
                '\\' => result.push('\\'),
                ';' => result.push(';'),
                ',' => result.push(','),
                _ => return None,
            },
            '\n' | '\r' => return None,
            _ if ch.is_control() && ch != '\t' => return None,
            _ => result.push(ch),
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_unicode_punctuation_and_normalized_newlines() {
        let input = "Planning; Berlin, Zürich\\meeting\r\nnext line";
        let escaped = "Planning\\; Berlin\\, Zürich\\\\meeting\\nnext line";
        assert_eq!(escape_ical_text(input).as_deref(), Some(escaped));
        assert_eq!(
            unescape_ical_text(escaped).as_deref(),
            Some("Planning; Berlin, Zürich\\meeting\nnext line")
        );
        assert_eq!(unescape_ical_text("a\\Nb"), Some("a\nb".to_owned()));
    }

    #[test]
    fn rejects_unrecognized_escapes_and_bare_controls() {
        for invalid in ["bad\\", "bad\\x", "line\nfeed", "carriage\rreturn", "\0"] {
            assert_eq!(unescape_ical_text(invalid), None);
        }
        assert_eq!(escape_ical_text("bad\rreturn"), None);
        assert_eq!(escape_ical_text("\0"), None);
    }
}
