//! Workday counts and one-based positions within Gregorian quarters.

use chrono::NaiveDate;

/// Count Monday-Friday dates in the quarter, ignoring public holidays.
pub fn get_business_days_in_quarter(date: NaiveDate) -> u32 {
    crate::count_business_days_in_interval(
        crate::start_of_quarter(date),
        crate::end_of_quarter(date),
    )
    .expect("quarter boundaries are ordered") as u32
}

/// Return the one-based workday position in the quarter, or None on weekends.
pub fn get_business_day_of_quarter(date: NaiveDate) -> Option<u32> {
    if !crate::is_business_day(date) {
        return None;
    }
    Some(
        crate::count_business_days_in_interval(crate::start_of_quarter(date), date)
            .expect("quarter start precedes the date") as u32,
    )
}

/// Count workdays in the quarter after excluding caller-supplied holidays.
///
/// Duplicate entries and weekend holidays are ignored.
pub fn get_business_days_in_quarter_with_holidays(date: NaiveDate, holidays: &[NaiveDate]) -> u32 {
    crate::count_business_days_in_interval_excluding_holidays(
        crate::start_of_quarter(date),
        crate::end_of_quarter(date),
        holidays,
    )
    .expect("quarter boundaries are ordered") as u32
}

/// Return the one-based holiday-adjusted workday ordinal, or None on days off.
pub fn get_business_day_of_quarter_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<u32> {
    if !crate::is_business_day(date) || holidays.contains(&date) {
        return None;
    }
    Some(
        crate::count_business_days_in_interval_excluding_holidays(
            crate::start_of_quarter(date),
            date,
            holidays,
        )
        .expect("quarter start precedes the date") as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_leap_quarter_workdays_and_rejects_weekends() {
        assert_eq!(get_business_days_in_quarter(date(2024, 2, 29)), 65);
        assert_eq!(get_business_day_of_quarter(date(2024, 1, 1)), Some(1));
        assert_eq!(get_business_day_of_quarter(date(2024, 3, 29)), Some(65));
        assert_eq!(get_business_day_of_quarter(date(2024, 3, 30)), None);
        assert_eq!(get_business_day_of_quarter(date(2024, 4, 1)), Some(1));
    }

    #[test]
    fn holiday_counts_ignore_duplicates_weekends_and_other_quarters() {
        let holidays = [
            date(2024, 1, 1),
            date(2024, 2, 29),
            date(2024, 2, 29),
            date(2024, 3, 2),
            date(2024, 4, 1),
        ];
        assert_eq!(
            get_business_days_in_quarter_with_holidays(date(2024, 2, 1), &holidays),
            63
        );
        assert_eq!(
            get_business_day_of_quarter_with_holidays(date(2024, 1, 2), &holidays),
            Some(1)
        );
        assert_eq!(
            get_business_day_of_quarter_with_holidays(date(2024, 3, 29), &holidays),
            Some(63)
        );
        assert_eq!(
            get_business_day_of_quarter_with_holidays(date(2024, 2, 29), &holidays),
            None
        );
        assert_eq!(
            get_business_days_in_quarter_with_holidays(date(2024, 2, 1), &[]),
            get_business_days_in_quarter(date(2024, 2, 1))
        );
    }

    #[test]
    fn supported_date_limits_have_consistent_ordinals() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            let total = get_business_days_in_quarter(date);
            assert!((64..=66).contains(&total));
            if let Some(position) = get_business_day_of_quarter(date) {
                assert!((1..=total).contains(&position));
            }
        }
    }
}
