//! Strict next and previous occurrences of a naive local clock time.

use chrono::{NaiveDateTime, NaiveTime};

/// Find the first occurrence of a clock time strictly after a timestamp.
///
/// Returns None when the next calendar day is outside Chrono's range.
/// This function uses local naive time and makes no daylight-saving assumption.
pub fn next_occurrence_of_time(after: NaiveDateTime, time: NaiveTime) -> Option<NaiveDateTime> {
    let today = after.date().and_time(time);
    if today > after {
        Some(today)
    } else {
        Some(after.date().succ_opt()?.and_time(time))
    }
}

/// Find the last occurrence of a clock time strictly before a timestamp.
///
/// Returns None when the previous calendar day is outside Chrono's range.
pub fn previous_occurrence_of_time(
    before: NaiveDateTime,
    time: NaiveTime,
) -> Option<NaiveDateTime> {
    let today = before.date().and_time(time);
    if today < before {
        Some(today)
    } else {
        Some(before.date().pred_opt()?.and_time(time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn navigates_equal_and_cross_day_occurrences() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let next_day = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let clock = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
        let reference = day.and_time(clock);

        assert_eq!(
            next_occurrence_of_time(reference, clock),
            Some(next_day.and_time(clock))
        );
        assert_eq!(
            previous_occurrence_of_time(reference, clock),
            Some(
                NaiveDate::from_ymd_opt(2024, 2, 28)
                    .unwrap()
                    .and_time(clock)
            )
        );
        assert_eq!(
            next_occurrence_of_time(day.and_hms_opt(8, 0, 0).unwrap(), clock),
            Some(reference)
        );
        assert_eq!(
            previous_occurrence_of_time(day.and_hms_opt(10, 0, 0).unwrap(), clock),
            Some(reference)
        );
    }

    #[test]
    fn preserves_fractional_clock_time_and_bounds() {
        let clock = NaiveTime::from_hms_nano_opt(12, 0, 0, 123_456_789).unwrap();
        let start = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            next_occurrence_of_time(start, clock),
            Some(start.date().and_time(clock))
        );
        assert_eq!(next_occurrence_of_time(NaiveDateTime::MAX, clock), None);
        assert_eq!(previous_occurrence_of_time(NaiveDateTime::MIN, clock), None);
    }
}
