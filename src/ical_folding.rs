//! RFC 5545 content-line folding on UTF-8 byte boundaries.

/// Fold a logical calendar content line at 75 octets per physical line.
///
/// UTF-8 code points are never split. Each continuation begins with CRLF and
/// a single space, which counts toward its own 75-byte limit.
pub fn fold_ical_content_line(input: &str) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    let mut width = 0;
    for ch in input.chars() {
        if ch == '\n' || ch == '\r' || (ch.is_control() && ch != '\t') {
            return None;
        }
        let bytes = ch.len_utf8();
        if width + bytes > 75 {
            result.push_str("\r\n ");
            width = 1;
        }
        result.push(ch);
        width += bytes;
    }
    Some(result)
}

/// Unfold physical iCalendar content lines using CRLF whitespace continuation.
///
/// CRLF followed by one space or tab is removed. CRLF between independent
/// properties is preserved; bare LF and CR are invalid.
pub fn unfold_ical_content_lines(input: &str) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    for (index, line) in input.split("\r\n").enumerate() {
        if line.contains('\r') || line.contains('\n') {
            return None;
        }
        if index == 0 {
            result.push_str(line);
        } else if let Some(continued) = line.strip_prefix(' ') {
            result.push_str(continued);
        } else if let Some(continued) = line.strip_prefix('\t') {
            result.push_str(continued);
        } else {
            result.push_str("\r\n");
            result.push_str(line);
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_long_utf8_properties_without_splitting_characters() {
        let original = format!("SUMMARY:{}", "🌍 München".repeat(15));
        let folded = fold_ical_content_line(&original).unwrap();
        assert!(folded.contains("\r\n "));
        assert!(folded.split("\r\n").all(|line| line.len() <= 75));
        assert_eq!(unfold_ical_content_lines(&folded), Some(original));
    }

    #[test]
    fn preserves_property_boundaries_and_unfolds_tabs() {
        let physical = "SUMMARY:Hello\r\n World\r\nDESCRIPTION:More\r\n\ttext";
        assert_eq!(
            unfold_ical_content_lines(physical).as_deref(),
            Some("SUMMARY:HelloWorld\r\nDESCRIPTION:Moretext")
        );
        assert_eq!(fold_ical_content_line("SUMMARY:Hello"), Some("SUMMARY:Hello".to_owned()));
    }

    #[test]
    fn rejects_bare_newlines_and_control_characters() {
        assert_eq!(fold_ical_content_line("bad\nline"), None);
        assert_eq!(fold_ical_content_line("bad\rline"), None);
        assert_eq!(unfold_ical_content_lines("bad\nline"), None);
        assert_eq!(unfold_ical_content_lines("bad\rline"), None);
    }
}
