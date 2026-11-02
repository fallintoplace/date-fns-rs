//! Bounded yearly iCalendar recurrences for a month and day.

use chrono::{Datelike, NaiveDate};

/// A yearly BYMONTH/BYMONTHDAY recurrence with optional interval and count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IcalYearlyRRule {
    month: u32,
    month_day: u32,
    interval_years: u32,
    count: Option<usize>,
}

fn positive(value: &str) -> Option<u32> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    value.parse::<u32>().ok().filter(|&number| number > 0)
}

/// Parse FREQ=YEARLY with required BYMONTH and BYMONTHDAY.
///
/// Only positive, single-valued fields are supported. An explicit caller end
/// date bounds iteration even when COUNT is omitted.
pub fn parse_ical_yearly_rrule(input: &str) -> Option<IcalYearlyRRule> {
    let mut frequency = false;
    let mut month = None;
    let mut month_day = None;
    let mut interval = None;
    let mut count = None;
    for part in input.split(';') {
        let (key, value) = part.split_once('=')?;
        match key {
            "FREQ" if !frequency && value == "YEARLY" => frequency = true,
            "BYMONTH" if month.is_none() => month = Some(positive(value)?),
            "BYMONTHDAY" if month_day.is_none() => month_day = Some(positive(value)?),
            "INTERVAL" if interval.is_none() => interval = Some(positive(value)?),
            "COUNT" if count.is_none() => {
                if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                    return None;
                }
                count = Some(value.parse::<usize>().ok().filter(|&n| n > 0)?);
            }
            _ => return None,
        }
    }
    let month = month?;
    let month_day = month_day?;
    (frequency
        && month <= 12
        && month_day <= 31
        && NaiveDate::from_ymd_opt(2000, month, month_day).is_some())
    .then_some(IcalYearlyRRule {
        month,
        month_day,
        interval_years: interval.unwrap_or(1),
        count,
    })
}

/// Enumerate calendar years with a matching BYMONTH/BYMONTHDAY.
///
/// DTSTART must match the rule. Missing days, including February 29 in
/// non-leap years, are skipped instead of clamped.
pub fn each_ical_yearly_rrule(
    rule: IcalYearlyRRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Option<impl Iterator<Item = NaiveDate>> {
    if start > end || start.month() != rule.month || start.day() != rule.month_day {
        return None;
    }
    Some(
        std::iter::successors(Some(i64::from(start.year())), move |year| {
            year.checked_add(i64::from(rule.interval_years))
        })
        .take_while(move |year| *year <= i64::from(end.year()))
        .filter_map(move |year| {
            NaiveDate::from_ymd_opt(i32::try_from(year).ok()?, rule.month, rule.month_day)
        })
        .take_while(move |date| *date <= end)
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
    fn leap_day_occurrences_skip_non_leap_years() {
        let rule = parse_ical_yearly_rrule("FREQ=YEARLY;BYMONTH=2;BYMONTHDAY=29;COUNT=3").unwrap();
        let dates: Vec<_> = each_ical_yearly_rrule(rule, date(2024, 2, 29), date(2035, 12, 31))
            .unwrap()
            .collect();
        assert_eq!(
            dates,
            vec![date(2024, 2, 29), date(2028, 2, 29), date(2032, 2, 29)]
        );
    }

    #[test]
    fn applies_interval_before_skipping_invalid_years() {
        let rule =
            parse_ical_yearly_rrule("FREQ=YEARLY;BYMONTH=2;BYMONTHDAY=29;INTERVAL=2").unwrap();
        assert_eq!(
            each_ical_yearly_rrule(rule, date(2024, 2, 29), date(2032, 2, 29))
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 2, 29), date(2028, 2, 29), date(2032, 2, 29)]
        );
        assert!(each_ical_yearly_rrule(rule, date(2024, 3, 1), date(2032, 2, 29)).is_none());
    }

    #[test]
    fn excludes_occurrences_after_the_end_day_in_same_year() {
        let rule = parse_ical_yearly_rrule("FREQ=YEARLY;BYMONTH=12;BYMONTHDAY=31").unwrap();
        assert_eq!(
            each_ical_yearly_rrule(rule, date(2024, 12, 31), date(2026, 6, 1))
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 12, 31), date(2025, 12, 31)]
        );
    }

    #[test]
    fn rejects_invalid_or_unsupported_rules() {
        for input in [
            "FREQ=YEARLY;BYMONTH=2",
            "FREQ=YEARLY;BYMONTHDAY=1",
            "FREQ=YEARLY;BYMONTH=0;BYMONTHDAY=1",
            "FREQ=YEARLY;BYMONTH=13;BYMONTHDAY=1",
            "FREQ=YEARLY;BYMONTH=1;BYMONTHDAY=32",
            "FREQ=YEARLY;BYMONTH=2;BYMONTHDAY=30",
            "FREQ=YEARLY;BYMONTH=4;BYMONTHDAY=31",
            "FREQ=YEARLY;BYMONTH=1;BYMONTHDAY=1;COUNT=0",
            "FREQ=YEARLY;BYMONTH=1;BYMONTHDAY=1;INTERVAL=0",
            "FREQ=YEARLY;BYMONTH=1;BYMONTHDAY=1;BYMONTH=2",
            "FREQ=YEARLY;BYMONTH=1;BYMONTHDAY=1;BYDAY=MO",
        ] {
            assert!(parse_ical_yearly_rrule(input).is_none(), "{input}");
        }
    }
}
