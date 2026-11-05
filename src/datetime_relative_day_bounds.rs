//! Inclusive local-day boundaries relative to an explicit reference datetime.

use chrono::NaiveDateTime;

/// Return midnight and the last nanosecond of a relative local calendar day.
///
/// Offset zero selects today, -1 yesterday, and 1 tomorrow, relative to
/// `reference`. Returns None if the shifted date is out of range.
pub fn relative_day_datetime_bounds(
    reference: NaiveDateTime,
    day_offset: i64,
) -> Option<(NaiveDateTime, NaiveDateTime)> {
    let day = crate::add_days(reference.date(), day_offset)?;
    let start = day.and_hms_opt(0, 0, 0)?;
    let end = day.and_hms_nano_opt(23, 59, 59, 999_999_999)?;
    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn resolves_today_yesterday_and_tomorrow_across_leap_days() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let reference = date.and_hms_opt(14, 15, 16).unwrap();
        let (today_start, today_end) = relative_day_datetime_bounds(reference, 0).unwrap();
        assert_eq!(today_start, date.and_hms_opt(0, 0, 0).unwrap());
        assert_eq!(
            today_end,
            date.and_hms_nano_opt(23, 59, 59, 999_999_999).unwrap()
        );
        let (yesterday_start, _) = relative_day_datetime_bounds(reference, -1).unwrap();
        let yesterday = yesterday_start.date();
        assert_eq!(yesterday, NaiveDate::from_ymd_opt(2024, 2, 28).unwrap());
        let tomorrow = relative_day_datetime_bounds(reference, 1).unwrap().0.date();
        assert_eq!(tomorrow, NaiveDate::from_ymd_opt(2024, 3, 1).unwrap());
    }

    #[test]
    fn handles_date_extremes_and_large_offsets() {
        assert_eq!(relative_day_datetime_bounds(NaiveDateTime::MAX, 1), None);
        assert_eq!(relative_day_datetime_bounds(NaiveDateTime::MIN, -1), None);
        assert_eq!(
            relative_day_datetime_bounds(NaiveDateTime::MIN, i64::MIN),
            None
        );
        let (_, last) = relative_day_datetime_bounds(NaiveDateTime::MAX, 0).unwrap();
        assert_eq!(last, NaiveDateTime::MAX);
    }
}
