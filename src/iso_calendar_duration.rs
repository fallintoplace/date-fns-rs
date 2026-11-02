//! Calendar-relative ISO 8601 durations for date-only arithmetic.

use chrono::{Days, NaiveDate};

/// A signed ISO 8601 duration with whole calendar year, month, week and day units.
///
/// Weeks cannot be mixed with other units in the strict parser. Applying
/// years and months first clamps the day to the target month; remaining weeks
/// and days are then added as calendar days.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsoCalendarDuration {
    pub negative: bool,
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u32,
}

/// Parse a strict whole-unit ISO period such as `P1Y2M3D` or `-P2W`.
///
/// Rejects fractions, time components, misplaced units, duplicates and mixed
/// week representations. The zero duration is written `P0D`.
pub fn parse_iso_calendar_duration(text: &str) -> Option<IsoCalendarDuration> {
    let (negative, text) = match text.strip_prefix('-') {
        Some(remaining) => (true, remaining),
        None => (false, text),
    };
    let bytes = text.as_bytes();
    if bytes.first() != Some(&b'P') || bytes.len() < 3 {
        return None;
    }

    let mut value = IsoCalendarDuration {
        negative,
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
    };
    let mut cursor = 1;
    let mut last_order = 0;
    let mut saw_week = false;
    let mut saw_other = false;

    while cursor < bytes.len() {
        let first_digit = cursor;
        let mut number = 0_u32;
        while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
            number = number
                .checked_mul(10)?
                .checked_add(u32::from(bytes[cursor] - b'0'))?;
            cursor += 1;
        }
        if cursor == first_digit || cursor == bytes.len() {
            return None;
        }
        let order = match bytes[cursor] {
            b'Y' => {
                value.years = number;
                saw_other = true;
                1
            }
            b'M' => {
                value.months = number;
                saw_other = true;
                2
            }
            b'W' => {
                value.weeks = number;
                saw_week = true;
                3
            }
            b'D' => {
                value.days = number;
                saw_other = true;
                4
            }
            _ => return None,
        };
        if order <= last_order || (saw_week && saw_other) {
            return None;
        }
        last_order = order;
        cursor += 1;
    }
    Some(value)
}

/// Format an ISO calendar duration with zero represented as `P0D`.
///
/// The sign is retained for nonzero values and suppressed for zero.
pub fn format_iso_calendar_duration(value: IsoCalendarDuration) -> String {
    let mut text = String::new();
    let nonzero = value.years != 0 || value.months != 0 || value.weeks != 0 || value.days != 0;
    if value.negative && nonzero {
        text.push('-');
    }
    text.push('P');

    if value.weeks != 0 && value.years == 0 && value.months == 0 && value.days == 0 {
        text.push_str(&format!("{}W", value.weeks));
    } else {
        if value.years != 0 {
            text.push_str(&format!("{}Y", value.years));
        }
        if value.months != 0 {
            text.push_str(&format!("{}M", value.months));
        }
        if value.days != 0 {
            text.push_str(&format!("{}D", value.days));
        }
        if !nonzero {
            text.push_str("0D");
        }
    }
    text
}

/// Apply a calendar-relative period using checked, month-clamping arithmetic.
///
/// Year and month components are combined before shifting; for example,
/// January 31 plus `P1M` becomes February 28 or 29, not a fixed 30-day jump.
pub fn apply_iso_calendar_duration(
    start: NaiveDate,
    period: IsoCalendarDuration,
) -> Option<NaiveDate> {
    if period.weeks != 0 && (period.years != 0 || period.months != 0 || period.days != 0) {
        return None;
    }

    let months = period
        .years
        .checked_mul(12)?
        .checked_add(period.months)?;
    let signed_months = i32::try_from(months).ok()?;
    let signed_months = if period.negative {
        -signed_months
    } else {
        signed_months
    };
    let shifted = crate::add_months(start, signed_months)?;
    let days = u64::from(period.weeks)
        .checked_mul(7)?
        .checked_add(u64::from(period.days))?;
    if period.negative {
        shifted.checked_sub_days(Days::new(days))
    } else {
        shifted.checked_add_days(Days::new(days))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_leap_years_month_clamping_and_negative_periods() {
        let plus = parse_iso_calendar_duration("P1M").unwrap();
        let minus = parse_iso_calendar_duration("-P1M").unwrap();
        assert_eq!(
            apply_iso_calendar_duration(date(2024, 1, 31), plus),
            Some(date(2024, 2, 29))
        );
        assert_eq!(
            apply_iso_calendar_duration(date(2024, 3, 31), minus),
            Some(date(2024, 2, 29))
        );
        assert_eq!(
            apply_iso_calendar_duration(
                date(2024, 1, 31),
                parse_iso_calendar_duration("P1Y2M3D").unwrap()
            ),
            Some(date(2025, 4, 3))
        );
        assert_eq!(
            apply_iso_calendar_duration(
                date(2024, 2, 28),
                parse_iso_calendar_duration("P2W").unwrap()
            ),
            Some(date(2024, 3, 13))
        );
    }

    #[test]
    fn round_trips_canonical_signed_periods() {
        for input in ["P0D", "P1Y", "P2M", "P3W", "P1Y2M3D", "-P2Y4M1D"] {
            let parsed = parse_iso_calendar_duration(input).unwrap();
            assert_eq!(format_iso_calendar_duration(parsed), input);
        }
        assert_eq!(
            format_iso_calendar_duration(parse_iso_calendar_duration("-P0D").unwrap()),
            "P0D"
        );
    }

    #[test]
    fn rejects_invalid_order_mixed_weeks_and_overflow() {
        for value in [
            "", "P", "PT1H", "P1.5D", "P1W2D", "P0W0D", "P1D1M",
            "P1Y1Y", "P-1D", "P1Q", "+P1D", "P1", "P4294967296D",
        ] {
            assert!(parse_iso_calendar_duration(value).is_none(), "{value}");
        }
        let big = parse_iso_calendar_duration("P4294967295Y").unwrap();
        assert_eq!(apply_iso_calendar_duration(date(2024, 1, 1), big), None);
        let one_day = parse_iso_calendar_duration("P1D").unwrap();
        assert_eq!(apply_iso_calendar_duration(NaiveDate::MAX, one_day), None);
    }

    #[test]
    fn supports_zero_at_chrono_extremes() {
        let zero = parse_iso_calendar_duration("P0D").unwrap();
        assert_eq!(apply_iso_calendar_duration(NaiveDate::MAX, zero), Some(NaiveDate::MAX));
        assert_eq!(apply_iso_calendar_duration(NaiveDate::MIN, zero), Some(NaiveDate::MIN));
    }
}
