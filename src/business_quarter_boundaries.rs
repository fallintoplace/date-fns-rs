//! First and last Monday-Friday dates of Gregorian calendar quarters.

use chrono::NaiveDate;

fn find_boundary(
    date: NaiveDate,
    holidays: &[NaiveDate],
    from_start: bool,
) -> Option<NaiveDate> {
    let first = crate::start_of_quarter(date);
    let last = crate::end_of_quarter(date);
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

/// Return the first Monday-Friday date in the date's calendar quarter.
pub fn first_business_day_of_quarter(date: NaiveDate) -> NaiveDate {
    find_boundary(date, &[], true).expect("a calendar quarter contains a weekday")
}

/// Return the last Monday-Friday date in the date's calendar quarter.
pub fn last_business_day_of_quarter(date: NaiveDate) -> NaiveDate {
    find_boundary(date, &[], false).expect("a calendar quarter contains a weekday")
}

/// Find the first weekday in the quarter excluding caller-supplied holidays.
///
/// Returns None if all weekdays of the quarter are excluded.
pub fn first_business_day_of_quarter_with_holidays(
    date: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<NaiveDate> {
    find_boundary(date, holidays, true)
}

/// Find the last weekday in the quarter excluding caller-supplied holidays.
pub fn last_business_day_of_quarter_with_holidays(
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
    fn skips_weekends_at_quarter_boundaries() {
        let march = date(2024, 3, 15);
        assert_eq!(first_business_day_of_quarter(march), date(2024, 1, 1));
        assert_eq!(last_business_day_of_quarter(march), date(2024, 3, 29));
        let december = date(2023, 12, 31);
        assert_eq!(first_business_day_of_quarter(december), date(2023, 10, 2));
        assert_eq!(last_business_day_of_quarter(december), date(2023, 12, 29));
    }

    #[test]
    fn excludes_duplicates_and_all_blocked_weekdays() {
        let march = date(2024, 3, 15);
        let holidays = [date(2024, 1, 1), date(2024, 1, 2), date(2024, 1, 2),
                        date(2024, 3, 28), date(2024, 3, 29)];
        assert_eq!(
            first_business_day_of_quarter_with_holidays(march, &holidays),
            Some(date(2024, 1, 3))
        );
        assert_eq!(
            last_business_day_of_quarter_with_holidays(march, &holidays),
            Some(date(2024, 3, 27))
        );
        let all: Vec<_> = crate::each_business_day_of_interval(
            date(2024, 1, 1),
            date(2024, 3, 31),
        )
        .unwrap()
        .collect();
        assert_eq!(first_business_day_of_quarter_with_holidays(march, &all), None);
        assert_eq!(last_business_day_of_quarter_with_holidays(march, &all), None);
    }

    #[test]
    fn supports_chrono_date_limits() {
        for edge in [NaiveDate::MIN, NaiveDate::MAX] {
            let first = first_business_day_of_quarter(edge);
            let last = last_business_day_of_quarter(edge);
            assert!(first <= edge && last >= first);
            assert!(crate::is_business_day(first));
            assert!(crate::is_business_day(last));
        }
    }
}
