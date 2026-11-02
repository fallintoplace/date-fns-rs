//! Configurable 4-4-5, 4-5-4 and 5-4-4 retail accounting calendars.

use chrono::{Days, NaiveDate};

use crate::Retail445Calendar;

/// Week distribution inside each of the four retail quarters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetailQuarterPattern {
    FourFourFive,
    FourFiveFour,
    FiveFourFour,
}

impl RetailQuarterPattern {
    fn period_weeks(self) -> [u8; 3] {
        match self {
            Self::FourFourFive => [4, 4, 5],
            Self::FourFiveFour => [4, 5, 4],
            Self::FiveFourFour => [5, 4, 4],
        }
    }
}

/// A date's position in a selected retail calendar pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetailPeriodPosition {
    pub quarter: u8,
    pub period: u8,
    pub week_of_year: u8,
    pub week_of_period: u8,
    pub day_of_period: u8,
}

/// An explicit 52- or 53-week fiscal year with a retail quarter pattern.
///
/// The year start is caller-defined, and a leap week always extends period
/// twelve. No implicit fiscal-year or week-start conventions are applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetailCalendar {
    base: Retail445Calendar,
    pattern: RetailQuarterPattern,
}

impl RetailCalendar {
    pub fn new(year_start: NaiveDate, weeks: u8, pattern: RetailQuarterPattern) -> Option<Self> {
        Some(Self {
            base: Retail445Calendar::new(year_start, weeks)?,
            pattern,
        })
    }

    pub fn year_start(self) -> NaiveDate {
        self.base.year_start()
    }

    pub fn year_end(self) -> NaiveDate {
        self.base.year_end()
    }

    pub fn weeks_in_year(self) -> u8 {
        self.base.weeks_in_year()
    }

    pub fn pattern(self) -> RetailQuarterPattern {
        self.pattern
    }

    /// Return inclusive bounds of one retail period, numbered 1 to 12.
    pub fn period_bounds(self, period: u8) -> Option<(NaiveDate, NaiveDate)> {
        if !(1..=12).contains(&period) {
            return None;
        }
        let quarter = u64::from((period - 1) / 3);
        let slot = usize::from((period - 1) % 3);
        let weeks = self.pattern.period_weeks();
        let within_quarter: u64 = weeks[..slot].iter().copied().map(u64::from).sum();
        let first_week = quarter * 13 + within_quarter;
        let extra = u64::from(period == 12 && self.base.weeks_in_year() == 53);
        let length = u64::from(weeks[slot]) + extra;
        let first = self
            .base
            .year_start()
            .checked_add_days(Days::new(first_week * 7))?;
        let last = first.checked_add_days(Days::new(length * 7 - 1))?;
        Some((first, last))
    }

    /// Return inclusive bounds of a retail quarter, numbered 1 to 4.
    pub fn quarter_bounds(self, quarter: u8) -> Option<(NaiveDate, NaiveDate)> {
        self.base.quarter_bounds(quarter)
    }

    /// Locate a date within the selected retail period pattern.
    pub fn position(self, date: NaiveDate) -> Option<RetailPeriodPosition> {
        if date < self.year_start() || date > self.year_end() {
            return None;
        }
        for period in 1..=12 {
            let (first, last) = self.period_bounds(period)?;
            if date <= last {
                let offset = date.signed_duration_since(first).num_days();
                let day_of_year = date.signed_duration_since(self.year_start()).num_days();
                return Some(RetailPeriodPosition {
                    quarter: (period - 1) / 3 + 1,
                    period,
                    week_of_year: u8::try_from(day_of_year / 7 + 1).ok()?,
                    week_of_period: u8::try_from(offset / 7 + 1).ok()?,
                    day_of_period: u8::try_from(offset + 1).ok()?,
                });
            }
        }
        None
    }

    /// Convert retail-year week and weekday numbers (both 1-based) to a date.
    pub fn date_for_week_day(self, week: u8, weekday: u8) -> Option<NaiveDate> {
        self.base.date_for_week_day(week, weekday)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn supports_each_quarter_pattern_without_changing_year_bounds() {
        let start = date(2024, 2, 4);
        let four_four_five =
            RetailCalendar::new(start, 52, RetailQuarterPattern::FourFourFive).unwrap();
        let four_five_four =
            RetailCalendar::new(start, 52, RetailQuarterPattern::FourFiveFour).unwrap();
        let five_four_four =
            RetailCalendar::new(start, 52, RetailQuarterPattern::FiveFourFour).unwrap();

        assert_eq!(
            four_four_five.period_bounds(2),
            Some((date(2024, 3, 3), date(2024, 3, 30)))
        );
        assert_eq!(
            four_five_four.period_bounds(2),
            Some((date(2024, 3, 3), date(2024, 4, 6)))
        );
        assert_eq!(
            five_four_four.period_bounds(2),
            Some((date(2024, 3, 10), date(2024, 4, 6)))
        );
        for calendar in [four_four_five, four_five_four, five_four_four] {
            assert_eq!(calendar.year_end(), date(2025, 2, 1));
            assert_eq!(
                calendar.quarter_bounds(4).unwrap().1,
                calendar.period_bounds(12).unwrap().1
            );
            assert_eq!(calendar.period_bounds(0), None);
            assert_eq!(calendar.period_bounds(13), None);
        }
    }

    #[test]
    fn places_extra_week_in_last_period_and_locates_dates() {
        for pattern in [
            RetailQuarterPattern::FourFourFive,
            RetailQuarterPattern::FourFiveFour,
            RetailQuarterPattern::FiveFourFour,
        ] {
            let calendar = RetailCalendar::new(date(2024, 2, 4), 53, pattern).unwrap();
            assert_eq!(calendar.weeks_in_year(), 53);
            let last = calendar.year_end();
            assert_eq!(calendar.period_bounds(12).unwrap().1, last);
            let position = calendar.position(last).unwrap();
            assert_eq!(position.quarter, 4);
            assert_eq!(position.period, 12);
            assert_eq!(position.week_of_year, 53);
            assert_eq!(calendar.date_for_week_day(53, 7), Some(last));
        }
    }

    #[test]
    fn rejects_invalid_year_lengths_and_outside_dates() {
        assert_eq!(
            RetailCalendar::new(date(2024, 2, 4), 51, RetailQuarterPattern::FourFiveFour),
            None
        );
        let calendar =
            RetailCalendar::new(date(2024, 2, 4), 52, RetailQuarterPattern::FiveFourFour).unwrap();
        assert_eq!(calendar.position(date(2024, 2, 3)), None);
        assert_eq!(calendar.position(date(2025, 2, 2)), None);
    }
}
