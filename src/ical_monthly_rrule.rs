//! Bounded monthly iCalendar recurrences for a fixed positive month day.

use chrono::{Datelike, Months, NaiveDate};

/// Monthly BYMONTHDAY recurrence with optional interval and count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IcalMonthlyRRule {
    month_day: u32,
    interval_months: u32,
    count: Option<usize>,
}

fn positive(value: &str) -> Option<u32> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    value.parse::<u32>().ok().filter(|&number| number > 0)
}

/// Parse FREQ=MONTHLY with required BYMONTHDAY=1..31.
///
/// INTERVAL and COUNT are optional positive integers. Ordinal BYDAY,
/// negative month days and all other RRULE components are unsupported.
pub fn parse_ical_monthly_rrule(input: &str) -> Option<IcalMonthlyRRule> {
    let mut frequency = false;
    let mut month_day = None;
    let mut interval = None;
    let mut count = None;
    for part in input.split(';') {
        let (key, value) = part.split_once('=')?;
        match key {
            "FREQ" if !frequency && value == "MONTHLY" => frequency = true,
            "BYMONTHDAY" if month_day.is_none() => {
                month_day = Some(positive(value)?);
            }
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
    let month_day = month_day?;
    (frequency && month_day <= 31).then_some(IcalMonthlyRRule {
        month_day,
        interval_months: interval.unwrap_or(1),
        count,
    })
}

/// Enumerate matching dates, skipping months without BYMONTHDAY.
///
/// DTSTART must match BYMONTHDAY, as required for a well-defined recurrence.
/// Month steps are anchored on the first day, avoiding drift after short months.
pub fn each_ical_monthly_rrule(
    rule: IcalMonthlyRRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Option<impl Iterator<Item = NaiveDate>> {
    if start > end || start.day() != rule.month_day {
        return None;
    }
    let first = start.with_day(1)?;
    Some(
        std::iter::successors(Some(first), move |month| {
            month.checked_add_months(Months::new(rule.interval_months))
        })
        .filter_map(move |month| month.with_day(rule.month_day))
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
    fn skips_months_without_the_requested_day() {
        let rule = parse_ical_monthly_rrule("FREQ=MONTHLY;BYMONTHDAY=31;COUNT=3").unwrap();
        let dates: Vec<_> = each_ical_monthly_rrule(rule, date(2024, 1, 31), date(2024, 6, 30))
            .unwrap()
            .collect();
        assert_eq!(
            dates,
            vec![date(2024, 1, 31), date(2024, 3, 31), date(2024, 5, 31)]
        );
    }

    #[test]
    fn respects_month_intervals_and_rejects_misaligned_start() {
        let rule = parse_ical_monthly_rrule("FREQ=MONTHLY;BYMONTHDAY=1;INTERVAL=2").unwrap();
        assert_eq!(
            each_ical_monthly_rrule(rule, date(2024, 1, 1), date(2024, 6, 30))
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 1, 1), date(2024, 3, 1), date(2024, 5, 1)]
        );
        assert!(each_ical_monthly_rrule(rule, date(2024, 1, 2), date(2024, 6, 30)).is_none());
        assert!(each_ical_monthly_rrule(rule, date(2024, 2, 1), date(2024, 1, 1)).is_none());
    }

    #[test]
    fn rejects_unsupported_fields_duplicate_keys_and_zero_values() {
        for input in [
            "FREQ=MONTHLY",
            "FREQ=MONTHLY;BYMONTHDAY=-1",
            "FREQ=MONTHLY;BYMONTHDAY=0",
            "FREQ=MONTHLY;BYMONTHDAY=32",
            "FREQ=MONTHLY;BYMONTHDAY=1;COUNT=0",
            "FREQ=MONTHLY;BYMONTHDAY=1;INTERVAL=0",
            "FREQ=MONTHLY;BYMONTHDAY=1;BYMONTHDAY=2",
            "FREQ=MONTHLY;BYMONTHDAY=1;BYDAY=MO",
            "FREQ=DAILY;BYMONTHDAY=1",
        ] {
            assert!(parse_ical_monthly_rrule(input).is_none(), "{input}");
        }
    }
}
