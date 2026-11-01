//! Lazy fixed-step daily iteration over inclusive naive datetime ranges.

use std::iter::FusedIterator;

use chrono::{Days, NaiveDateTime};

/// A daily iterator that retains the original time and nanosecond precision.
#[derive(Clone, Debug)]
pub struct DaysInDatetimeInterval {
    next: Option<NaiveDateTime>,
    end: NaiveDateTime,
    step_days: u64,
}

impl Iterator for DaysInDatetimeInterval {
    type Item = NaiveDateTime;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(next) = self.next else {
            return (0, Some(0));
        };
        let whole_days = self.end.signed_duration_since(next).num_days() as u64;
        let count = whole_days / self.step_days + 1;
        match usize::try_from(count) {
            Ok(size) => (size, Some(size)),
            Err(_) => (0, None),
        }
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current
            .date()
            .checked_add_days(Days::new(self.step_days))
            .map(|date| date.and_time(current.time()))
            .filter(|next| *next <= self.end);
        Some(current)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.next = None;
            return None;
        }

        let current = self.next?;
        let offset = match u64::try_from(n)
            .ok()
            .and_then(|n| n.checked_mul(self.step_days))
        {
            Some(offset) => offset,
            None => {
                self.next = None;
                return None;
            }
        };
        self.next = current
            .date()
            .checked_add_days(Days::new(offset))
            .map(|date| date.and_time(current.time()))
            .filter(|next| *next <= self.end);
        self.next()
    }
}

impl ExactSizeIterator for DaysInDatetimeInterval {}

impl FusedIterator for DaysInDatetimeInterval {}

/// Visit every nth calendar day at the exact starting clock time.
/// Both endpoints are inclusive. Returns None for a reversed range or zero step.
pub fn each_nth_day_of_datetime_interval(
    start: NaiveDateTime,
    end: NaiveDateTime,
    step_days: u64,
) -> Option<DaysInDatetimeInterval> {
    (start <= end && step_days > 0).then_some(DaysInDatetimeInterval {
        next: Some(start),
        end,
        step_days,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn dt(day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2024, 2, day)
            .unwrap()
            .and_hms_nano_opt(23, 30, 0, 123_456_789)
            .unwrap()
    }

    #[test]
    fn iterates_across_leap_day_and_preserves_nanoseconds() {
        let start = dt(27);
        let last = NaiveDate::from_ymd_opt(2024, 3, 2)
            .unwrap()
            .and_time(start.time());
        let values: Vec<_> = each_nth_day_of_datetime_interval(start, last, 2)
            .unwrap()
            .collect();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], start);
        assert_eq!(values[1], dt(29));
        assert_eq!(values[2], last);
    }

    #[test]
    fn supports_constant_time_skipping_and_exact_sizes() {
        let start = dt(27);
        let last = NaiveDate::from_ymd_opt(2024, 3, 4)
            .unwrap()
            .and_time(start.time());
        let mut days = each_nth_day_of_datetime_interval(start, last, 2).unwrap();
        assert_eq!(days.len(), 4);
        let skipped = NaiveDate::from_ymd_opt(2024, 3, 2)
            .unwrap()
            .and_time(start.time());
        assert_eq!(days.nth(2), Some(skipped));
        assert_eq!(days.len(), 1);
        assert_eq!(days.next(), Some(last));
        assert_eq!(days.next(), None);
    }

    #[test]
    fn validates_endpoints_and_avoids_range_overflow() {
        assert!(each_nth_day_of_datetime_interval(dt(28), dt(27), 1).is_none());
        assert!(each_nth_day_of_datetime_interval(dt(27), dt(28), 0).is_none());
        let mut single =
            each_nth_day_of_datetime_interval(NaiveDateTime::MAX, NaiveDateTime::MAX, 1).unwrap();
        assert_eq!(single.next(), Some(NaiveDateTime::MAX));
        assert_eq!(single.next(), None);
        let mut enormous =
            each_nth_day_of_datetime_interval(NaiveDateTime::MIN, NaiveDateTime::MAX, 1).unwrap();
        assert_eq!(enormous.nth(usize::MAX), None);
        assert_eq!(enormous.next(), None);
    }
}
