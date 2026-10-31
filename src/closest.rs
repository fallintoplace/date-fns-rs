//! Find the closest calendar date in an iterable.

use std::borrow::Borrow;

use chrono::{Datelike, NaiveDate};

/// Find the candidate nearest to target by absolute calendar-day distance.
///
/// Returns None for an empty iterator. Ties keep the first candidate. Both
/// owned dates and references to dates can be provided.
pub fn closest_to<I, D>(target: NaiveDate, candidates: I) -> Option<NaiveDate>
where
    I: IntoIterator<Item = D>,
    D: Borrow<NaiveDate>,
{
    let target_day = i64::from(target.num_days_from_ce());
    let mut nearest = None;
    let mut distance = i64::MAX;

    for candidate in candidates {
        let value = *candidate.borrow();
        let difference = (i64::from(value.num_days_from_ce()) - target_day).abs();
        if difference < distance {
            distance = difference;
            nearest = Some(value);
        }
    }
    nearest
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn chooses_nearest_date_and_prefers_first_on_ties() {
        let target = date(2024, 2, 29);
        assert_eq!(
            closest_to(target, [date(2024, 3, 3), date(2024, 2, 28)]),
            Some(date(2024, 2, 28))
        );
        assert_eq!(
            closest_to(target, [date(2024, 2, 28), date(2024, 3, 1)]),
            Some(date(2024, 2, 28))
        );
        assert_eq!(closest_to(target, [target]), Some(target));
    }

    #[test]
    fn supports_borrowed_candidates_and_empty_iterators() {
        let candidates = [date(2024, 1, 1), date(2024, 1, 5)];
        assert_eq!(
            closest_to(date(2024, 1, 4), &candidates),
            Some(date(2024, 1, 5))
        );
        assert_eq!(closest_to(date(2024, 1, 4), []), None);
    }

    #[test]
    fn avoids_overflow_across_supported_date_range() {
        assert_eq!(
            closest_to(NaiveDate::MIN, [NaiveDate::MAX, NaiveDate::MIN]),
            Some(NaiveDate::MIN)
        );
        assert_eq!(
            closest_to(NaiveDate::MAX, [NaiveDate::MIN]),
            Some(NaiveDate::MIN)
        );
    }
}
