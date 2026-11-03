//! Date-only recurrence sets with explicit inclusions and exclusions.

use std::collections::{BTreeSet, HashSet};

use chrono::NaiveDate;

/// Combine finite RRULE occurrences, RDATE inclusions, and EXDATE exclusions.
///
/// The result is sorted chronologically and deduplicated. EXDATE takes
/// precedence over both the generated dates and explicit RDATE values.
/// The caller is responsible for including DTSTART in the rule dates.
pub fn combine_ical_date_recurrences(
    rule_dates: &[NaiveDate],
    additional_dates: &[NaiveDate],
    excluded_dates: &[NaiveDate],
) -> Vec<NaiveDate> {
    let excluded: HashSet<_> = excluded_dates.iter().copied().collect();
    let included: BTreeSet<_> = rule_dates
        .iter()
        .chain(additional_dates.iter())
        .copied()
        .filter(|date| !excluded.contains(date))
        .collect();
    included.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).unwrap()
    }

    #[test]
    fn merges_sorts_deduplicates_and_applies_exclusions() {
        let rule = [date(3), date(1), date(4)];
        let additional = [date(2), date(3), date(4)];
        let excluded = [date(4), date(4)];
        assert_eq!(
            combine_ical_date_recurrences(&rule, &additional, &excluded),
            vec![date(1), date(2), date(3)]
        );
    }

    #[test]
    fn handles_empty_sets_and_chrono_boundaries() {
        assert!(combine_ical_date_recurrences(&[], &[], &[]).is_empty());
        assert_eq!(
            combine_ical_date_recurrences(
                &[NaiveDate::MAX, NaiveDate::MIN],
                &[NaiveDate::MIN],
                &[NaiveDate::MAX]
            ),
            vec![NaiveDate::MIN]
        );
    }

    #[test]
    fn exclusions_override_every_source() {
        let day = date(1);
        assert!(combine_ical_date_recurrences(&[day], &[day], &[day]).is_empty());
    }
}
