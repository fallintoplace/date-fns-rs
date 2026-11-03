//! Workday counts and one-based positions within Gregorian half-years.

use chrono::NaiveDate;

/// Count Monday-Friday dates in the containing calendar half-year.
pub fn get_business_days_in_half_year(date: NaiveDate) -> u32 {
    crate::count_business_days_in_interval(
        crate::start_of_half_year(date),
        crate::end_of_half_year(date),
    )
    .expect("half-year boundaries are ordered") as u32
}

/// Return the one-based workday ordinal within a half-year, or None on weekends.
pub fn get_business_day_of_half_year(date: NaiveDate) -> Option<u32> {
    if !crate::is_business_day(date) {
        return None;
    }
    Some(
        crate::count_business_days_in_interval(crate::start_of_half_year(date), date)
            .expect("half-year start precedes the date") as u32,
    )
}

/// Count half-year workdays, excluding duplicate and weekend holiday dates.
pub fn get_business_days_in_half_year_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> u32 {
    crate::count_business_days_in_interval_excluding_holidays(
        crate::start_of_half_year(date),
        crate::end_of_half_year(date),
        holidays,
    )
    .expect("half-year boundaries are ordered") as u32
}

/// Return the holiday-adjusted workday ordinal, or None on a non-workday.
pub fn get_business_day_of_half_year_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<u32> {
    if !crate::is_business_day(date) || holidays.contains(&date) {
        return None;
    }
    Some(
        crate::count_business_days_in_interval_excluding_holidays(
            crate::start_of_half_year(date),
            date,
            holidays,
        )
        .expect("half-year start precedes the date") as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn leap_year_halves_have_distinct_workday_totals() {
        assert_eq!(get_business_days_in_half_year(date(2024, 2, 29)), 130);
        assert_eq!(get_business_days_in_half_year(date(2024, 12, 31)), 132);
        assert_eq!(get_business_day_of_half_year(date(2024, 1, 1)), Some(1));
        assert_eq!(get_business_day_of_half_year(date(2024, 6, 28)), Some(130));
        assert_eq!(get_business_day_of_half_year(date(2024, 6, 29)), None);
        assert_eq!(get_business_day_of_half_year(date(2024, 7, 1)), Some(1));
        assert_eq!(get_business_day_of_half_year(date(2024, 12, 31)), Some(132));
    }

    #[test]
    fn holiday_aware_half_year_ignores_duplicates_and_non_workdays() {
        let holidays = [
            date(2024, 1, 1),
            date(2024, 6, 28),
            date(2024, 6, 28),
            date(2024, 6, 29),
            date(2024, 7, 1),
        ];
        assert_eq!(
            get_business_days_in_half_year_with_holidays(date(2024, 2, 29), &holidays),
            128
        );
        assert_eq!(
            get_business_day_of_half_year_with_holidays(date(2024, 1, 2), &holidays),
            Some(1)
        );
        assert_eq!(
            get_business_day_of_half_year_with_holidays(date(2024, 6, 27), &holidays),
            Some(128)
        );
        assert_eq!(
            get_business_day_of_half_year_with_holidays(date(2024, 6, 28), &holidays),
            None
        );
        assert_eq!(
            get_business_days_in_half_year_with_holidays(date(2024, 2, 29), &[]),
            get_business_days_in_half_year(date(2024, 2, 29))
        );
    }

    #[test]
    fn supported_limits_have_consistent_business_day_numbers() {
        for date in [NaiveDate::MIN, NaiveDate::MAX] {
            let total = get_business_days_in_half_year(date);
            assert!((128..=132).contains(&total));
            if let Some(position) = get_business_day_of_half_year(date) {
                assert!((1..=total).contains(&position));
            }
        }
    }
}
