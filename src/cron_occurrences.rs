//! Lazy, bounded enumeration of matching five-field cron minutes.

use std::iter::FusedIterator;

use chrono::NaiveDateTime;

use crate::CronSchedule;

/// An iterator of matching clock minutes inside a caller-defined horizon.
#[derive(Clone, Debug)]
pub struct CronOccurrences {
    schedule: CronSchedule,
    after: NaiveDateTime,
    through: NaiveDateTime,
    remaining: usize,
}

impl Iterator for CronOccurrences {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let days = self.through.date().signed_duration_since(self.after.date()).num_days();
        let search_days = u32::try_from(days).ok()?;
        let candidate = match self.schedule.next_after(self.after, search_days) {
            Some(candidate) if candidate <= self.through => candidate,
            _ => {
                self.remaining = 0;
                return None;
            }
        };
        self.after = candidate;
        self.remaining -= 1;
        Some(candidate)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}

impl FusedIterator for CronOccurrences {}

/// Iterate matching schedule minutes in (after, through], up to max_results.
///
/// The start is exclusive and the end inclusive. Returns None for reversed
/// bounds. A zero maximum creates an empty iterator. No timezone is assumed,
/// and impossible cron expressions terminate at the explicit end bound.
pub fn each_cron_minute_between(
    schedule: &CronSchedule,
    after: NaiveDateTime,
    through: NaiveDateTime,
    max_results: usize,
) -> Option<CronOccurrences> {
    if after > through {
        return None;
    }
    Some(CronOccurrences {
        schedule: schedule.clone(),
        after,
        through,
        remaining: max_results,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn stamp(y: i32, m: u32, d: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn iterates_in_order_and_honors_result_limit() {
        let schedule = crate::parse_cron_schedule("*/15 9-10 * * *").unwrap();
        let start = stamp(2024, 3, 1, 9, 0);
        let end = stamp(2024, 3, 1, 10, 0);
        let actual: Vec<_> = each_cron_minute_between(&schedule, start, end, 10)
            .unwrap()
            .collect();
        assert_eq!(
            actual,
            vec![
                stamp(2024, 3, 1, 9, 15),
                stamp(2024, 3, 1, 9, 30),
                stamp(2024, 3, 1, 9, 45),
                stamp(2024, 3, 1, 10, 0),
            ]
        );
        let first: Vec<_> = each_cron_minute_between(&schedule, start, end, 2)
            .unwrap()
            .collect();
        assert_eq!(first, actual[..2]);
    }

    #[test]
    fn skips_weekends_and_bounds_impossible_recurrences() {
        let weekdays = crate::parse_cron_schedule("0 9 * * 1-5").unwrap();
        let days: Vec<_> = each_cron_minute_between(
            &weekdays,
            stamp(2024, 3, 1, 9, 0),
            stamp(2024, 3, 4, 9, 0),
            10,
        )
        .unwrap()
        .collect();
        assert_eq!(days, vec![stamp(2024, 3, 4, 9, 0)]);

        let impossible = crate::parse_cron_schedule("0 0 31 2 *").unwrap();
        assert_eq!(
            each_cron_minute_between(
                &impossible,
                stamp(2024, 1, 1, 0, 0),
                stamp(2026, 1, 1, 0, 0),
                10
            )
            .unwrap()
            .next(),
            None
        );
    }

    #[test]
    fn terminates_for_zero_limits_and_empty_horizons() {
        let every_minute = crate::parse_cron_schedule("* * * * *").unwrap();
        let start = stamp(2024, 3, 1, 9, 0);
        let mut empty = each_cron_minute_between(&every_minute, start, start, 10).unwrap();
        assert_eq!(empty.next(), None);
        assert_eq!(empty.next(), None);
        assert_eq!(
            each_cron_minute_between(&every_minute, start, start, 0)
                .unwrap()
                .next(),
            None
        );
        assert!(each_cron_minute_between(
            &every_minute,
            stamp(2024, 3, 2, 0, 0),
            start,
            10
        )
        .is_none());
    }
}
