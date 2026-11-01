//! Inclusive business-day counts with caller-specified holiday exclusions.

use std::collections::BTreeSet;

use chrono::NaiveDate;

/// Count Monday-Friday dates in an interval excluding supplied holidays.
///
/// Duplicate and weekend holidays have no effect. Reversed intervals return
/// None. Runtime depends on the number of holidays, not the length of the range.
pub fn count_business_days_in_interval_excluding_holidays(
    start: NaiveDate,
    end: NaiveDate,
    holidays: &[NaiveDate],
) -> Option<i64> {
    let base = crate::count_business_days_in_interval(start, end)?;
    let excluded: BTreeSet<_> = holidays
        .iter()
        .copied()
        .filter(|date| *date >= start && *date <= end && crate::is_business_day(*date))
        .collect();
    Some(base - i64::try_from(excluded.len()).ok()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(month: u32, date: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, month, date).unwrap()
    }

    #[test]
    fn ignores_duplicates_weekends_and_outside_holidays() {
        let holidays = [
            day(2, 29),
            day(2, 29),
            day(3, 2),
            day(3, 4),
            day(3, 12),
        ];
        assert_eq!(
            count_business_days_in_interval_excluding_holidays(
                day(2, 28),
                day(3, 5),
                &holidays
            ),
            Some(3)
        );
        assert_eq!(
            count_business_days_in_interval_excluding_holidays(
                day(3, 2),
                day(3, 3),
                &holidays
            ),
            Some(0)
        );
    }

    #[test]
    fn validates_ranges_and_handles_supported_boundaries() {
        assert_eq!(
            count_business_days_in_interval_excluding_holidays(
                day(3, 2),
                day(3, 1),
                &[]
            ),
            None
        );
        let on_edge = i64::from(crate::is_business_day(NaiveDate::MAX));
        assert_eq!(
            count_business_days_in_interval_excluding_holidays(
                NaiveDate::MAX,
                NaiveDate::MAX,
                &[]
            ),
            Some(on_edge)
        );
        assert_eq!(
            count_business_days_in_interval_excluding_holidays(
                NaiveDate::MAX,
                NaiveDate::MAX,
                &[NaiveDate::MAX]
            ),
            Some(0)
        );
    }
}
