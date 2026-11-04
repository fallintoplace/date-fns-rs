//! Calendar quarter metadata for naive date-times without timezone inference.

use chrono::NaiveDateTime;

/// Return the quarter number from 1 to 4.
pub fn get_quarter_datetime(value: NaiveDateTime) -> u32 {
    crate::get_quarter(value.date())
}

/// Return the 1-based ordinal day inside the calendar quarter.
pub fn get_day_of_quarter_datetime(value: NaiveDateTime) -> u32 {
    crate::get_day_of_quarter(value.date())
}

/// Return the total days in the current calendar quarter.
pub fn get_days_in_quarter_datetime(value: NaiveDateTime) -> u32 {
    crate::get_days_in_quarter(value.date())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn datetime(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap()
    }

    #[test]
    fn includes_leap_day_in_quarter_metadata() {
        let leap = datetime(2024, 2, 29);
        assert_eq!(get_quarter_datetime(leap), 1);
        assert_eq!(get_day_of_quarter_datetime(leap), 60);
        assert_eq!(get_days_in_quarter_datetime(leap), 91);
        assert_eq!(get_day_of_quarter_datetime(datetime(2024, 4, 1)), 1);
        assert_eq!(get_days_in_quarter_datetime(datetime(2023, 2, 28)), 90);
    }

    #[test]
    fn ignores_clock_fields_and_supports_boundaries() {
        let late = datetime(2024, 12, 31);
        let early = late.date().and_hms_opt(0, 0, 0).unwrap();
        assert_eq!(get_quarter_datetime(late), get_quarter_datetime(early));
        assert_eq!(
            get_day_of_quarter_datetime(late),
            get_day_of_quarter_datetime(early)
        );
        assert_eq!(get_quarter_datetime(late), 4);
        assert_eq!(get_day_of_quarter_datetime(late), 92);
        for value in [NaiveDateTime::MIN, NaiveDateTime::MAX] {
            assert!((1..=4).contains(&get_quarter_datetime(value)));
            assert!((1..=92).contains(&get_day_of_quarter_datetime(value)));
            assert!((90..=92).contains(&get_days_in_quarter_datetime(value)));
        }
    }
}
