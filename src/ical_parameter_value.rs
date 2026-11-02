//! RFC 6868 caret encoding for iCalendar parameter values.

/// Decode one parameter value, including optional enclosing double quotes.
///
/// Caret-n and caret-N encode a newline, double carets encode a caret,
/// and caret-apostrophe encodes a double quote. Unknown sequences are literal.
/// Raw controls and malformed quotes are rejected.
pub fn decode_ical_parameter_value(value: &str) -> Option<String> {
    if value.chars().any(char::is_control) {
        return None;
    }
    let inner = if let Some(unquoted) = value.strip_prefix('"') {
        unquoted.strip_suffix('"')?
    } else {
        if value.is_empty() || value.contains('"') || value.contains(';') || value.contains(':') {
            return None;
        }
        value
    };
    if inner.contains('"') {
        return None;
    }

    let mut decoded = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '^' {
            decoded.push(ch);
            continue;
        }
        match chars.peek() {
            Some('n' | 'N') => {
                chars.next();
                decoded.push('\n');
            }
            Some('^') => {
                chars.next();
                decoded.push('^');
            }
            Some('\'') => {
                chars.next();
                decoded.push('"');
            }
            _ => decoded.push('^'),
        }
    }
    Some(decoded)
}

/// Encode a parameter value for insertion in an iCalendar content line.
///
/// Literal line feeds, quotes, and carets use RFC 6868 caret sequences.
/// Reserved separators are wrapped in quotes; other controls are rejected.
pub fn encode_ical_parameter_value(value: &str) -> Option<String> {
    let mut encoded = String::with_capacity(value.len());
    let mut quote = value.is_empty();
    for ch in value.chars() {
        match ch {
            '\n' => encoded.push_str("^n"),
            '^' => encoded.push_str("^^"),
            '"' => encoded.push_str("^'"),
            ch if ch.is_control() => return None,
            ',' | ';' | ':' => {
                quote = true;
                encoded.push(ch);
            }
            _ => encoded.push(ch),
        }
    }
    if quote {
        Some(format!("\"{encoded}\""))
    } else {
        Some(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_carets_and_quoted_parameter_values() {
        assert_eq!(
            decode_ical_parameter_value(r#""First^nSecond; ^'Hi^' ^^""#).as_deref(),
            Some("First\nSecond; \"Hi\" ^")
        );
        assert_eq!(decode_ical_parameter_value("a^x^"), Some("a^x^".to_owned()));
        assert_eq!(decode_ical_parameter_value("a^Nb"), Some("a\nb".to_owned()));
    }

    #[test]
    fn encodes_values_for_existing_content_line_parser() {
        for original in [
            "hello",
            "semi;colon",
            "a,b:c",
            "quoted \"value\"",
            "line\nbreak",
            "caret ^",
            "",
            "Café",
        ] {
            let encoded = encode_ical_parameter_value(original).unwrap();
            let line = format!("SUMMARY;CN={encoded}:Meeting");
            let parsed = crate::parse_ical_content_line(&line).unwrap();
            assert_eq!(
                decode_ical_parameter_value(parsed.parameters[0].1).as_deref(),
                Some(original)
            );
        }
    }

    #[test]
    fn rejects_invalid_raw_controls_and_quote_structure() {
        for input in [
            "abc\rdef",
            "abc\n",
            "a\tb",
            "\"unterminated",
            "literal\"quote",
            "text;more",
            "text:more",
        ] {
            assert_eq!(decode_ical_parameter_value(input), None, "{input:?}");
        }
        assert_eq!(encode_ical_parameter_value("a\rb"), None);
        assert_eq!(encode_ical_parameter_value("a\tb"), None);
    }
}
