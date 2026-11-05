//! Atomic replacement of calendar date fields.

use chrono::{Datelike, NaiveDate};

/// Optional calendar fields to replace in a single checked operation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateFieldPatch {
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

/// Replace selected date fields without intermediate invalid dates.
///
/// Omitted fields retain their original values. Returns None when the
/// resulting date does not exist or is outside Chrono's supported range.
pub fn set_date_fields(date: NaiveDate, patch: DateFieldPatch) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(
        patch.year.unwrap_or(date.year()),
        patch.month.unwrap_or(date.month()),
        patch.day.unwrap_or(date.day()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_year_month_and_day_together() {
        let leap_day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let patch = DateFieldPatch {
            year: Some(2023),
            month: Some(3),
            day: Some(1),
        };
        assert_eq!(
            set_date_fields(leap_day, patch),
            NaiveDate::from_ymd_opt(2023, 3, 1)
        );
        assert_eq!(
            set_date_fields(leap_day, DateFieldPatch::default()),
            Some(leap_day)
        );
    }

    #[test]
    fn rejects_invalid_final_dates_without_partial_updates() {
        let leap_day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            set_date_fields(
                leap_day,
                DateFieldPatch {
                    year: Some(2023),
                    ..DateFieldPatch::default()
                }
            ),
            None
        );
        assert_eq!(
            set_date_fields(
                leap_day,
                DateFieldPatch {
                    day: Some(31),
                    ..DateFieldPatch::default()
                }
            ),
            None
        );
        assert_eq!(
            set_date_fields(NaiveDate::MAX, DateFieldPatch::default()),
            Some(NaiveDate::MAX)
        );
    }
}
