//! Strict RFC 5545 UTC-OFFSET values for iCalendar VTIMEZONE properties.

/// Parse an iCalendar UTC-OFFSET in +HHMM or +HHMMSS form.
///
/// Returns the signed number of seconds east of UTC. Zero must use +0000.
pub fn parse_ical_utc_offset(input: &str) -> Option<i32> {
    let bytes = input.as_bytes();
    if bytes.len() != 5 && bytes.len() != 7 {
        return None;
    }
    let sign = match bytes[0] {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    if !bytes[1..].iter().all(u8::is_ascii_digit) {
        return None;
    }
    let hours = input[1..3].parse::<i32>().ok()?;
    let minutes = input[3..5].parse::<i32>().ok()?;
    let seconds = if bytes.len() == 7 {
        input[5..7].parse::<i32>().ok()?
    } else {
        0
    };
    if hours > 23 || minutes > 59 || seconds > 59 {
        return None;
    }
    let amount = hours * 3600 + minutes * 60 + seconds;
    if sign < 0 && amount == 0 {
        return None;
    }
    Some(sign * amount)
}

/// Format a signed UTC offset using minute or second precision.
///
/// Rejects offsets outside the 23:59:59 range.
pub fn format_ical_utc_offset(seconds: i32) -> Option<String> {
    if !(-86_399..=86_399).contains(&seconds) {
        return None;
    }
    let sign = if seconds < 0 { '-' } else { '+' };
    let amount = seconds.unsigned_abs();
    let hours = amount / 3600;
    let minutes = amount % 3600 / 60;
    let remainder = amount % 60;
    if remainder == 0 {
        Some(format!("{sign}{hours:02}{minutes:02}"))
    } else {
        Some(format!("{sign}{hours:02}{minutes:02}{remainder:02}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_positive_negative_and_second_precision() {
        for (text, value) in [
            ("+0000", 0),
            ("+0530", 19_800),
            ("-0930", -34_200),
            ("+124530", 45_930),
            ("-235959", -86_399),
        ] {
            assert_eq!(parse_ical_utc_offset(text), Some(value));
            assert_eq!(format_ical_utc_offset(value).as_deref(), Some(text));
        }
    }

    #[test]
    fn rejects_invalid_magnitudes_and_noncanonical_zero() {
        for invalid in [
            "-0000", "-000000", "0000", "+2400", "+1260", "+236060",
            "+013", "+000", "+00:00", "+0000 ", "+0000000", "+010Z",
        ] {
            assert_eq!(parse_ical_utc_offset(invalid), None, "{invalid}");
        }
        assert_eq!(format_ical_utc_offset(86_400), None);
        assert_eq!(format_ical_utc_offset(-86_400), None);
    }
}
