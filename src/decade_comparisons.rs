//! Calendar decade comparisons with configurable decade-year boundaries.

use chrono::{Datelike, NaiveDate};

fn decade_index(date: NaiveDate, first_year_offset: u32) -> Option<i64> {
    if first_year_offset > 9 {
        return None;
    }
    Some((i64::from(date.year()) - i64::from(first_year_offset)).div_euclid(10))
}

/// Count signed decade boundaries between dates using a decade start offset.
///
/// Offset 0 uses decades 2020-2029; offset 1 uses decades 2021-2030.
/// Returns None for offsets outside 0 through 9.
pub fn difference_in_calendar_decades_with_offset(
    left: NaiveDate,
    right: NaiveDate,
    first_year_offset: u32,
) -> Option<i64> {
    Some(decade_index(left, first_year_offset)? - decade_index(right, first_year_offset)?)
}

/// Check whether two dates share a standard calendar decade (2020-2029).
pub fn is_same_decade(left: NaiveDate, right: NaiveDate) -> bool {
    left.year().div_euclid(10) == right.year().div_euclid(10)
}

/// Check whether dates share a configurable calendar decade.
pub fn is_same_decade_with_offset(
    left: NaiveDate,
    right: NaiveDate,
    first_year_offset: u32,
) -> Option<bool> {
    Some(difference_in_calendar_decades_with_offset(left, right, first_year_offset)? == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap()
    }

    #[test]
    fn distinguishes_standard_and_offset_decades() {
        assert!(is_same_decade(date(2020), date(2029)));
        assert!(!is_same_decade(date(2019), date(2020)));
        assert_eq!(
            difference_in_calendar_decades_with_offset(date(2020), date(2019), 0),
            Some(1)
        );
        assert_eq!(
            difference_in_calendar_decades_with_offset(date(2020), date(2019), 1),
            Some(0)
        );
        assert_eq!(
            is_same_decade_with_offset(date(2020), date(2019), 1),
            Some(true)
        );
        assert_eq!(is_same_decade_with_offset(date(2020), date(2019), 10), None);
    }

    #[test]
    fn uses_euclidean_decades_across_year_zero() {
        assert!(is_same_decade(date(-10), date(-1)));
        assert!(!is_same_decade(date(-1), date(0)));
        assert_eq!(
            difference_in_calendar_decades_with_offset(date(0), date(-1), 0),
            Some(1)
        );
        assert_eq!(
            difference_in_calendar_decades_with_offset(date(-1), date(-10), 0),
            Some(0)
        );
    }

    #[test]
    fn supports_chrono_extremes_and_rejects_invalid_offsets() {
        let forward =
            difference_in_calendar_decades_with_offset(NaiveDate::MAX, NaiveDate::MIN, 7).unwrap();
        assert!(forward > 0);
        assert_eq!(
            difference_in_calendar_decades_with_offset(NaiveDate::MIN, NaiveDate::MAX, 7),
            Some(-forward)
        );
        assert_eq!(
            difference_in_calendar_decades_with_offset(NaiveDate::MAX, NaiveDate::MIN, u32::MAX),
            None
        );
    }
}
