//! Monthly BYSETPOS-like recurrence over a set of weekdays.

use std::collections::VecDeque;
use std::iter::FusedIterator;

use chrono::{Datelike, Months, NaiveDate};

/// Lazily select ordinal positions from matching weekdays in each month.
///
/// `weekday_mask` uses bit 0 for Monday through bit 6 for Sunday. Positions
/// are 1-based: 1 means the first matching day and -1 means the last.
/// Up to 31 positions may be selected, and duplicates are emitted once.
#[derive(Clone, Debug)]
pub struct MonthlyWeekdaySetPosition {
    next_month: Option<NaiveDate>,
    start: NaiveDate,
    end: NaiveDate,
    weekday_mask: u8,
    positions: Vec<i8>,
    step_months: u32,
    ready: VecDeque<NaiveDate>,
}

impl Iterator for MonthlyWeekdaySetPosition {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(date) = self.ready.pop_front() {
                return Some(date);
            }
            let month = self.next_month?;
            if month > self.end {
                self.next_month = None;
                return None;
            }
            self.next_month = month
                .checked_add_months(Months::new(self.step_months))
                .filter(|next| *next <= self.end);

            let mut candidates = Vec::new();
            let mut date = month;
            while date.month() == month.month() && date.year() == month.year() {
                let bit = date.weekday().num_days_from_monday();
                if self.weekday_mask & (1 << bit) != 0 {
                    candidates.push(date);
                }
                let Some(next) = date.succ_opt() else {
                    break;
                };
                date = next;
            }

            let count = candidates.len() as isize;
            let mut selected = Vec::new();
            for &position in &self.positions {
                let index = if position > 0 {
                    isize::from(position) - 1
                } else {
                    count + isize::from(position)
                };
                if index >= 0 && index < count {
                    let value = candidates[index as usize];
                    if value >= self.start && value <= self.end {
                        selected.push(value);
                    }
                }
            }
            selected.sort_unstable();
            selected.dedup();
            self.ready.extend(selected);
        }
    }
}

impl FusedIterator for MonthlyWeekdaySetPosition {}

/// Iterate selected weekday positions every `step_months` calendar months.
///
/// The first evaluated month contains `start`. Missing positions are skipped.
/// Zero steps, reversed endpoints, invalid masks and zero or oversized
/// positions return None.
pub fn each_monthly_weekday_set_position(
    start: NaiveDate,
    end: NaiveDate,
    weekday_mask: u8,
    positions: &[i8],
    step_months: u32,
) -> Option<MonthlyWeekdaySetPosition> {
    if start > end
        || step_months == 0
        || weekday_mask == 0
        || weekday_mask & !0x7f != 0
        || positions.is_empty()
        || positions.iter().any(|position| *position == 0 || position.unsigned_abs() > 31)
    {
        return None;
    }
    Some(MonthlyWeekdaySetPosition {
        next_month: Some(NaiveDate::from_ymd_opt(start.year(), start.month(), 1)?),
        start,
        end,
        weekday_mask,
        positions: positions.to_vec(),
        step_months,
        ready: VecDeque::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    const MON_WED_FRI: u8 = 0b0010101;

    #[test]
    fn selects_first_and_last_matching_weekday_per_month() {
        let dates: Vec<_> = each_monthly_weekday_set_position(
            date(2024, 1, 1),
            date(2024, 3, 31),
            MON_WED_FRI,
            &[-1, 1, 1],
            1,
        )
        .unwrap()
        .collect();
        assert_eq!(
            dates,
            vec![
                date(2024, 1, 1),
                date(2024, 1, 31),
                date(2024, 2, 2),
                date(2024, 2, 28),
                date(2024, 3, 1),
                date(2024, 3, 29),
            ]
        );
    }

    #[test]
    fn supports_month_skips_and_partial_start_windows() {
        let dates: Vec<_> = each_monthly_weekday_set_position(
            date(2024, 1, 15),
            date(2024, 5, 31),
            MON_WED_FRI,
            &[1, -1],
            2,
        )
        .unwrap()
        .collect();
        assert_eq!(
            dates,
            vec![
                date(2024, 1, 31),
                date(2024, 3, 1),
                date(2024, 3, 29),
                date(2024, 5, 1),
                date(2024, 5, 31),
            ]
        );
    }

    #[test]
    fn rejects_invalid_recurrence_configuration() {
        let start = date(2024, 1, 1);
        let end = date(2024, 2, 1);
        for (mask, positions, step) in [
            (0, vec![1], 1),
            (128, vec![1], 1),
            (1, vec![], 1),
            (1, vec![0], 1),
            (1, vec![-32], 1),
            (1, vec![1], 0),
        ] {
            assert!(
                each_monthly_weekday_set_position(start, end, mask, &positions, step).is_none()
            );
        }
        assert!(
            each_monthly_weekday_set_position(end, start, 1, &[1], 1).is_none()
        );
    }

    #[test]
    fn is_fused_at_chrono_limits_and_skips_missing_positions() {
        let mut empty = each_monthly_weekday_set_position(
            date(2024, 2, 1),
            date(2024, 2, 29),
            1,
            &[5],
            1,
        )
        .unwrap();
        assert_eq!(empty.next(), None);
        assert_eq!(empty.next(), None);
        let mut edge = each_monthly_weekday_set_position(
            NaiveDate::MAX,
            NaiveDate::MAX,
            1 << NaiveDate::MAX.weekday().num_days_from_monday(),
            &[-1],
            1,
        )
        .unwrap();
        assert_eq!(edge.next(), Some(NaiveDate::MAX));
        assert_eq!(edge.next(), None);
    }
}
