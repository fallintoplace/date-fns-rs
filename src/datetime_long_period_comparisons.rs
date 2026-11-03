//! Decade and century comparisons for naive datetimes.

use chrono::NaiveDateTime;

/// Whether both timestamps fall in the same standard Gregorian decade.
///
/// Decades begin in a year ending in zero, including for negative years.
pub fn is_same_decade_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    crate::is_same_decade(left.date(), right.date())
}

/// Whether both timestamps fall in the same one-based Gregorian century.
///
/// For example, 2001 through 2100 belong to the same century.
pub fn is_same_century_datetime(left: NaiveDateTime, right: NaiveDateTime) -> bool {
    crate::is_same_century(left.date(), right.date())
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
    fn compares_decades_ignoring_month_and_clock() {
        assert!(is_same_decade_datetime(
            at(2020, 1, 1, 0),
            at(2029, 12, 31, 23)
        ));
        assert!(!is_same_decade_datetime(
            at(2019, 12, 31, 23),
            at(2020, 1, 1, 0)
        ));
        assert!(is_same_decade_datetime(
            at(-10, 1, 1, 0),
            at(-1, 12, 31, 23)
        ));
        assert!(!is_same_decade_datetime(at(-1, 12, 31, 23), at(0, 1, 1, 0)));
    }

    #[test]
    fn compares_one_based_centuries_and_supported_limits() {
        assert!(is_same_century_datetime(
            at(2001, 1, 1, 0),
            at(2100, 12, 31, 23)
        ));
        assert!(!is_same_century_datetime(
            at(2000, 12, 31, 23),
            at(2001, 1, 1, 0)
        ));
        assert!(is_same_century_datetime(
            NaiveDateTime::MIN,
            NaiveDateTime::MIN
        ));
        assert!(is_same_decade_datetime(
            NaiveDateTime::MAX,
            NaiveDateTime::MAX
        ));
    }
}
