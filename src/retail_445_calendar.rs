//! Explicit 4-4-5 retail calendars with optional 53-week years.
//!
//! Each retail year starts on a caller-specified date. There is no implicit
//! locale, fiscal anchor, or rule for choosing a leap-week year.

use chrono::{Days, NaiveDate};

/// Inclusive cumulative week boundaries for twelve 4-4-5 retail periods.
const PERIOD_END_WEEKS: [u8; 13] = [0, 4, 8, 13, 17, 21, 26, 30, 34, 39, 43, 47, 52];

/// A retail calendar anchored to an explicit first day of the year.
///
/// The year contains 52 or 53 complete seven-day weeks. In a 53-week year
/// the extra week belongs to the twelfth retail period and fourth quarter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Retail445Calendar {
    year_start: NaiveDate,
    year_end: NaiveDate,
    weeks: u8,
}

/// The location of a date inside a 4-4-5 retail year.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Retail445Position {
    /// Retail quarter, numbered 1 through 4.
    pub quarter: u8,
    /// Retail period, numbered 1 through 12.
    pub period: u8,
    /// Week of retail year, numbered 1 through 52 or 53.
    pub week_of_year: u8,
    /// Week within this period, numbered 1 through 4, 5, or 6.
    pub week_of_period: u8,
    /// Day within this period, numbered 1 through 28, 35, or 42.
    pub day_of_period: u8,
}

impl Retail445Calendar {
    /// Construct a 52- or 53-week year beginning on the supplied date.
    ///
    /// Returns None if the week count is invalid or the inclusive end of
    /// the year would exceed Chrono's supported calendar range.
    pub fn new(year_start: NaiveDate, weeks: u8) -> Option<Self> {
        if weeks != 52 && weeks != 53 {
            return None;
        }
        let length = u64::from(weeks) * 7;
        let year_end = year_start.checked_add_days(Days::new(length - 1))?;
        Some(Self {
            year_start,
            year_end,
            weeks,
        })
    }

    /// The inclusive first date of the retail year.
    pub fn year_start(self) -> NaiveDate {
        self.year_start
    }

    /// The inclusive last date of the retail year.
    pub fn year_end(self) -> NaiveDate {
        self.year_end
    }

    /// Number of weeks in the retail year, either 52 or 53.
    pub fn weeks_in_year(self) -> u8 {
        self.weeks
    }

    /// Inclusive first and last dates for a retail period 1 through 12.
    pub fn period_bounds(self, period: u8) -> Option<(NaiveDate, NaiveDate)> {
        if !(1..=12).contains(&period) {
            return None;
        }
        let index = usize::from(period);
        let begin_week = u64::from(PERIOD_END_WEEKS[index - 1]);
        let end_week = if period == 12 {
            u64::from(self.weeks)
        } else {
            u64::from(PERIOD_END_WEEKS[index])
        };
        let start = self.year_start.checked_add_days(Days::new(begin_week * 7))?;
        let end = self
            .year_start
            .checked_add_days(Days::new(end_week * 7 - 1))?;
        Some((start, end))
    }

    /// Inclusive first and last dates for retail quarter 1 through 4.
    ///
    /// Each quarter contains 13 weeks, except the fourth quarter of a
    /// 53-week retail year, which contains 14.
    pub fn quarter_bounds(self, quarter: u8) -> Option<(NaiveDate, NaiveDate)> {
        if !(1..=4).contains(&quarter) {
            return None;
        }
        let first_week = u64::from(quarter - 1) * 13;
        let final_week = if quarter == 4 {
            u64::from(self.weeks)
        } else {
            u64::from(quarter) * 13
        };
        let start = self.year_start.checked_add_days(Days::new(first_week * 7))?;
        let end = self
            .year_start
            .checked_add_days(Days::new(final_week * 7 - 1))?;
        Some((start, end))
    }

    /// Locate a date inside its retail period, quarter, and week.
    ///
    /// Returns None for dates outside this calendar year.
    pub fn position(self, date: NaiveDate) -> Option<Retail445Position> {
        if date < self.year_start || date > self.year_end {
            return None;
        }
        let day_offset = date.signed_duration_since(self.year_start).num_days();
        let week_index = day_offset / 7;
        for period in 1..=12_u8 {
            let last_week = if period == 12 {
                self.weeks
            } else {
                PERIOD_END_WEEKS[usize::from(period)]
            };
            if week_index < i64::from(last_week) {
                let first_week = i64::from(PERIOD_END_WEEKS[usize::from(period) - 1]);
                let weeks_into_period = week_index - first_week;
                return Some(Retail445Position {
                    quarter: (period - 1) / 3 + 1,
                    period,
                    week_of_year: u8::try_from(week_index + 1).ok()?,
                    week_of_period: u8::try_from(weeks_into_period + 1).ok()?,
                    day_of_period: u8::try_from(
                        weeks_into_period * 7 + (day_offset % 7) + 1,
                    )
                    .ok()?,
                });
            }
        }
        None
    }

    /// Convert a 1-based retail week and 1-based weekday to a date.
    ///
    /// Weekday 1 means the retail year's chosen starting weekday, not
    /// necessarily Monday. Week 53 is accepted only in a 53-week year.
    pub fn date_for_week_day(self, week: u8, weekday: u8) -> Option<NaiveDate> {
        if week == 0 || week > self.weeks || !(1..=7).contains(&weekday) {
            return None;
        }
        let offset = u64::from(week - 1) * 7 + u64::from(weekday - 1);
        self.year_start.checked_add_days(Days::new(offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn follows_four_four_five_pattern_across_leap_february() {
        let calendar = Retail445Calendar::new(date(2024, 2, 4), 52).unwrap();
        assert_eq!(calendar.weeks_in_year(), 52);
        assert_eq!(calendar.year_end(), date(2025, 2, 1));
        assert_eq!(
            calendar.period_bounds(1),
            Some((date(2024, 2, 4), date(2024, 3, 2)))
        );
        assert_eq!(
            calendar.period_bounds(2),
            Some((date(2024, 3, 3), date(2024, 3, 30)))
        );
        assert_eq!(
            calendar.period_bounds(3),
            Some((date(2024, 3, 31), date(2024, 5, 4)))
        );
        assert_eq!(
            calendar.quarter_bounds(1),
            Some((date(2024, 2, 4), date(2024, 5, 4)))
        );
        assert_eq!(calendar.period_bounds(0), None);
        assert_eq!(calendar.period_bounds(13), None);
        assert_eq!(calendar.quarter_bounds(0), None);
        assert_eq!(calendar.quarter_bounds(5), None);
    }

    #[test]
    fn assigns_53rd_week_only_to_final_period_and_quarter() {
        let base = date(2024, 2, 4);
        let regular = Retail445Calendar::new(base, 52).unwrap();
        let leap_week = Retail445Calendar::new(base, 53).unwrap();
        assert_eq!(leap_week.year_end(), date(2025, 2, 8));
        assert_eq!(leap_week.period_bounds(11), regular.period_bounds(11));
        assert_eq!(leap_week.period_bounds(12).unwrap().0, regular.period_bounds(12).unwrap().0);
        let (last_start, last_end) = leap_week.period_bounds(12).unwrap();
        assert_eq!(last_end.signed_duration_since(last_start).num_days() + 1, 42);
        let (quarter_start, quarter_end) = leap_week.quarter_bounds(4).unwrap();
        assert_eq!(
            quarter_end.signed_duration_since(quarter_start).num_days() + 1,
            98
        );
        let last = leap_week.position(leap_week.year_end()).unwrap();
        assert_eq!(
            last,
            Retail445Position {
                quarter: 4,
                period: 12,
                week_of_year: 53,
                week_of_period: 6,
                day_of_period: 42,
            }
        );
        assert_eq!(regular.date_for_week_day(53, 1), None);
        assert_eq!(leap_week.date_for_week_day(53, 7), Some(leap_week.year_end()));
    }

    #[test]
    fn exhaustive_dates_round_trip_through_week_and_period_positions() {
        let start = date(2024, 2, 4);
        for weeks in [52, 53] {
            let calendar = Retail445Calendar::new(start, weeks).unwrap();
            for day in 0..u64::from(weeks) * 7 {
                let current = start.checked_add_days(Days::new(day)).unwrap();
                let position = calendar.position(current).unwrap();
                assert_eq!(position.week_of_year, (day / 7 + 1) as u8);
                assert_eq!(
                    calendar.date_for_week_day(position.week_of_year, (day % 7 + 1) as u8),
                    Some(current)
                );
                let (period_start, period_end) =
                    calendar.period_bounds(position.period).unwrap();
                assert!(period_start <= current && current <= period_end);
                assert_eq!(
                    position.day_of_period,
                    (current.signed_duration_since(period_start).num_days() + 1) as u8
                );
                let (quarter_start, quarter_end) =
                    calendar.quarter_bounds(position.quarter).unwrap();
                assert!(quarter_start <= current && current <= quarter_end);
            }
        }
    }

    #[test]
    fn validates_week_count_weekdays_and_chrono_limits() {
        assert!(Retail445Calendar::new(date(2024, 2, 4), 51).is_none());
        assert!(Retail445Calendar::new(date(2024, 2, 4), 54).is_none());
        assert!(Retail445Calendar::new(NaiveDate::MAX, 52).is_none());
        assert!(Retail445Calendar::new(NaiveDate::MIN, 53).is_some());
        let calendar = Retail445Calendar::new(date(2024, 2, 4), 52).unwrap();
        assert_eq!(calendar.position(date(2024, 2, 3)), None);
        assert_eq!(calendar.position(date(2025, 2, 2)), None);
        assert_eq!(calendar.date_for_week_day(0, 1), None);
        assert_eq!(calendar.date_for_week_day(1, 0), None);
        assert_eq!(calendar.date_for_week_day(1, 8), None);
    }
}
