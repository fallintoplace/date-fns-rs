//! Financial T+n settlement dates with explicit holiday exclusions.

use chrono::NaiveDate;

/// Return the date reached after the given number of business-day settlements.
///
/// The trade day is excluded for T+n with n greater than zero. T+0 rolls
/// non-working trade days forward to the next business date. Weekends are
/// Saturday and Sunday, and supplied holidays are excluded.
pub fn settlement_date(
    trade_date: NaiveDate,
    lag_business_days: u32,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    if lag_business_days == 0 {
        if crate::is_business_day_with_holidays(trade_date, holidays) {
            Some(trade_date)
        } else {
            crate::next_business_day_with_holidays(trade_date, holidays)
        }
    } else {
        crate::add_business_days_with_holidays(trade_date, i64::from(lag_business_days), holidays)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn accounts_for_holidays_and_weekends_in_t_plus_two() {
        let friday = date(2024, 3, 1);
        let holidays = [date(2024, 3, 4)];
        assert_eq!(
            settlement_date(friday, 2, &holidays),
            Some(date(2024, 3, 6))
        );
        assert_eq!(
            settlement_date(friday, 1, &holidays),
            Some(date(2024, 3, 5))
        );
    }

    #[test]
    fn rolls_t_plus_zero_and_handles_chrono_bounds() {
        let sunday = date(2024, 3, 3);
        assert_eq!(settlement_date(sunday, 0, &[]), Some(date(2024, 3, 4)));
        assert_eq!(
            settlement_date(date(2024, 3, 4), 0, &[]),
            Some(date(2024, 3, 4))
        );
        assert_eq!(settlement_date(NaiveDate::MAX, 1, &[]), None);
    }
}
