//! Signed business-day differences with supplied holiday exclusions.

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::business_days::is_business_day;
use crate::business_difference::difference_in_business_days;

/// Count business days in the signed half-open interval between two dates.
///
/// Positive counts use (right, left]; negative counts reverse the dates.
/// Excludes Saturdays, Sundays, and any distinct weekday in `holidays`.
/// The starting date is excluded even if it is a holiday.
pub fn difference_in_business_days_with_holidays(
    left: NaiveDate,
    right: NaiveDate,
    holidays: &[NaiveDate],
) -> i64 {
    if left == right {
        return 0;
    }
    let (start, end, sign) = if left > right {
        (right, left, 1)
    } else {
        (left, right, -1)
    };

    let excluded: BTreeSet<_> = holidays
        .iter()
        .copied()
        .filter(|day| *day > start && *day <= end && is_business_day(*day))
        .collect();
    let count = difference_in_business_days(end, start) - excluded.len() as i64;
    count * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn deducts_holidays_without_double_counting() {
        let friday = date(2024, 3, 1);
        let tuesday = date(2024, 3, 5);
        let holidays = [date(2024, 3, 4), date(2024, 3, 4)];
        assert_eq!(
            difference_in_business_days_with_holidays(tuesday, friday, &holidays),
            1
        );
        assert_eq!(
            difference_in_business_days_with_holidays(friday, tuesday, &holidays),
            -1
        );
    }

    #[test]
    fn excludes_start_but_includes_end_and_ignores_weekends() {
        let friday = date(2024, 3, 1);
        let tuesday = date(2024, 3, 5);
        let holidays = [friday, date(2024, 3, 2), tuesday];
        assert_eq!(
            difference_in_business_days_with_holidays(tuesday, friday, &holidays),
            1
        );
        assert_eq!(
            difference_in_business_days_with_holidays(tuesday, tuesday, &holidays),
            0
        );
    }

    #[test]
    fn preserves_plain_business_differences_without_exclusions() {
        let monday = date(2024, 2, 26);
        let friday = date(2024, 3, 8);
        assert_eq!(
            difference_in_business_days_with_holidays(friday, monday, &[]),
            difference_in_business_days(friday, monday)
        );
        assert_eq!(
            difference_in_business_days_with_holidays(NaiveDate::MAX, NaiveDate::MIN, &[]),
            difference_in_business_days(NaiveDate::MAX, NaiveDate::MIN)
        );
    }
}
