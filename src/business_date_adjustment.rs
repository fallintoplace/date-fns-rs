//! Business-day rolling conventions for settlement and payment dates.

use chrono::{Datelike, NaiveDate};

/// Choose a direction for rolling non-business dates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BusinessDayAdjustment {
    Following,
    Preceding,
    ModifiedFollowing,
    ModifiedPreceding,
}

/// Adjust a date using Monday-Friday business days and supplied holidays.
///
/// Modified conventions keep the resulting date in the original month when
/// possible. Returns None if the chosen direction exceeds Chrono's bounds.
pub fn adjust_business_date(
    date: NaiveDate,
    convention: BusinessDayAdjustment,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if crate::is_business_day_with_holidays(date, holidays) {
        return Some(date);
    }
    match convention {
        BusinessDayAdjustment::Following => crate::next_business_day_with_holidays(date, holidays),
        BusinessDayAdjustment::Preceding => {
            crate::previous_business_day_with_holidays(date, holidays)
        }
        BusinessDayAdjustment::ModifiedFollowing => {
            let following = crate::next_business_day_with_holidays(date, holidays)?;
            if following.month() == date.month() {
                Some(following)
            } else {
                crate::previous_business_day_with_holidays(date, holidays)
            }
        }
        BusinessDayAdjustment::ModifiedPreceding => {
            let preceding = crate::previous_business_day_with_holidays(date, holidays)?;
            if preceding.month() == date.month() {
                Some(preceding)
            } else {
                crate::next_business_day_with_holidays(date, holidays)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn modified_following_stays_in_month_across_holidays() {
        let holidays = [date(2024, 3, 29), date(2024, 4, 1)];
        let sunday = date(2024, 3, 31);
        assert_eq!(
            adjust_business_date(sunday, BusinessDayAdjustment::Following, &holidays),
            Some(date(2024, 4, 2))
        );
        assert_eq!(
            adjust_business_date(sunday, BusinessDayAdjustment::ModifiedFollowing, &holidays),
            Some(date(2024, 3, 28))
        );
    }

    #[test]
    fn modified_preceding_stays_in_month_and_business_days_do_not_move() {
        let first = date(2024, 6, 1);
        assert_eq!(
            adjust_business_date(first, BusinessDayAdjustment::Preceding, &[]),
            Some(date(2024, 5, 31))
        );
        assert_eq!(
            adjust_business_date(first, BusinessDayAdjustment::ModifiedPreceding, &[]),
            Some(date(2024, 6, 3))
        );
        assert_eq!(
            adjust_business_date(date(2024, 6, 3), BusinessDayAdjustment::Following, &[]),
            Some(date(2024, 6, 3))
        );
    }
}
