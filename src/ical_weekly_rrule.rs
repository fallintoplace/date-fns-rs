//! Bounded RFC 5545 weekly recurrence subset without time-zone inference.

use chrono::{Datelike, NaiveDate};

/// A weekly iCalendar rule with BYDAY and optional COUNT or UNTIL.
///
/// Only FREQ=WEEKLY, BYDAY, INTERVAL and COUNT fields are supported.
/// Monday is the first weekday (WKST=MO).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IcalWeeklyRRule {
    weekday_mask: u8,
    interval_weeks: u32,
    count: Option<usize>,
    until: Option<NaiveDate>,
}

/// Parse a strict weekly RRULE body such as FREQ=WEEKLY;BYDAY=MO,WE;COUNT=4.
///
/// Unrecognized fields and repeated keys are rejected rather than ignored.
/// A caller-supplied end date always bounds evaluation, even without COUNT.
pub fn parse_ical_weekly_rrule(input: &str) -> Option<IcalWeeklyRRule> {
    let mut saw_frequency = false;
    let mut weekdays = None;
    let mut interval = None;
    let mut count = None;
    let mut until = None;
    for part in input.split(';') {
        let (key, value) = part.split_once('=')?;
        match key {
            "FREQ" if !saw_frequency && value == "WEEKLY" => {
                saw_frequency = true;
            }
            "BYDAY" if weekdays.is_none() => {
                let mut mask = 0_u8;
                for day in value.split(',') {
                    let bit = match day {
                        "MO" => 0,
                        "TU" => 1,
                        "WE" => 2,
                        "TH" => 3,
                        "FR" => 4,
                        "SA" => 5,
                        "SU" => 6,
                        _ => return None,
                    };
                    mask |= 1_u8 << bit;
                }
                if mask == 0 {
                    return None;
                }
                weekdays = Some(mask);
            }
            "INTERVAL" if interval.is_none() => {
                let weeks = value.parse::<u32>().ok()?;
                if weeks == 0 || !value.as_bytes().iter().all(u8::is_ascii_digit) {
                    return None;
                }
                interval = Some(weeks);
            }
            "COUNT" if count.is_none() => {
                let occurrences = value.parse::<usize>().ok()?;
                if occurrences == 0 || !value.as_bytes().iter().all(u8::is_ascii_digit) {
                    return None;
                }
                count = Some(occurrences);
            }
            "UNTIL" if until.is_none() => until = Some(crate::parse_ical_date(value)?),
            _ => return None,
        }
    }
    if count.is_some() && until.is_some() {
        return None;
    }
    Some(IcalWeeklyRRule {
        weekday_mask: weekdays?,
        interval_weeks: interval.unwrap_or(1),
        count,
        until,
    })
    .filter(|_| saw_frequency)
}

/// Enumerate a weekly rule within an inclusive bounded date interval.
///
/// DTSTART is the anchor for alternating weeks and the first occurrence.
/// A DTSTART whose weekday is absent from BYDAY is rejected because RFC 5545
/// defines the resulting recurrence set as unspecified.
pub fn each_ical_weekly_rrule(
    rule: IcalWeeklyRRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Option<impl Iterator<Item = NaiveDate>> {
    if rule.weekday_mask & (1_u8 << start.weekday().num_days_from_monday()) == 0 {
        return None;
    }
    let through = rule.until.map_or(end, |until| end.min(until));
    let dates = crate::each_selected_weekday_in_interval(
        start,
        through,
        rule.weekday_mask,
        rule.interval_weeks,
    )?;
    Some(dates.take(rule.count.unwrap_or(usize::MAX)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_week_skips_multiple_days_and_count() {
        let rule = parse_ical_weekly_rrule("FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE;COUNT=3").unwrap();
        let days: Vec<_> = each_ical_weekly_rrule(rule, date(2024, 3, 4), date(2024, 3, 31))
            .unwrap()
            .collect();
        assert_eq!(
            days,
            vec![date(2024, 3, 4), date(2024, 3, 6), date(2024, 3, 18)]
        );
    }

    #[test]
    fn rejects_dtstart_not_in_weekday_rule() {
        let rule = parse_ical_weekly_rrule("FREQ=WEEKLY;BYDAY=MO").unwrap();
        assert!(each_ical_weekly_rrule(rule, date(2024, 3, 1), date(2024, 3, 31)).is_none());
    }

    #[test]
    fn until_is_inclusive_and_cannot_be_combined_with_count() {
        let rule = parse_ical_weekly_rrule("FREQ=WEEKLY;BYDAY=MO,WE;UNTIL=20240313").unwrap();
        assert_eq!(
            each_ical_weekly_rrule(rule, date(2024, 3, 4), date(2024, 3, 31))
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 3, 4), date(2024, 3, 6), date(2024, 3, 11), date(2024, 3, 13)]
        );
        assert!(parse_ical_weekly_rrule(
            "FREQ=WEEKLY;BYDAY=MO;COUNT=2;UNTIL=20240331"
        )
        .is_none());
    }

    #[test]
    fn rejects_unsupported_fields_and_duplicate_keys() {
        for invalid in [
            "FREQ=DAILY;BYDAY=MO",
            "FREQ=WEEKLY",
            "FREQ=WEEKLY;BYDAY=2MO",
            "FREQ=WEEKLY;BYDAY=MO;COUNT=0",
            "FREQ=WEEKLY;BYDAY=MO;INTERVAL=0",
            "FREQ=WEEKLY;BYDAY=MO;UNTIL=20240230",
            "FREQ=WEEKLY;BYDAY=MO;BYDAY=TU",
            "FREQ=WEEKLY;BYDAY=MO;FREQ=WEEKLY",
        ] {
            assert!(parse_ical_weekly_rrule(invalid).is_none(), "{invalid}");
        }
    }
}
