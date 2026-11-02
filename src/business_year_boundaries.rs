//! First and last Monday-Friday dates of Gregorian calendar years.

use chrono::NaiveDate;

fn find_boundary(
    date: NaiveDate,
    holidays: &[NaiveDate],
    from_start: bool,
) -> Option<NaiveDate> {
    let first = crate::start_of_year(date);
    let last = crate::end_of_year(date);
    let mut current = if from_start { first } else { last };
    loop {
        if crate::is_business_day_with_holidays(current, holidays) {
            return Some(current);
        }
        if (from_start && current == last) || (!from_start && current == first) {
            return None;
        }
        current = if from_start {
            current.succ_opt()?
        } else {
            current.pred_opt()?
        };
    }
}

/// Return the first Monday-Friday date in the calendar year.
pub fn first_business_day_of_year(date: NaiveDate) -> NaiveDate {
    find_boundary(date, &[], true).expect("a calendar year contains a weekday")
}

/// Return the last Monday-Friday date in the calendar year.
pub fn last_business_day_of_year(date: NaiveDate) -> NaiveDate {
    find_boundary(date, &[], false).expect("a calendar year contains a weekday")
}

/// Find the first weekday in the year excluding caller-supplied holidays.
///
/// Returns None if all weekdays are excluded.
pub fn first_business_day_of_year_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    find_boundary(date, holidays, true)
}

/// Find the last weekday in the year excluding caller-supplied holidays.
pub fn last_business_day_of_year_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    find_boundary(date, holidays, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn finds_both_year_boundaries_and_skips_weekends() {
        let middle = date(2024, 7, 4);
        assert_eq!(first_business_day_of_year(middle), date(2024, 1, 1));
        assert_eq!(last_business_day_of_year(middle), date(2024, 12, 31));
        let previous = date(2023, 8, 15);
        assert_eq!(first_business_day_of_year(previous), date(2023, 1, 2));
        assert_eq!(last_business_day_of_year(previous), date(2023, 12, 29));
    }

    #[test]
    fn skips_holiday_dates_and_detects_fully_blocked_years() {
        let middle = date(2024, 7, 4);
        let holidays = [date(2024, 1, 1), date(2024, 1, 2), date(2024, 1, 2),
                        date(2024, 12, 30), date(2024, 12, 31)];
        assert_eq!(
            first_business_day_of_year_with_holidays(middle, &holidays),
            Some(date(2024, 1, 3))
        );
        assert_eq!(
            last_business_day_of_year_with_holidays(middle, &holidays),
            Some(date(2024, 12, 27))
        );
        let all: Vec<_> = crate::each_business_day_of_interval(
            date(2024, 1, 1),
            date(2024, 12, 31),
        )
        .unwrap()
        .collect();
        assert_eq!(first_business_day_of_year_with_holidays(middle, &all), None);
        assert_eq!(last_business_day_of_year_with_holidays(middle, &all), None);
    }

    #[test]
    fn handles_chrono_date_limits() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let first = first_business_day_of_year(edge);
            let last = last_business_day_of_year(edge);
            assert!(first <= edge && last >= first);
            assert!(crate::is_business_day(first));
            assert!(crate::is_business_day(last));
        }
    }
}
