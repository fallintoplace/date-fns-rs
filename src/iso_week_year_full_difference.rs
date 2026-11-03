//! Full ISO week-numbering year differences.
use chrono::{Datelike, NaiveDate};

fn iso_ordinal(date: NaiveDate) -> u32 {
    let week = date.iso_week().week();
    (week - 1) * 7 + date.weekday().number_from_monday()
}

/// Count full ISO week-numbering years from `right` to `left`.
///
/// Unlike calendar ISO-year differences, partial years do not count.
/// Dates in week 53 are compared by their ordinal, even if the other
/// ISO year only has 52 weeks.
pub fn difference_in_iso_week_years(left: NaiveDate, right: NaiveDate) -> i64 {
    let difference = i64::from(left.iso_week().year()) - i64::from(right.iso_week().year());
    if difference == 0 {
        return 0;
    }

    let partial = if difference > 0 {
        iso_ordinal(left) < iso_ordinal(right)
    } else {
        iso_ordinal(left) > iso_ordinal(right)
    };

    if partial {
        difference - difference.signum()
    } else {
        difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn counts_completed_iso_years_not_calendar_year_changes() {
        assert_eq!(difference_in_iso_week_years(day(2012, 1, 1), day(2010, 1, 1)), 1);
        assert_eq!(difference_in_iso_week_years(day(2024, 12, 30), day(2024, 1, 1)), 1);
        assert_eq!(difference_in_iso_week_years(day(2025, 1, 1), day(2024, 1, 8)), 0);
        assert_eq!(difference_in_iso_week_years(day(2021, 1, 3), day(2020, 12, 31)), 0);
    }

    #[test]
    fn handles_week_fifty_three_and_signed_differences() {
        let start = day(2020, 12, 31);
        let end = day(2021, 12, 30);
        assert_eq!(difference_in_iso_week_years(end, start), 0);
        assert_eq!(difference_in_iso_week_years(start, end), 0);
        let a = day(2024, 1, 1);
        let b = day(2024, 12, 30);
        assert_eq!(difference_in_iso_week_years(a, b), -1);
    }

    #[test]
    fn stays_symmetric_at_chrono_limits() {
        let delta = difference_in_iso_week_years(NaiveDate::MAX, NaiveDate::MIN);
        assert!(delta > 0);
        assert_eq!(difference_in_iso_week_years(NaiveDate::MIN, NaiveDate::MAX), -delta);
    }
}
