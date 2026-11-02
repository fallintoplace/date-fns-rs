//! Financial day-count conventions for naive calendar dates.
//!
//! Day-count conventions are calendar arithmetic, not time-zone calculations.
//! All year fractions are computed from the corresponding integral day count.

use chrono::{Datelike, NaiveDate};

/// A financial convention for accrual day counts and year fractions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DayCountConvention {
    /// Actual elapsed calendar days with a fixed 360-day denominator.
    Actual360,
    /// Actual elapsed calendar days with a fixed 365-day denominator.
    Actual365Fixed,
    /// US 30/360 with start-of-period February end-of-month adjustments.
    ///
    /// A start on the last day of February counts as day 30. If both dates
    /// are the last day of February, the end counts as day 30 too. End-of-month
    /// day 31 counts as day 30 when the adjusted start day is at least 30.
    Thirty360US,
    /// European 30E/360. Both 31st days are treated as day 30.
    ///
    /// Unlike US 30/360, February endpoints are not specially adjusted.
    ThirtyE360,
    /// European 30E+/360. The 31st at the end of a period is day 1 of
    /// the following 30-day month; 31st starts are treated as day 30.
    ThirtyEPlus360,
}

/// Compute a signed calendar day count under a financial convention.
///
/// Reversed endpoints return the negative forward count. This makes every
/// convention antisymmetric, including 30/360 endpoint adjustments. Inputs
/// may cover the full representable Chrono date range without overflow.
pub fn financial_day_count(
    start: NaiveDate,
    end: NaiveDate,
    convention: DayCountConvention,
) -> i64 {
    if start == end {
        return 0;
    }
    if start > end {
        return -forward_day_count(end, start, convention);
    }
    forward_day_count(start, end, convention)
}

fn is_last_day_of_february(date: NaiveDate) -> bool {
    date.month() == 2 && crate::end_of_month(date) == date
}

fn forward_day_count(start: NaiveDate, end: NaiveDate, convention: DayCountConvention) -> i64 {
    use DayCountConvention::{Actual360, Actual365Fixed, Thirty360US, ThirtyE360, ThirtyEPlus360};

    match convention {
        Actual360 | Actual365Fixed => end.signed_duration_since(start).num_days(),
        ThirtyEPlus360 => {
            360 * (i64::from(end.year()) - i64::from(start.year()))
                + 30 * (i64::from(end.month()) - i64::from(start.month()))
                + i64::from(end.day())
                - i64::from(start.day().min(30))
        }
        Thirty360US | ThirtyE360 => {
            let mut start_day = start.day();
            let mut end_day = end.day();
            if convention == ThirtyE360 {
                start_day = start_day.min(30);
                end_day = end_day.min(30);
            } else {
                let start_last_february = is_last_day_of_february(start);
                let end_last_february = is_last_day_of_february(end);
                if start_last_february {
                    start_day = 30;
                    if end_last_february {
                        end_day = 30;
                    }
                }
                if start_day == 31 {
                    start_day = 30;
                }
                if end_day == 31 && start_day >= 30 {
                    end_day = 30;
                }
            }
            360 * (i64::from(end.year()) - i64::from(start.year()))
                + 30 * (i64::from(end.month()) - i64::from(start.month()))
                + i64::from(end_day)
                - i64::from(start_day)
        }
    }
}

/// Compute a signed year fraction using a fixed day-count denominator.
///
/// The denominator is 365 for Actual/365 Fixed and 360 otherwise.
/// This intentionally does not implement Actual/Actual conventions, whose
/// denominator depends on a specific market convention or coupon schedule.
pub fn financial_year_fraction(
    start: NaiveDate,
    end: NaiveDate,
    convention: DayCountConvention,
) -> f64 {
    let denominator = if convention == DayCountConvention::Actual365Fixed {
        365.0
    } else {
        360.0
    };
    financial_day_count(start, end, convention) as f64 / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn counts_actual_days_across_leap_day_and_epoch() {
        let first = date(2024, 2, 28);
        let last = date(2024, 3, 1);
        assert_eq!(
            financial_day_count(first, last, DayCountConvention::Actual360),
            2
        );
        assert_eq!(
            financial_day_count(first, last, DayCountConvention::Actual365Fixed),
            2
        );
        assert_eq!(
            financial_year_fraction(first, last, DayCountConvention::Actual360),
            2.0 / 360.0
        );
        assert_eq!(
            financial_year_fraction(first, last, DayCountConvention::Actual365Fixed),
            2.0 / 365.0
        );
    }

    #[test]
    fn european_30e360_handles_both_31sts_without_february_adjustment() {
        let convention = DayCountConvention::ThirtyE360;
        assert_eq!(
            financial_day_count(date(2024, 1, 31), date(2024, 2, 29), convention),
            29
        );
        assert_eq!(
            financial_day_count(date(2024, 2, 29), date(2024, 3, 31), convention),
            31
        );
        assert_eq!(
            financial_day_count(date(2024, 3, 31), date(2024, 4, 30), convention),
            30
        );
        assert_eq!(
            financial_day_count(date(2024, 1, 31), date(2025, 1, 31), convention),
            360
        );
    }

    #[test]
    fn us_30360_adjusts_last_february_days_and_month_ends() {
        let convention = DayCountConvention::Thirty360US;
        assert_eq!(
            financial_day_count(date(2024, 2, 29), date(2024, 3, 31), convention),
            30
        );
        assert_eq!(
            financial_day_count(date(2023, 2, 28), date(2024, 2, 29), convention),
            360
        );
        assert_eq!(
            financial_day_count(date(2024, 1, 31), date(2024, 2, 29), convention),
            29
        );
        assert_eq!(
            financial_day_count(date(2024, 1, 30), date(2024, 1, 31), convention),
            0
        );
        assert_eq!(
            financial_day_count(date(2024, 1, 29), date(2024, 1, 31), convention),
            2
        );
        assert_eq!(
            financial_day_count(date(2024, 3, 31), date(2024, 4, 30), convention),
            30
        );
    }

    #[test]
    fn every_convention_is_antisymmetric_and_zero_on_equal_dates() {
        let conventions = [
            DayCountConvention::Actual360,
            DayCountConvention::Actual365Fixed,
            DayCountConvention::Thirty360US,
            DayCountConvention::ThirtyE360,
            DayCountConvention::ThirtyEPlus360,
        ];
        let samples = [
            date(2023, 2, 28),
            date(2024, 2, 29),
            date(2024, 3, 30),
            date(2024, 3, 31),
            date(2025, 1, 31),
        ];
        for convention in conventions {
            for left in samples {
                assert_eq!(financial_day_count(left, left, convention), 0);
                for right in samples {
                    assert_eq!(
                        financial_day_count(left, right, convention),
                        -financial_day_count(right, left, convention)
                    );
                }
            }
        }
    }

    #[test]
    fn financial_day_counts_support_chrono_min_and_max() {
        let actual = financial_day_count(
            NaiveDate::MIN,
            NaiveDate::MAX,
            DayCountConvention::Actual365Fixed,
        );
        assert!(actual > 0);
        let thirty = financial_day_count(
            NaiveDate::MIN,
            NaiveDate::MAX,
            DayCountConvention::ThirtyE360,
        );
        assert!(thirty > 0);
    }
}


#[cfg(test)]
mod thirty_e_plus_tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn carries_end_of_month_31_into_the_next_thirty_day_month() {
        let kind = DayCountConvention::ThirtyEPlus360;
        assert_eq!(financial_day_count(date(2024, 1, 30), date(2024, 1, 31), kind), 1);
        assert_eq!(financial_day_count(date(2024, 1, 31), date(2024, 2, 28), kind), 28);
        assert_eq!(financial_day_count(date(2024, 1, 31), date(2024, 3, 31), kind), 61);
        assert_eq!(financial_day_count(date(2024, 12, 31), date(2025, 1, 31), kind), 31);
        assert_eq!(financial_day_count(date(2024, 1, 31), date(2024, 1, 31), kind), 0);
        assert_eq!(financial_day_count(date(2024, 1, 31), date(2024, 1, 30), kind), -1);
    }

    #[test]
    fn uses_a_360_day_year_and_does_not_change_other_conventions() {
        let start = date(2024, 1, 30);
        let end = date(2024, 1, 31);
        assert_eq!(financial_year_fraction(start, end, DayCountConvention::ThirtyEPlus360), 1.0 / 360.0);
        assert_eq!(financial_day_count(start, end, DayCountConvention::ThirtyE360), 0);
    }
}
