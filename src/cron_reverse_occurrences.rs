//! Lazy, bounded reverse enumeration of five-field cron matching minutes.

use std::iter::FusedIterator;

use chrono::NaiveDateTime;

use crate::CronSchedule;

/// An iterator over matching minutes in reverse chronological order.
#[derive(Clone, Debug)]
pub struct CronReverseOccurrences {
    schedule: CronSchedule,
    before: NaiveDateTime,
    since: NaiveDateTime,
    remaining: usize,
}

impl Iterator for CronReverseOccurrences {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let span = self
            .before
            .date()
            .signed_duration_since(self.since.date())
            .num_days();
        let days = u32::try_from(span).ok()?;
        let candidate = match self.schedule.previous_before(self.before, days) {
            Some(candidate) if candidate >= self.since => candidate,
            _ => {
                self.remaining = 0;
                return None;
            }
        };
        self.before = candidate;
        self.remaining -= 1;
        Some(candidate)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}

impl FusedIterator for CronReverseOccurrences {}

/// Enumerate matching minutes in [since, before), up to max_results.
///
/// Both endpoints are naive local timestamps. Reversed bounds return None.
pub fn each_previous_cron_minute_between(
    schedule: &CronSchedule,
    since: NaiveDateTime,
    before: NaiveDateTime,
    max_results: usize,
) -> Option<CronReverseOccurrences> {
    if since > before {
        return None;
    }
    Some(CronReverseOccurrences {
        schedule: schedule.clone(),
        before,
        since,
        remaining: max_results,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn stamp(day: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 3, day)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn yields_previous_minutes_with_exclusive_upper_bound() {
        let schedule = crate::parse_cron_schedule("0,30 9-10 * * 1-5").unwrap();
        let dates: Vec<_> = each_previous_cron_minute_between(
            &schedule,
            stamp(1, 9, 0),
            stamp(4, 9, 31),
            3,
        )
        .unwrap()
        .collect();
        assert_eq!(
            dates,
            vec![stamp(4, 9, 30), stamp(4, 9, 0), stamp(1, 10, 30)]
        );
    }

    #[test]
    fn validates_horizon_and_respects_zero_limit() {
        let cron = crate::parse_cron_schedule("* * * * *").unwrap();
        let start = stamp(1, 9, 0);
        assert_eq!(
            each_previous_cron_minute_between(&cron, start, start, 10)
                .unwrap()
                .next(),
            None
        );
        assert_eq!(
            each_previous_cron_minute_between(&cron, start, stamp(1, 9, 1), 0)
                .unwrap()
                .next(),
            None
        );
        assert!(each_previous_cron_minute_between(&cron, stamp(2, 9, 0), start, 10).is_none());
    }
}
