//! Strict ISO 8601 fixed-length durations with nanosecond precision.
//!
//! Calendar months and years are deliberately unsupported because their
//! elapsed duration depends on a reference date. No time zone is inferred.

const NANOS_PER_SECOND: u128 = 1_000_000_000;
const NANOS_PER_MINUTE: u128 = 60 * NANOS_PER_SECOND;
const NANOS_PER_HOUR: u128 = 60 * NANOS_PER_MINUTE;
const NANOS_PER_DAY: u128 = 24 * NANOS_PER_HOUR;
const NANOS_PER_WEEK: u128 = 7 * NANOS_PER_DAY;

/// Parse a fixed-length ISO duration into signed nanoseconds.
///
/// Supports -P3DT4H5M6.123456789S, PT0S, and week-only forms such as P2W.
/// Date-only forms may contain days. Within each section, units must appear
/// once and in decreasing order of magnitude. Fractions are allowed only
/// on seconds with at most nine digits. Months and years are rejected.
pub fn parse_iso_fixed_duration(input: &str) -> Option<i128> {
    let bytes = input.as_bytes();
    let (negative, mut index) = if bytes.first() == Some(&b'-') {
        (true, 1)
    } else {
        (false, 0)
    };
    if bytes.get(index) != Some(&b'P') {
        return None;
    }
    index += 1;

    let mut total = 0_u128;
    let mut in_time = false;
    let mut seen_field = false;
    let mut seen_time_field = false;
    let mut saw_week = false;
    let mut last_date_rank = 0;
    let mut last_time_rank = 0;

    while index < bytes.len() {
        if bytes[index] == b'T' {
            if in_time || saw_week {
                return None;
            }
            in_time = true;
            index += 1;
            continue;
        }
        let integer_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index == integer_start {
            return None;
        }
        let whole = input[integer_start..index].parse::<u128>().ok()?;

        let mut fractional_nanos = None;
        if bytes.get(index) == Some(&b'.') {
            index += 1;
            let fraction_start = index;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
            let count = index - fraction_start;
            if count == 0 || count > 9 {
                return None;
            }
            let mut fraction = input[fraction_start..index].parse::<u128>().ok()?;
            for _ in count..9 {
                fraction *= 10;
            }
            fractional_nanos = Some(fraction);
        }

        let unit = *bytes.get(index)?;
        index += 1;
        let factor = match (in_time, unit) {
            (false, b'W') if !seen_field => {
                saw_week = true;
                last_date_rank = 2;
                NANOS_PER_WEEK
            }
            (false, b'D') if !saw_week && last_date_rank == 0 => {
                last_date_rank = 1;
                NANOS_PER_DAY
            }
            (true, b'H') if last_time_rank == 0 => {
                last_time_rank = 1;
                NANOS_PER_HOUR
            }
            (true, b'M') if last_time_rank < 2 => {
                last_time_rank = 2;
                NANOS_PER_MINUTE
            }
            (true, b'S') if last_time_rank < 3 => {
                last_time_rank = 3;
                NANOS_PER_SECOND
            }
            _ => return None,
        };
        if fractional_nanos.is_some() && unit != b'S' {
            return None;
        }
        let component = whole
            .checked_mul(factor)?
            .checked_add(fractional_nanos.unwrap_or(0))?;
        total = total.checked_add(component)?;
        seen_field = true;
        if in_time {
            seen_time_field = true;
        }
    }
    if !seen_field || (in_time && !seen_time_field) {
        return None;
    }
    let limit = (i128::MAX as u128) + u128::from(negative);
    if total > limit {
        return None;
    }
    if negative && total == limit {
        Some(i128::MIN)
    } else {
        let value = i128::try_from(total).ok()?;
        Some(if negative { -value } else { value })
    }
}

/// Format a signed fixed-length duration as a canonical ISO string.
///
/// Uses day and clock fields rather than months/years or weeks. Fractional
/// seconds have up to nine digits with trailing zeroes removed. Zero is
/// formatted as PT0S, and i128::MIN round-trips safely.
pub fn format_iso_fixed_duration(nanoseconds: i128) -> String {
    let mut remainder = nanoseconds.unsigned_abs();
    let days = remainder / NANOS_PER_DAY;
    remainder %= NANOS_PER_DAY;
    let hours = remainder / NANOS_PER_HOUR;
    remainder %= NANOS_PER_HOUR;
    let minutes = remainder / NANOS_PER_MINUTE;
    remainder %= NANOS_PER_MINUTE;
    let seconds = remainder / NANOS_PER_SECOND;
    let nanos = remainder % NANOS_PER_SECOND;

    let mut result = String::new();
    if nanoseconds < 0 {
        result.push('-');
    }
    result.push('P');
    if days > 0 {
        result.push_str(&format!("{days}D"));
    }
    if hours > 0 || minutes > 0 || seconds > 0 || nanos > 0 || nanoseconds == 0 {
        result.push('T');
        if hours > 0 {
            result.push_str(&format!("{hours}H"));
        }
        if minutes > 0 {
            result.push_str(&format!("{minutes}M"));
        }
        if nanos > 0 {
            let mut fraction = format!("{nanos:09}");
            while fraction.ends_with('0') {
                fraction.pop();
            }
            result.push_str(&format!("{seconds}.{fraction}S"));
        } else if seconds > 0 || nanoseconds == 0 {
            result.push_str(&format!("{seconds}S"));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_composite_week_and_negative_durations() {
        assert_eq!(parse_iso_fixed_duration("PT0S"), Some(0));
        assert_eq!(
            parse_iso_fixed_duration("P2W"),
            Some((2 * NANOS_PER_WEEK) as i128)
        );
        assert_eq!(
            parse_iso_fixed_duration("-P2DT3H4M5.000000006S"),
            Some(
                -((2 * NANOS_PER_DAY
                    + 3 * NANOS_PER_HOUR
                    + 4 * NANOS_PER_MINUTE
                    + 5 * NANOS_PER_SECOND
                    + 6) as i128)
            )
        );
        assert_eq!(parse_iso_fixed_duration("PT.5S"), None);
        assert_eq!(parse_iso_fixed_duration("PT0.5S"), Some(500_000_000));
    }

    #[test]
    fn rejects_invalid_ambiguous_and_noncanonical_fields() {
        for invalid in [
            "",
            "P",
            "PT",
            "P1DT",
            "P1M",
            "P1Y",
            "P1W2D",
            "P1WT2H",
            "P1D1D",
            "PT2H1H",
            "PT1M2H",
            "PT1S2M",
            "PT1.5H",
            "PT1.1234567890S",
            "PT1.S",
            "PT1,1S",
            "PT1Sgarbage",
            "P-1D",
            "+PT1S",
            "P1DT2D",
            "P1W1W",
            "P1DTT1H",
        ] {
            assert_eq!(parse_iso_fixed_duration(invalid), None, "{invalid}");
        }
    }

    #[test]
    fn round_trips_nanosecond_precision_and_long_durations() {
        for value in [
            0,
            1,
            10,
            123_456_789,
            1_000_000_000,
            86_400_000_000_000,
            -1,
            -123_456_789,
            -86_400_000_000_000,
            i64::MAX as i128,
            i64::MIN as i128,
            i128::MAX,
            i128::MIN,
        ] {
            let formatted = format_iso_fixed_duration(value);
            assert_eq!(
                parse_iso_fixed_duration(&formatted),
                Some(value),
                "{value} => {formatted}"
            );
        }
    }

    #[test]
    fn formats_canonical_zero_days_and_fractional_seconds() {
        assert_eq!(format_iso_fixed_duration(0), "PT0S");
        assert_eq!(format_iso_fixed_duration(1), "PT0.000000001S");
        assert_eq!(format_iso_fixed_duration(1_200_000_000), "PT1.2S");
        assert_eq!(
            format_iso_fixed_duration((NANOS_PER_WEEK * 2) as i128),
            "P14D"
        );
        assert_eq!(
            format_iso_fixed_duration(-((NANOS_PER_DAY + NANOS_PER_HOUR) as i128)),
            "-P1DT1H"
        );
    }

    #[test]
    fn parser_rejects_out_of_range_magnitudes_and_fractions() {
        assert_eq!(
            parse_iso_fixed_duration("P9999999999999999999999999999999999999999999D"),
            None
        );
        assert_eq!(
            parse_iso_fixed_duration("PT9999999999999999999999999999999999999999S"),
            None
        );
        assert_eq!(parse_iso_fixed_duration("PT0.0000000001S"), None);
    }
}
