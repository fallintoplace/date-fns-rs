//! Day ordinals and complete lengths of Gregorian decades.

use chrono::NaiveDate;

/// Return the one-based day position within the date's Gregorian decade.
///
/// Returns None if the decade start falls outside Chrono's supported range.
pub fn get_day_of_decade(date: NaiveDate) -> Option<u32> {
    let start = crate::start_of_decade(date)?;
    u32::try_from(date.signed_duration_since(start).num_days() + 1).ok()
}

/// Return the number of calendar days in the complete decade.
///
/// Returns None if either decade boundary cannot be represented.
pub fn get_days_in_decade(date: NaiveDate) -> Option<u32> {
    let start = crate::start_of_decade(date)?;
    let end = crate::end_of_decade(date)?;
    u32::try_from(end.signed_duration_since(start).num_days() + 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_days_in_leap_and_common_decades() {
        let leap = day(2024, 2, 29);
        assert_eq!(get_day_of_decade(day(2020, 1, 1)), Some(1));
        assert_eq!(get_day_of_decade(leap), Some(1521));
        assert_eq!(get_days_in_decade(leap), Some(3653));
        assert_eq!(get_days_in_decade(day(2010, 1, 1)), Some(3652));
        assert_eq!(get_day_of_decade(day(2029, 12, 31)), Some(3653));
    }

    #[test]
    fn handles_year_zero_and_date_limits() {
        let last_bce = day(-1, 12, 31);
        assert_eq!(get_days_in_decade(last_bce), Some(3652));
        assert_eq!(get_day_of_decade(last_bce), Some(3652));
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            if let (Some(day), Some(days)) = (get_day_of_decade(edge), get_days_in_decade(edge)) {
                assert!((1..=days).contains(&day));
            }
        }
    }
}
