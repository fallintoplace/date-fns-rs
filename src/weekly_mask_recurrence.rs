//! Lazy multi-week recurrence over a Monday-first weekday bitmask.

use std::iter::FusedIterator;

use chrono::{Datelike, NaiveDate};

/// An iterator over selected weekdays in every Nth calendar week.
#[derive(Clone, Debug)]
pub struct WeeklyMaskRecurrences {
    week_start: Option<i64>,
    start_day: i64,
    end_day: i64,
    step_days: i64,
    mask: u8,
    weekday: u8,
}

impl Iterator for WeeklyMaskRecurrences {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(week) = self.week_start {
            if self.weekday == 7 {
                self.week_start = week
                    .checked_add(self.step_days)
                    .filter(|&next| next <= self.end_day);
                self.weekday = 0;
                continue;
            }
            let day_offset = self.weekday;
            self.weekday += 1;
            let ordinal = week + i64::from(day_offset);
            if ordinal > self.end_day {
                self.week_start = None;
                return None;
            }
            if ordinal >= self.start_day && (self.mask & (1_u8 << day_offset)) != 0 {
                return NaiveDate::from_num_days_from_ce_opt(i32::try_from(ordinal).ok()?);
            }
        }
        None
    }
}

impl FusedIterator for WeeklyMaskRecurrences {}

/// Iterate matching weekdays in every `step_weeks` week of an inclusive range.
///
/// Bit 0 represents Monday and bit 6 Sunday. The week containing `start`
/// is week zero, even when `start` is not Monday. Skipped weeks are jumped
/// over without scanning individual dates. Returns None for reversed bounds,
/// an empty or invalid mask, or a zero step.
pub fn each_selected_weekday_in_interval(
    start: NaiveDate,
    end: NaiveDate,
    mask: u8,
    step_weeks: u32,
) -> Option<WeeklyMaskRecurrences> {
    if start > end || mask == 0 || mask & !0x7f_u8 != 0 || step_weeks == 0 {
        return None;
    }
    let start_day = i64::from(start.num_days_from_ce());
    let week = start_day - i64::from(start.weekday().num_days_from_monday());
    Some(WeeklyMaskRecurrences {
        week_start: Some(week),
        start_day,
        end_day: i64::from(end.num_days_from_ce()),
        step_days: i64::from(step_weeks) * 7,
        mask,
        weekday: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn selects_two_weekly_days_with_alternating_weeks() {
        let actual: Vec<_> = each_selected_weekday_in_interval(
            date(2024, 3, 6),
            date(2024, 3, 31),
            0b0001_0101,
            2,
        )
        .unwrap()
        .collect();
        assert_eq!(
            actual,
            vec![
                date(2024, 3, 6),
                date(2024, 3, 15),
                date(2024, 3, 18),
                date(2024, 3, 20),
                date(2024, 3, 22),
            ]
        );
    }

    #[test]
    fn matches_day_by_day_reference_across_leap_year() {
        let start = date(2024, 2, 28);
        let end = date(2024, 5, 17);
        let monday = i64::from(start.num_days_from_ce())
            - i64::from(start.weekday().num_days_from_monday());
        for mask in [1, 0b0101_0101, 0b0111_1111] {
            for step in [1_u32, 2, 3, 7] {
                let expected: Vec<_> = crate::each_day_of_interval(start, end)
                    .unwrap()
                    .filter(|day| {
                        let week = (i64::from(day.num_days_from_ce()) - monday) / 7;
                        let bit = 1_u8 << day.weekday().num_days_from_monday();
                        week % i64::from(step) == 0 && (mask & bit) != 0
                    })
                    .collect();
                let actual: Vec<_> = each_selected_weekday_in_interval(start, end, mask, step)
                    .unwrap()
                    .collect();
                assert_eq!(actual, expected, "{mask}, {step}");
            }
        }
    }

    #[test]
    fn rejects_bad_parameters_and_fuses_at_limits() {
        let start = date(2024, 2, 29);
        assert!(each_selected_weekday_in_interval(start, start, 0, 1).is_none());
        assert!(each_selected_weekday_in_interval(start, start, 0x80, 1).is_none());
        assert!(each_selected_weekday_in_interval(start, start, 1, 0).is_none());
        assert!(each_selected_weekday_in_interval(start, date(2024, 2, 1), 1, 1).is_none());
        for boundary in [NaiveDate::MIN, NaiveDate::MAX] {
            let mask = 1_u8 << boundary.weekday().num_days_from_monday();
            let mut recurrences = each_selected_weekday_in_interval(
                boundary,
                boundary,
                mask,
                u32::MAX,
            )
            .unwrap();
            assert_eq!(recurrences.next(), Some(boundary));
            assert_eq!(recurrences.next(), None);
            assert_eq!(recurrences.next(), None);
        }
    }
}
