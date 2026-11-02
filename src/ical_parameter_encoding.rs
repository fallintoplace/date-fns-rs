//! RFC 6868 caret escaping for iCalendar parameter values.

/// Encode quotes, carets, and line breaks in an iCalendar parameter value.
///
/// This handles value escaping, not surrounding syntactic quoting.
pub fn encode_ical_parameter_value(input: &str) -> Option<String> {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '^' => output.push_str("^^"),
            '"' => output.push_str("^'"),
            '\n' => output.push_str("^n"),
            '\r' => {
                if chars.next() != Some('\n') {
                    return None;
                }
                output.push_str("^n");
            }
            _ if ch.is_control() && ch != '\t' => return None,
            _ => output.push(ch),
        }
    }
    Some(output)
}

/// Decode RFC 6868 caret escapes from an iCalendar parameter value.
///
/// Unknown sequences are preserved verbatim, as required by RFC 6868.
pub fn decode_ical_parameter_value(input: &str) -> Option<String> {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '^' {
            match chars.next() {
                Some('n') => output.push('\n'),
                Some('^') => output.push('^'),
                Some('\'') => output.push('"'),
                Some(other) => {
                    output.push('^');
                    output.push(other);
                }
                None => output.push('^'),
            }
        } else if ch.is_control() && ch != '\t' {
            return None;
        } else {
            output.push(ch);
        }
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_quotes_carets_and_unicode() {
        let original = "George ^ \"Babe\" Ruth\nZürich";
        let encoded = "George ^^ ^'Babe^' Ruth^nZürich";
        assert_eq!(encode_ical_parameter_value(original).as_deref(), Some(encoded));
        assert_eq!(decode_ical_parameter_value(encoded).as_deref(), Some(original));
        assert_eq!(encode_ical_parameter_value("a\r\nb").as_deref(), Some("a^nb"));
    }

    #[test]
    fn keeps_unknown_sequences_and_rejects_raw_controls() {
        assert_eq!(decode_ical_parameter_value("x^q ^"), Some("x^q ^".to_owned()));
        assert_eq!(encode_ical_parameter_value("bad\r"), None);
        assert_eq!(decode_ical_parameter_value("bad\n"), None);
        assert_eq!(encode_ical_parameter_value("bad\0"), None);
    }
}
