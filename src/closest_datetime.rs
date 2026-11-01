//! Closest datetime selection across the full supported range.

use chrono::NaiveDateTime;

fn unix_nanos(value: NaiveDateTime) -> i128 {
    let utc = value.and_utc();
    i128::from(utc.timestamp()) * 1_000_000_000 + i128::from(utc.timestamp_subsec_nanos())
}

/// Return the index of the nearest datetime, preserving input order on ties.
///
/// The distance uses i128 and works across Chrono's complete datetime range.
pub fn closest_datetime_index_to(
    target: NaiveDateTime,
    candidates: &[NaiveDateTime],
) -> Option<usize> {
    let target_nanos = unix_nanos(target);
    let mut closest: Option<(usize, i128)> = None;

    for (index, &candidate) in candidates.iter().enumerate() {
        let distance = (unix_nanos(candidate) - target_nanos).abs();
        if closest.map_or(true, |(_, best)| distance < best) {
            closest = Some((index, distance));
        }
    }
    closest.map(|(index, _)| index)
}

/// Return the closest datetime, or None if no candidates exist.
pub fn closest_datetime_to(
    target: NaiveDateTime,
    candidates: &[NaiveDateTime],
) -> Option<NaiveDateTime> {
    closest_datetime_index_to(target, candidates).map(|index| candidates[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeDelta};

    #[test]
    fn respects_nanoseconds_and_stable_ties() {
        let target = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let before = target
            .checked_sub_signed(TimeDelta::nanoseconds(2))
            .unwrap();
        let after = target
            .checked_add_signed(TimeDelta::nanoseconds(2))
            .unwrap();
        assert_eq!(closest_datetime_index_to(target, &[after, before]), Some(0));
        assert_eq!(closest_datetime_to(target, &[before, after]), Some(before));
        assert_eq!(closest_datetime_index_to(target, &[]), None);
    }

    #[test]
    fn compares_extreme_datetime_values_without_overflow() {
        let dates = [NaiveDateTime::MAX, NaiveDateTime::MIN];
        assert_eq!(
            closest_datetime_to(NaiveDateTime::MIN, &dates),
            Some(NaiveDateTime::MIN)
        );
        assert_eq!(
            closest_datetime_index_to(NaiveDateTime::MAX, &dates),
            Some(0)
        );
    }
}
