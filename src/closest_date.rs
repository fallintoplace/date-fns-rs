//! Nearest-date selection without date-range overflow.

use chrono::{Datelike, NaiveDate};

/// Return the index of the closest candidate to `date`.
///
/// Ties select the first candidate. An empty slice returns None.
pub fn closest_date_index_to(date: NaiveDate, candidates: &[NaiveDate]) -> Option<usize> {
    let target = i64::from(date.num_days_from_ce());
    let mut best: Option<(usize, i64)> = None;

    for (index, candidate) in candidates.iter().enumerate() {
        let distance = (i64::from(candidate.num_days_from_ce()) - target).abs();
        if best.map_or(true, |(_, best_distance)| distance < best_distance) {
            best = Some((index, distance));
        }
    }

    best.map(|(index, _)| index)
}

/// Return the closest date from a slice, preferring the first on a tie.
///
/// Returns None if no candidates were provided.
pub fn closest_date_to(date: NaiveDate, candidates: &[NaiveDate]) -> Option<NaiveDate> {
    closest_date_index_to(date, candidates).map(|index| candidates[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn selects_the_nearest_date_and_first_on_ties() {
        let candidates = [date(2024, 1, 9), date(2024, 1, 11), date(2024, 1, 20)];
        let target = date(2024, 1, 10);
        assert_eq!(closest_date_index_to(target, &candidates), Some(0));
        assert_eq!(closest_date_to(target, &candidates), Some(candidates[0]));
        assert_eq!(
            closest_date_index_to(date(2024, 1, 19), &candidates),
            Some(2)
        );
        assert_eq!(
            closest_date_to(candidates[1], &candidates),
            Some(candidates[1])
        );
    }

    #[test]
    fn handles_empty_candidates_and_date_extremes() {
        assert_eq!(closest_date_index_to(date(2024, 1, 1), &[]), None);
        assert_eq!(closest_date_to(date(2024, 1, 1), &[]), None);
        let candidates = [NaiveDate::MAX, NaiveDate::MIN];
        assert_eq!(closest_date_index_to(NaiveDate::MIN, &candidates), Some(1));
        assert_eq!(
            closest_date_to(NaiveDate::MAX, &candidates),
            Some(NaiveDate::MAX)
        );
    }
}
