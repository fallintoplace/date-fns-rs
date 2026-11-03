//! Calendar decade and century boundary differences for naive datetimes.

use chrono::NaiveDateTime;

/// Count signed Gregorian decade boundaries, ignoring month, day and time.
///
/// Standard decades span years 2020-2029, 2030-2039, and so on.
pub fn difference_in_calendar_decades_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> i64 {
    crate::difference_in_calendar_decades(left.date(), right.date())
}

/// Count signed one-based century boundaries, ignoring clock precision.
///
/// The 21st century begins January 1, 2001, not January 1, 2000.
pub fn difference_in_calendar_centuries_datetime(
    left: NaiveDateTime,
    right: NaiveDateTime,
) -> i32 {
    crate::difference_in_calendar_centuries(left.date(), right.date())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, h: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, 30, 0)
            .unwrap()
    }

    #[test]
    fn counts_period_boundaries_not_elapsed_time() {
        let late = at(2019, 12, 31, 23);
        let early = at(2020, 1, 1, 0);
        assert_eq!(difference_in_calendar_decades_datetime(early, late), 1);
        assert_eq!(difference_in_calendar_decades_datetime(late, early), -1);
        assert_eq!(difference_in_calendar_decades_datetime(early, early), 0);
        assert_eq!(
            difference_in_calendar_centuries_datetime(at(2001, 1, 1, 0), at(2000, 12, 31, 23)),
            1
        );
        assert_eq!(
            difference_in_calendar_centuries_datetime(at(2100, 12, 31, 23), at(2001, 1, 1, 0)),
            0
        );
    }

    #[test]
    fn handles_negative_years_and_chrono_limits() {
        assert_eq!(
            difference_in_calendar_decades_datetime(at(0, 1, 1, 0), at(-1, 12, 31, 23)),
            1
        );
        assert_eq!(
            difference_in_calendar_centuries_datetime(at(1, 1, 1, 0), at(0, 12, 31, 23)),
            1
        );
        let count =
            difference_in_calendar_centuries_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(count > 0);
        assert_eq!(
            difference_in_calendar_centuries_datetime(NaiveDateTime::MIN, NaiveDateTime::MAX),
            -count
        );
    }
}
