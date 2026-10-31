//! Date-only helpers for Rust using the proleptic Gregorian calendar.
//!
//! The crate operates on [`NaiveDate`] and never silently chooses a time zone.
//! Arithmetic that could exceed Chrono's supported date range returns `None`.

#![forbid(unsafe_code)]

pub use chrono::{NaiveDate, Weekday};

mod arithmetic;
pub use arithmetic::{add_days, add_weeks, sub_days, sub_weeks};

mod months;
pub use months::{add_months, sub_months};

mod years;
pub use years::{add_years, sub_years};

mod boundaries;
pub use boundaries::{end_of_month, end_of_year, is_leap_year, start_of_month, start_of_year};

mod difference;
pub use difference::{
    difference_in_calendar_days, difference_in_calendar_months, difference_in_calendar_years,
};

mod weeks;
pub use weeks::{end_of_iso_week, end_of_week, start_of_iso_week, start_of_week};

mod compare;
pub use compare::{is_after, is_before, is_same_day, is_same_month, is_same_year, is_weekend};

mod interval;
pub use interval::{
    are_intervals_overlapping, each_day_of_interval, is_within_interval, DaysInInterval,
};

mod iso;
pub use iso::{format_iso_date, parse_iso_date};

mod metadata;
pub use metadata::{get_day_of_year, get_days_in_month, get_days_in_year};

mod quarters;
pub use quarters::{end_of_quarter, get_quarter, start_of_quarter};

mod quarter_arithmetic;
pub use quarter_arithmetic::{add_quarters, sub_quarters};

mod iso_weeks;
pub use iso_weeks::{get_iso_week, get_iso_week_year, get_iso_weeks_in_year};

mod business_days;
pub use business_days::{
    add_business_days, is_business_day, next_business_day, previous_business_day, sub_business_days,
};

mod business_difference;
pub use business_difference::difference_in_business_days;

mod interval_ops;
pub use interval_ops::{clamp_to_interval, intersection_of_intervals, interval_length_in_days};

mod interval_weeks;
pub use interval_weeks::{each_week_of_interval, WeeksInInterval};

mod interval_months;
pub use interval_months::{each_month_of_interval, MonthsInInterval};

mod interval_years;
pub use interval_years::{each_year_of_interval, YearsInInterval};

mod interval_quarters;
pub use interval_quarters::{each_quarter_of_interval, QuartersInInterval};

mod interval_business_days;
pub use interval_business_days::{each_business_day_of_interval, BusinessDaysInInterval};
