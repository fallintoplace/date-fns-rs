//! Strict five-field cron schedules evaluated without assuming a time zone.

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

/// A five-field numeric cron expression at minute precision.
///
/// Fields are minute, hour, day-of-month, month, day-of-week. Sunday is 0 or
/// 7. When both day fields are restricted they use the standard cron OR rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CronSchedule {
    minutes: u64,
    hours: u64,
    month_days: u64,
    months: u64,
    weekdays: u64,
    every_month_day: bool,
    every_weekday: bool,
}

fn numeric(text: &str) -> Option<u32> {
    if text.is_empty() || !text.as_bytes().iter().all(u8::is_ascii_digit) {
        return None;
    }
    text.parse().ok()
}

fn parse_field(text: &str, min: u32, max: u32, sunday_alias: bool) -> Option<(u64, bool)> {
    let limit = if sunday_alias { 7 } else { max };
    let mut mask = 0_u64;
    for part in text.split(',') {
        let (range, step) = match part.split_once('/') {
            Some((base, value)) => (base, numeric(value)?),
            None => (part, 1),
        };
        if step == 0 {
            return None;
        }
        let (first, last) = if range == "*" {
            (min, limit)
        } else if let Some((lo, hi)) = range.split_once('-') {
            (numeric(lo)?, numeric(hi)?)
        } else {
            let first = numeric(range)?;
            (first, if part.contains('/') { limit } else { first })
        };
        if first < min || last > limit || first > last {
            return None;
        }
        for value in (first..=last).step_by(step as usize) {
            let bit = if sunday_alias && value == 7 { 0 } else { value };
            mask |= 1_u64 << bit;
        }
    }
    let wildcard = text == "*"
        || text
            .split_once('/')
            .is_some_and(|(base, step)| base == "*" && numeric(step) == Some(1));
    (mask != 0).then_some((mask, wildcard))
}

/// Parse numeric five-field cron with lists, ranges and steps.
///
/// Examples: `*/15 9-17 * * 1-5`, `0 0 29 2 *`. Rejects names, seconds,
/// unsupported macros, reversed ranges, and zero or overflowing steps.
pub fn parse_cron_schedule(text: &str) -> Option<CronSchedule> {
    let fields: Vec<_> = text.split_whitespace().collect();
    if fields.len() != 5 {
        return None;
    }
    let (minutes, _) = parse_field(fields[0], 0, 59, false)?;
    let (hours, _) = parse_field(fields[1], 0, 23, false)?;
    let (month_days, every_month_day) = parse_field(fields[2], 1, 31, false)?;
    let (months, _) = parse_field(fields[3], 1, 12, false)?;
    let (weekdays, every_weekday) = parse_field(fields[4], 0, 6, true)?;
    Some(CronSchedule {
        minutes,
        hours,
        month_days,
        months,
        weekdays,
        every_month_day,
        every_weekday,
    })
}

/// Parse a numeric cron schedule or a standard fixed-time macro.
///
/// Supported macros: @yearly, @annually, @monthly, @weekly, @daily,
/// @midnight, and @hourly. @reboot is not a fixed calendar schedule.
/// The original parse_cron_schedule remains strict and numeric-only.
pub fn parse_cron_schedule_with_aliases(input: &str) -> Option<CronSchedule> {
    let expanded = match input.trim() {
        "@yearly" | "@annually" => "0 0 1 1 *",
        "@monthly" => "0 0 1 * *",
        "@weekly" => "0 0 * * 0",
        "@daily" | "@midnight" => "0 0 * * *",
        "@hourly" => "0 * * * *",
        _ => input,
    };
    parse_cron_schedule(expanded)
}

impl CronSchedule {
    fn matches_date(&self, date: NaiveDate) -> bool {
        if self.months & (1_u64 << date.month()) == 0 {
            return false;
        }
        let matches_month_day = self.month_days & (1_u64 << date.day()) != 0;
        let matches_weekday = self.weekdays & (1_u64 << date.weekday().num_days_from_sunday()) != 0;
        match (self.every_month_day, self.every_weekday) {
            (true, true) => true,
            (true, false) => matches_weekday,
            (false, true) => matches_month_day,
            (false, false) => matches_month_day || matches_weekday,
        }
    }

    /// Test an exact minute boundary against this schedule.
    ///
    /// Seconds and nanoseconds must be zero.
    pub fn matches(&self, datetime: NaiveDateTime) -> bool {
        datetime.second() == 0
            && datetime.nanosecond() == 0
            && self.matches_date(datetime.date())
            && self.hours & (1_u64 << datetime.hour()) != 0
            && self.minutes & (1_u64 << datetime.minute()) != 0
    }

    /// Find the next scheduled minute strictly after `after`.
    ///
    /// Search includes the remainder of the starting date and at most
    /// `search_days` further calendar days. This explicit bound avoids
    /// infinite loops for impossible expressions such as February 31.
    /// Calendar days are skipped without iterating individual minutes.
    pub fn next_after(&self, after: NaiveDateTime, search_days: u32) -> Option<NaiveDateTime> {
        let mut day = after.date();
        for _ in 0..=search_days {
            if self.matches_date(day) {
                for hour in 0..24 {
                    if self.hours & (1_u64 << hour) == 0 {
                        continue;
                    }
                    for minute in 0..60 {
                        if self.minutes & (1_u64 << minute) == 0 {
                            continue;
                        }
                        let candidate = day.and_hms_opt(hour, minute, 0)?;
                        if candidate > after {
                            return Some(candidate);
                        }
                    }
                }
            }
            day = day.succ_opt()?;
        }
        None
    }

    /// Find the previous scheduled minute strictly before `before`.
    ///
    /// Examines the starting date and at most `search_days` earlier dates.
    /// Scans matching clock values in descending order, without visiting
    /// unrelated minutes or assuming a time zone.
    pub fn previous_before(
        &self,
        before: NaiveDateTime,
        search_days: u32,
    ) -> Option<NaiveDateTime> {
        let mut day = before.date();
        for offset in 0..=search_days {
            if self.matches_date(day) {
                for hour in (0..24).rev() {
                    if self.hours & (1_u64 << hour) == 0 {
                        continue;
                    }
                    for minute in (0..60).rev() {
                        if self.minutes & (1_u64 << minute) == 0 {
                            continue;
                        }
                        let candidate = day.and_hms_opt(hour, minute, 0)?;
                        if candidate < before {
                            return Some(candidate);
                        }
                    }
                }
            }
            if offset == search_days {
                break;
            }
            day = day.pred_opt()?;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stamp(y: i32, m: u32, d: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn parses_steps_ranges_lists_and_matches_exact_minute_ticks() {
        let schedule = parse_cron_schedule("*/15 9-10 * * 1-5").unwrap();
        assert!(schedule.matches(stamp(2024, 3, 1, 9, 15)));
        assert!(schedule.matches(stamp(2024, 3, 1, 10, 45)));
        assert!(!schedule.matches(stamp(2024, 3, 2, 9, 15)));
        assert!(!schedule.matches(stamp(2024, 3, 1, 9, 14)));
        assert!(!schedule.matches(
            stamp(2024, 3, 1, 9, 15)
                .checked_add_signed(chrono::TimeDelta::nanoseconds(1))
                .unwrap()
        ));
        assert_eq!(
            schedule.next_after(stamp(2024, 3, 1, 9, 15), 0),
            Some(stamp(2024, 3, 1, 9, 30))
        );
    }

    #[test]
    fn observes_restricted_day_of_month_or_weekday_semantics() {
        let schedule = parse_cron_schedule("0 9 1 * 1").unwrap();
        assert!(schedule.matches(stamp(2026, 11, 1, 9, 0)));
        assert!(schedule.matches(stamp(2026, 11, 2, 9, 0)));
        assert!(!schedule.matches(stamp(2026, 11, 3, 9, 0)));
        assert!(parse_cron_schedule("0 0 * * 7")
            .unwrap()
            .matches(stamp(2026, 11, 1, 0, 0)));
        let starred_step = parse_cron_schedule("0 9 */1 * 1").unwrap();
        assert!(starred_step.matches(stamp(2026, 11, 2, 9, 0)));
        assert!(!starred_step.matches(stamp(2026, 11, 3, 9, 0)));
    }

    #[test]
    fn finds_leap_day_and_bounds_impossible_recurrences() {
        let leap = parse_cron_schedule("0 0 29 2 *").unwrap();
        assert_eq!(
            leap.next_after(stamp(2023, 3, 1, 0, 0), 370),
            Some(stamp(2024, 2, 29, 0, 0))
        );
        assert_eq!(
            parse_cron_schedule("0 0 31 2 *")
                .unwrap()
                .next_after(stamp(2024, 1, 1, 0, 0), 800),
            None
        );
        assert_eq!(
            parse_cron_schedule("* * * * *")
                .unwrap()
                .next_after(NaiveDateTime::MAX, 0),
            None
        );
    }

    #[test]
    fn finds_previous_minutes_across_weekends_and_year_boundaries() {
        let workdays = parse_cron_schedule("0,30 9-10 * * 1-5").unwrap();
        assert_eq!(
            workdays.previous_before(stamp(2024, 3, 4, 9, 0), 3),
            Some(stamp(2024, 3, 1, 10, 30))
        );
        assert_eq!(
            workdays.previous_before(stamp(2024, 3, 4, 9, 31), 0),
            Some(stamp(2024, 3, 4, 9, 30))
        );
        assert_eq!(workdays.previous_before(stamp(2024, 3, 4, 9, 0), 0), None);
        let midnight = parse_cron_schedule("0 0 1 1 *").unwrap();
        assert_eq!(
            midnight.previous_before(stamp(2026, 1, 1, 0, 0), 366),
            Some(stamp(2025, 1, 1, 0, 0))
        );
    }

    #[test]
    fn reverse_search_rejects_impossible_matches_and_date_underflow() {
        let impossible = parse_cron_schedule("0 0 31 2 *").unwrap();
        assert_eq!(
            impossible.previous_before(stamp(2024, 3, 1, 0, 0), 366),
            None
        );
        assert_eq!(
            parse_cron_schedule("* * * * *")
                .unwrap()
                .previous_before(NaiveDateTime::MIN, 0),
            None
        );
    }

    #[test]
    fn macro_parser_expands_fixed_calendar_schedules() {
        assert!(parse_cron_schedule("@daily").is_none());
        let hourly = parse_cron_schedule_with_aliases("@hourly").unwrap();
        assert!(hourly.matches(stamp(2024, 3, 1, 13, 0)));
        assert!(!hourly.matches(stamp(2024, 3, 1, 13, 1)));
        let weekly = parse_cron_schedule_with_aliases("@weekly").unwrap();
        assert!(weekly.matches(stamp(2026, 11, 1, 0, 0)));
        assert!(!weekly.matches(stamp(2026, 11, 2, 0, 0)));
        for alias in ["@yearly", "@annually"] {
            let schedule = parse_cron_schedule_with_aliases(alias).unwrap();
            assert!(schedule.matches(stamp(2026, 1, 1, 0, 0)));
            assert!(!schedule.matches(stamp(2026, 2, 1, 0, 0)));
        }
        let monthly = parse_cron_schedule_with_aliases("@monthly").unwrap();
        assert!(monthly.matches(stamp(2026, 2, 1, 0, 0)));
        let midnight = parse_cron_schedule_with_aliases("@midnight").unwrap();
        assert!(midnight.matches(stamp(2026, 2, 2, 0, 0)));
        assert!(parse_cron_schedule_with_aliases("@reboot").is_none());
        assert!(parse_cron_schedule_with_aliases("@hourly more").is_none());
        assert!(parse_cron_schedule_with_aliases("*/10 * * * *").is_some());
    }

    #[test]
    fn rejects_malformed_or_unsupported_cron_syntax() {
        for expression in [
            "",
            "* * * *",
            "* * * * * *",
            "*/0 * * * *",
            "60 * * * *",
            "0 24 * * *",
            "0 0 32 * *",
            "0 0 * 13 *",
            "0 0 * * 8",
            "0 0 * * MON",
            "0 0 * * 5-1",
            "0 0 * * 1,,2",
            "@daily",
            "0 0 * * 1/0",
            "0 0 * * 1-2-3",
        ] {
            assert!(parse_cron_schedule(expression).is_none(), "{expression}");
        }
    }
}
