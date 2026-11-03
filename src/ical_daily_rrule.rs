//! Bounded RFC 5545 daily recurrence subset without time-zone inference.

use chrono::{Days, NaiveDate};

/// A daily recurrence with an optional occurrence cap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IcalDailyRRule {
    interval_days: u32,
    count: Option<usize>,
    until: Option<NaiveDate>,
}

/// Parse FREQ=DAILY with optional positive INTERVAL, COUNT, or UNTIL.
///
/// COUNT and UNTIL are mutually exclusive. Duplicate or unknown fields are
/// rejected. The caller provides the end
/// date when evaluating a rule, so a missing COUNT is still bounded.
pub fn parse_ical_daily_rrule(input: &str) -> Option<IcalDailyRRule> {
    let mut frequency = false;
    let mut interval = None;
    let mut count = None;
    let mut until = None;
    for part in input.split(';') {
        let (key, value) = part.split_once('=')?;
        let positive = || {
            (!value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
                .then(|| value.parse::<usize>().ok())
                .flatten()
                .filter(|&number| number > 0)
        };
        match key {
            "FREQ" if !frequency && value == "DAILY" => frequency = true,
            "INTERVAL" if interval.is_none() => {
                interval = Some(u32::try_from(positive()?).ok()?);
            }
            "COUNT" if count.is_none() => count = Some(positive()?),
            "UNTIL" if until.is_none() => until = Some(crate::parse_ical_date(value)?),
            _ => return None,
        }
    }
    if count.is_some() && until.is_some() {
        return None;
    }
    frequency.then_some(IcalDailyRRule {
        interval_days: interval.unwrap_or(1),
        count,
        until,
    })
}

/// Enumerate a daily rule from DTSTART through the inclusive end date.
///
/// DTSTART is always the first occurrence. Returns None for reversed dates.
pub fn each_ical_daily_rrule(
    rule: IcalDailyRRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Option<impl Iterator<Item = NaiveDate>> {
    if start > end {
        return None;
    }
    let through = rule.until.map_or(end, |until| end.min(until));
    Some(
        std::iter::successors(Some(start), move |date| {
            date.checked_add_days(Days::new(u64::from(rule.interval_days)))
        })
        .take_while(move |date| *date <= through)
        .take(rule.count.unwrap_or(usize::MAX)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn iterates_across_leap_day_with_interval_and_count() {
        let rule = parse_ical_daily_rrule("FREQ=DAILY;INTERVAL=2;COUNT=3").unwrap();
        let days: Vec<_> = each_ical_daily_rrule(rule, date(2024, 2, 28), date(2024, 3, 15))
            .unwrap()
            .collect();
        assert_eq!(
            days,
            vec![date(2024, 2, 28), date(2024, 3, 1), date(2024, 3, 3)]
        );
    }

    #[test]
    fn honors_inclusive_bounds_and_chrono_limits() {
        let rule = parse_ical_daily_rrule("FREQ=DAILY").unwrap();
        assert_eq!(
            each_ical_daily_rrule(rule, date(2024, 2, 28), date(2024, 2, 29))
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 2, 28), date(2024, 2, 29)]
        );
        assert_eq!(
            each_ical_daily_rrule(rule, NaiveDate::MAX, NaiveDate::MAX)
                .unwrap()
                .collect::<Vec<_>>(),
            vec![NaiveDate::MAX]
        );
        assert!(each_ical_daily_rrule(rule, date(2024, 3, 2), date(2024, 3, 1)).is_none());
    }

    #[test]
    fn until_is_inclusive_and_bounded_by_the_requested_end() {
        let rule = parse_ical_daily_rrule("FREQ=DAILY;INTERVAL=2;UNTIL=20240303").unwrap();
        assert_eq!(
            each_ical_daily_rrule(rule, date(2024, 2, 28), date(2024, 3, 20))
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 2, 28), date(2024, 3, 1), date(2024, 3, 3)]
        );
        assert_eq!(
            each_ical_daily_rrule(rule, date(2024, 3, 4), date(2024, 3, 20))
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn rejects_unsupported_and_ambiguous_rules() {
        for input in [
            "",
            "FREQ=WEEKLY",
            "INTERVAL=2",
            "FREQ=DAILY;INTERVAL=0",
            "FREQ=DAILY;COUNT=0",
            "FREQ=DAILY;COUNT=-2",
            "FREQ=DAILY;INTERVAL=1;INTERVAL=2",
            "FREQ=DAILY;FREQ=DAILY",
            "FREQ=DAILY;BYDAY=MO",
            "FREQ=DAILY;COUNT=1;UNTIL=20240301",
            "FREQ=DAILY;UNTIL=20240230",
            "FREQ=DAILY;UNTIL=20240301;UNTIL=20240302",
        ] {
            assert!(parse_ical_daily_rrule(input).is_none(), "{input}");
        }
    }
}
