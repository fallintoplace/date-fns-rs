//! Configurable working weekdays with constant-time week arithmetic.

use chrono::{Datelike, Days, NaiveDate, Weekday};

/// A nonempty set of working weekdays, with Monday in bit zero.
///
/// Supports non-Western workweeks and seven-day schedules without assuming
/// Saturday and Sunday are always days off. There is no time-zone dependency.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Workweek {
    mask: u8,
}

impl Workweek {
    /// Construct a calendar from one or more weekdays. Duplicates are ignored.
    ///
    /// A calendar with no working days is rejected.
    pub fn new(days: &[Weekday]) -> Option<Self> {
        let mask = days.iter().fold(0_u8, |mask, day| {
            mask | (1 << day.num_days_from_monday())
        });
        Self::from_mask(mask)
    }

    /// Construct from a Monday-first seven-bit mask.
    ///
    /// Bit 0 represents Monday and bit 6 represents Sunday. Unused high
    /// bits and empty calendars are rejected.
    pub fn from_mask(mask: u8) -> Option<Self> {
        (mask != 0 && mask & 0x80 == 0).then_some(Self { mask })
    }

    /// Return the seven-bit Monday-first working-day mask.
    pub fn mask(self) -> u8 {
        self.mask
    }

    /// Number of working days in a complete calendar week.
    pub fn workdays_per_week(self) -> u32 {
        self.mask.count_ones()
    }

    /// Test whether the date falls on a configured working weekday.
    pub fn is_workday(self, date: NaiveDate) -> bool {
        let bit = date.weekday().num_days_from_monday();
        self.mask & (1_u8 << bit) != 0
    }

    /// Count working days in an inclusive date interval in O(1).
    ///
    /// Full weeks use arithmetic; at most six remainder days are inspected.
    /// Reversed intervals return None. The entire Chrono range is supported.
    pub fn count_workdays(self, start: NaiveDate, end: NaiveDate) -> Option<i64> {
        if start > end {
            return None;
        }
        let days = end.signed_duration_since(start).num_days() + 1;
        let mut total = (days / 7) * i64::from(self.workdays_per_week());
        let first = start.weekday().num_days_from_monday();
        for step in 0..(days % 7) as u32 {
            let weekday = (first + step) % 7;
            if self.mask & (1_u8 << weekday) != 0 {
                total += 1;
            }
        }
        Some(total)
    }

    /// Move by signed workdays, excluding the starting date.
    ///
    /// Zero preserves the starting date even when it is not a workday.
    /// Whole workweeks are skipped in O(1), followed by at most seven
    /// calendar-day steps. Returns None if the result is out of range.
    pub fn add_workdays(self, date: NaiveDate, offset: i64) -> Option<NaiveDate> {
        if offset == 0 {
            return Some(date);
        }
        let per_week = u64::from(self.workdays_per_week());
        let magnitude = offset.unsigned_abs();
        let weeks = (magnitude - 1) / per_week;
        let skipped = weeks.checked_mul(7)?;
        let mut current = if offset > 0 {
            date.checked_add_days(Days::new(skipped))?
        } else {
            date.checked_sub_days(Days::new(skipped))?
        };
        let mut remaining = magnitude - weeks * per_week;
        while remaining != 0 {
            current = if offset > 0 {
                current.succ_opt()?
            } else {
                current.pred_opt()?
            };
            if self.is_workday(current) {
                remaining -= 1;
            }
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn slow_advance(workweek: Workweek, start: NaiveDate, count: i64) -> NaiveDate {
        let mut current = start;
        let mut left = count.unsigned_abs();
        while left != 0 {
            current = if count > 0 {
                current.succ_opt().unwrap()
            } else {
                current.pred_opt().unwrap()
            };
            if workweek.is_workday(current) {
                left -= 1;
            }
        }
        current
    }

    #[test]
    fn supports_friday_saturday_weekends_and_full_week_calendars() {
        let sun_thu = Workweek::new(&[
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
        ])
        .unwrap();
        assert_eq!(sun_thu.workdays_per_week(), 5);
        assert!(!sun_thu.is_workday(date(2024, 3, 1)));
        assert!(!sun_thu.is_workday(date(2024, 3, 2)));
        assert!(sun_thu.is_workday(date(2024, 3, 3)));
        assert_eq!(
            sun_thu.add_workdays(date(2024, 2, 29), 1),
            Some(date(2024, 3, 3))
        );
        assert_eq!(
            sun_thu.add_workdays(date(2024, 3, 3), -1),
            Some(date(2024, 2, 29))
        );
        let every_day = Workweek::from_mask(0x7f).unwrap();
        assert_eq!(every_day.count_workdays(date(2024, 2, 28), date(2024, 3, 2)), Some(4));
        assert_eq!(
            every_day.add_workdays(date(2024, 2, 28), 3),
            Some(date(2024, 3, 2))
        );
    }

    #[test]
    fn rejects_empty_and_out_of_range_masks_and_offsets() {
        assert_eq!(Workweek::new(&[]), None);
        assert_eq!(Workweek::from_mask(0), None);
        assert_eq!(Workweek::from_mask(0x80), None);
        let mon = Workweek::new(&[Weekday::Mon, Weekday::Mon]).unwrap();
        assert_eq!(mon.workdays_per_week(), 1);
        assert_eq!(mon.add_workdays(NaiveDate::MAX, 1), None);
        assert_eq!(mon.add_workdays(NaiveDate::MIN, -1), None);
        assert_eq!(mon.add_workdays(date(2024, 3, 1), i64::MIN), None);
        assert_eq!(mon.add_workdays(date(2024, 3, 1), 0), Some(date(2024, 3, 1)));
        assert_eq!(mon.count_workdays(date(2024, 3, 2), date(2024, 3, 1)), None);
    }

    #[test]
    fn exhaustive_masks_and_signed_offsets_match_day_by_day_reference() {
        let start = date(2024, 2, 27);
        for mask in 1..=127_u8 {
            let workweek = Workweek::from_mask(mask).unwrap();
            for origin_offset in 0..7 {
                let origin = start.checked_add_days(Days::new(origin_offset)).unwrap();
                for offset in -30..=30 {
                    let expected = slow_advance(workweek, origin, offset);
                    assert_eq!(
                        workweek.add_workdays(origin, offset),
                        Some(expected),
                        "mask={mask}, origin={origin}, offset={offset}"
                    );
                }
            }
            for length in 0..=20_u64 {
                let end = start.checked_add_days(Days::new(length)).unwrap();
                let expected = (0..=length)
                    .filter(|step| {
                        workweek.is_workday(start.checked_add_days(Days::new(*step)).unwrap())
                    })
                    .count() as i64;
                assert_eq!(workweek.count_workdays(start, end), Some(expected));
            }
        }
    }

    #[test]
    fn handles_chrono_extremes_without_iteration() {
        let all_days = Workweek::from_mask(0x7f).unwrap();
        let full_count = NaiveDate::MAX
            .signed_duration_since(NaiveDate::MIN)
            .num_days()
            + 1;
        assert_eq!(
            all_days.count_workdays(NaiveDate::MIN, NaiveDate::MAX),
            Some(full_count)
        );
    }
}
