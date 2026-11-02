//! European 30E/360 ISDA day count with final maturity handling.

use chrono::{Datelike, NaiveDate};

/// Count signed 30E/360 ISDA accrual days.
///
/// February month ends count as day 30, except that an end date which is the
/// final maturity date keeps its actual February day. Reversed endpoints
/// negate the forward count.
pub fn thirty_e_360_isda_day_count(start: NaiveDate, end: NaiveDate, end_is_maturity: bool) -> i64 {
    if start == end {
        return 0;
    }
    if start > end {
        return -thirty_e_360_isda_day_count(end, start, end_is_maturity);
    }
    let start_feb_end = start.month() == 2 && crate::end_of_month(start) == start;
    let end_feb_end = end.month() == 2 && crate::end_of_month(end) == end;
    let start_day = if start.day() == 31 || start_feb_end {
        30
    } else {
        start.day()
    };
    let end_day = if end.day() == 31 || (end_feb_end && !end_is_maturity) {
        30
    } else {
        end.day()
    };
    360 * i64::from(end.year() - start.year())
        + 30 * (i64::from(end.month()) - i64::from(start.month()))
        + i64::from(end_day)
        - i64::from(start_day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn handles_february_maturity_and_leap_day() {
        let start = date(2023, 2, 28);
        let end = date(2024, 2, 29);
        assert_eq!(thirty_e_360_isda_day_count(start, end, false), 360);
        assert_eq!(thirty_e_360_isda_day_count(start, end, true), 359);
        assert_eq!(thirty_e_360_isda_day_count(start, start, true), 0);
        assert_eq!(thirty_e_360_isda_day_count(end, end, true), 0);
        assert_eq!(
            thirty_e_360_isda_day_count(date(2024, 1, 31), end, false),
            30
        );
        assert_eq!(
            thirty_e_360_isda_day_count(date(2024, 1, 31), end, true),
            29
        );
    }

    #[test]
    fn preserves_sign_and_31st_adjustments() {
        let start = date(2024, 3, 31);
        let end = date(2024, 4, 30);
        assert_eq!(thirty_e_360_isda_day_count(start, end, false), 30);
        assert_eq!(thirty_e_360_isda_day_count(end, start, false), -30);
        assert_eq!(thirty_e_360_isda_day_count(start, start, false), 0);
    }
}
