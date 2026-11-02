//! A zero-copy parser for a single unfolded RFC 5545 content line.

/// An unfolded iCalendar property with borrowed source slices.
///
/// Parameter values are returned verbatim, including surrounding quotes.
/// TEXT property values still need RFC 5545 unescaping separately.
#[derive(Debug, PartialEq, Eq)]
pub struct IcalContentLine<'a> {
    pub name: &'a str,
    pub parameters: Vec<(&'a str, &'a str)>,
    pub value: &'a str,
}

fn valid_name(name: &str) -> bool {
    name.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
        && name.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}

fn split_unquoted(text: &str) -> Option<Vec<&str>> {
    let mut result = Vec::new();
    let mut quoted = false;
    let mut start = 0;
    for (offset, ch) in text.char_indices() {
        if ch == '"' {
            quoted = !quoted;
        } else if ch == ';' && !quoted {
            result.push(&text[start..offset]);
            start = offset + 1;
        }
    }
    if quoted {
        return None;
    }
    result.push(&text[start..]);
    Some(result)
}

fn valid_param_value(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    if let Some(inner) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return !inner.contains('"') && !inner.chars().any(char::is_control);
    }
    !value.contains('"') && !value.contains(';') && !value.contains(':')
}

/// Parse one unfolded iCalendar content line, retaining borrowed values.
///
/// Quotes protect colons and semicolons in parameter values. Rejects invalid
/// tokens, unterminated quotes, missing values, and physical line breaks.
/// This is a structural parser; it does not validate property-specific values
/// or decode RFC 6868 parameter escaping.
pub fn parse_ical_content_line(input: &str) -> Option<IcalContentLine<'_>> {
    if input.chars().any(|ch| ch.is_control() && ch != '\t') {
        return None;
    }
    let mut quoted = false;
    let mut colon = None;
    for (offset, ch) in input.char_indices() {
        if ch == '"' {
            quoted = !quoted;
        } else if ch == ':' && !quoted {
            colon = Some(offset);
            break;
        }
    }
    let colon = colon?;
    let header = &input[..colon];
    let value = &input[colon + 1..];
    let mut fields = split_unquoted(header)?.into_iter();
    let name = fields.next()?;
    if !valid_name(name) {
        return None;
    }
    let mut parameters = Vec::new();
    for entry in fields {
        let (param_name, param_value) = entry.split_once('=')?;
        if !valid_name(param_name) || !valid_param_value(param_value) {
            return None;
        }
        parameters.push((param_name, param_value));
    }
    Some(IcalContentLine { name, parameters, value })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_colons_and_semicolons_without_losing_source_text() {
        let line = r#"ATTENDEE;CN="Doe; Jane: Team";ROLE=REQ-PARTICIPANT:mailto:jane@example.org"#;
        let parsed = parse_ical_content_line(line).unwrap();
        assert_eq!(parsed.name, "ATTENDEE");
        assert_eq!(
            parsed.parameters,
            vec![("CN", r#""Doe; Jane: Team""#), ("ROLE", "REQ-PARTICIPANT")]
        );
        assert_eq!(parsed.value, "mailto:jane@example.org");
    }

    #[test]
    fn supports_simple_properties_and_unicode_values() {
        let line = "SUMMARY:Planning in Zürich; bring notes";
        assert_eq!(
            parse_ical_content_line(line),
            Some(IcalContentLine {
                name: "SUMMARY",
                parameters: vec![],
                value: "Planning in Zürich; bring notes",
            })
        );
        assert_eq!(
            parse_ical_content_line("DTSTART;TZID=Europe/Berlin:20241103T090000")
                .unwrap().parameters,
            vec![("TZID", "Europe/Berlin")]
        );
        assert_eq!(parse_ical_content_line("X-CUSTOM:").unwrap().value, "");
    }

    #[test]
    fn rejects_invalid_headers_and_unfolded_input() {
        for invalid in [
            "SUMMARY",
            ":value",
            "SUMMARY;:value",
            "SUMMARY;CN=value",
            "SUMMARY;CN=:value",
            "SUMMARY;CN=\"unclosed:value",
            "SUMMARY;CN=value;bad:hello",
            "SUMMARY;CN=\"a\"b:hello",
            "SUM MARY:hello",
            "SUMMARY:line\nbreak",
            "SUMMARY:line\r\n more",
            "SUMMARY:\0bad",
        ] {
            assert!(parse_ical_content_line(invalid).is_none(), "{invalid:?}");
        }
    }
}
