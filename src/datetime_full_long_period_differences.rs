//! Complete decade and century differences between naive datetimes.

use chrono::NaiveDateTime;

/// Count complete ten-year periods from right to left.
///
/// Partial decades truncate toward zero. Clock time and leap-day clamping
/// follow difference_in_years_datetime rather than calendar boundaries.
pub fn difference_in_decades_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    crate::difference_in_years_datetime(left, right) / 10
}

/// Count complete hundred-year periods from right to left.
pub fn difference_in_centuries_datetime(left: NaiveDateTime, right: NaiveDateTime) -> i64 {
    crate::difference_in_years_datetime(left, right) / 100
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, m: u32, d: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_nano_opt(hour, 15, 16, 987_654_321)
            .unwrap()
    }

    #[test]
    fn differentiates_full_decades_from_boundary_counts() {
        let start = at(2020, 2, 29, 12);
        assert_eq!(
            difference_in_decades_datetime(at(2030, 2, 28, 11), start),
            0
        );
        assert_eq!(
            difference_in_decades_datetime(at(2030, 2, 28, 12), start),
            1
        );
        assert_eq!(
            difference_in_decades_datetime(at(2040, 2, 29, 12), start),
            2
        );
        assert_eq!(
            difference_in_decades_datetime(start, at(2030, 2, 28, 12)),
            -1
        );
    }

    #[test]
    fn counts_complete_centuries_and_truncates_partial_ones() {
        let start = at(2000, 2, 29, 12);
        assert_eq!(
            difference_in_centuries_datetime(at(2100, 2, 28, 11), start),
            0
        );
        assert_eq!(
            difference_in_centuries_datetime(at(2100, 2, 28, 12), start),
            1
        );
        assert_eq!(
            difference_in_centuries_datetime(at(2200, 2, 28, 12), start),
            2
        );
        assert_eq!(
            difference_in_centuries_datetime(start, at(2100, 2, 28, 12)),
            -1
        );
    }

    #[test]
    fn spans_chrono_limits_and_equal_datetimes() {
        let decade_span = difference_in_decades_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        let century_span = difference_in_centuries_datetime(NaiveDateTime::MAX, NaiveDateTime::MIN);
        assert!(decade_span > century_span);
        assert!(century_span > 0);
        assert_eq!(
            difference_in_decades_datetime(NaiveDateTime::MAX, NaiveDateTime::MAX),
            0
        );
    }
}
