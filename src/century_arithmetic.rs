//! Checked calendar-century arithmetic with leap-day clamping.

use chrono::NaiveDate;

/// Add signed centuries (100 calendar years per century).
///
/// Invalid offsets and dates outside Chrono's range return None.
pub fn add_centuries(date: NaiveDate, centuries: i32) -> Option<NaiveDate> {
    crate::add_years(date, centuries.checked_mul(100)?)
}

/// Subtract signed centuries, clamping February 29 when needed.
pub fn sub_centuries(date: NaiveDate, centuries: i32) -> Option<NaiveDate> {
    crate::sub_years(date, centuries.checked_mul(100)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn clamps_leap_days_and_accepts_negative_offsets() {
        assert_eq!(add_centuries(date(2000, 2, 29), 1), Some(date(2100, 2, 28)));
        assert_eq!(sub_centuries(date(2000, 2, 29), 1), Some(date(1900, 2, 28)));
        assert_eq!(
            add_centuries(date(2024, 6, 15), -1),
            Some(date(1924, 6, 15))
        );
        assert_eq!(
            sub_centuries(date(2024, 6, 15), -1),
            Some(date(2124, 6, 15))
        );
        assert_eq!(add_centuries(date(2024, 6, 15), 0), Some(date(2024, 6, 15)));
    }

    #[test]
    fn refuses_unrepresentable_century_shifts() {
        assert_eq!(add_centuries(NaiveDate::MAX, 1), None);
        assert_eq!(sub_centuries(NaiveDate::MIN, 1), None);
        assert_eq!(add_centuries(date(2024, 1, 1), i32::MAX), None);
        assert_eq!(sub_centuries(date(2024, 1, 1), i32::MIN), None);
    }
}
