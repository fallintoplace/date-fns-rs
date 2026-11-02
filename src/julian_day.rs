//! Integer Julian day numbers for proleptic Gregorian civil dates.
//! A Julian day number refers here to the civil day starting at midnight.

use chrono::{Datelike, NaiveDate};

const CE_TO_JULIAN: i64 = 1_721_425;

/// Convert a Gregorian civil date to its integer Julian day number.
///
/// The Unix epoch 1970-01-01 maps to JDN 2440588.
pub fn to_julian_day(date: NaiveDate) -> i64 {
    i64::from(date.num_days_from_ce()) + CE_TO_JULIAN
}

/// Convert an integer Julian day number to a Gregorian civil date.
///
/// Returns None outside Chrono's supported date range.
pub fn from_julian_day(day: i64) -> Option<NaiveDate> {
    let ce = i32::try_from(day.checked_sub(CE_TO_JULIAN)?).ok()?;
    NaiveDate::from_num_days_from_ce_opt(ce)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_standard_julian_day_numbers() {
        assert_eq!(to_julian_day(date(1970, 1, 1)), 2_440_588);
        assert_eq!(to_julian_day(date(2000, 1, 1)), 2_451_545);
        assert_eq!(to_julian_day(date(1, 1, 1)), 1_721_426);
        assert_eq!(from_julian_day(2_440_588), Some(date(1970, 1, 1)));
    }

    #[test]
    fn round_trips_leap_dates_negative_years_and_limits() {
        for day in [
            date(2024, 2, 29),
            date(1900, 3, 1),
            date(0, 1, 1),
            date(-44, 3, 15),
            NaiveDate::MIN,
            NaiveDate::MAX,
        ] {
            assert_eq!(from_julian_day(to_julian_day(day)), Some(day));
        }
    }

    #[test]
    fn rejects_numbers_outside_the_date_domain() {
        assert_eq!(from_julian_day(i64::MAX), None);
        assert_eq!(from_julian_day(i64::MIN), None);
        assert_eq!(from_julian_day(to_julian_day(NaiveDate::MIN) - 1), None);
        assert_eq!(from_julian_day(to_julian_day(NaiveDate::MAX) + 1), None);
    }
}
