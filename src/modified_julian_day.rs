//! Modified Julian Day numbers for Gregorian civil dates.
//!
//! These are integer midnight-based day numbers, not astronomical noon ticks.

use chrono::NaiveDate;

const JULIAN_DAY_OFFSET: i64 = 2_400_001;

/// Convert a Gregorian date to its integer Modified Julian Day.
/// The epoch 1858-11-17 is day zero.
pub fn to_modified_julian_day(date: NaiveDate) -> i64 {
    crate::to_julian_day(date) - JULIAN_DAY_OFFSET
}

/// Convert an integer Modified Julian Day back to a Gregorian date.
///
/// Returns None for values outside Chrono's supported range.
pub fn from_modified_julian_day(day: i64) -> Option<NaiveDate> {
    crate::from_julian_day(day.checked_add(JULIAN_DAY_OFFSET)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn matches_standard_modified_julian_epoch_values() {
        assert_eq!(to_modified_julian_day(date(1858, 11, 17)), 0);
        assert_eq!(to_modified_julian_day(date(1970, 1, 1)), 40_587);
        assert_eq!(to_modified_julian_day(date(2000, 1, 1)), 51_544);
        assert_eq!(from_modified_julian_day(0), Some(date(1858, 11, 17)));
    }

    #[test]
    fn round_trips_leap_years_and_chrono_boundaries() {
        for value in [
            date(2024, 2, 29),
            date(1900, 3, 1),
            date(0, 1, 1),
            date(-44, 3, 15),
            NaiveDate::MIN,
            NaiveDate::MAX,
        ] {
            assert_eq!(from_modified_julian_day(to_modified_julian_day(value)), Some(value));
        }
    }

    #[test]
    fn rejects_unrepresentable_day_numbers() {
        assert_eq!(from_modified_julian_day(i64::MAX), None);
        assert_eq!(from_modified_julian_day(i64::MIN), None);
        assert_eq!(from_modified_julian_day(to_modified_julian_day(NaiveDate::MIN) - 1), None);
        assert_eq!(from_modified_julian_day(to_modified_julian_day(NaiveDate::MAX) + 1), None);
    }
}
