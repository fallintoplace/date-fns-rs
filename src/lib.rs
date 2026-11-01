//! Date-only helpers for Rust using the proleptic Gregorian calendar.
//!
//! The crate operates on [`NaiveDate`] and never silently chooses a time zone.
//! Arithmetic that could exceed Chrono's supported date range returns `None`.

#![forbid(unsafe_code)]

pub use chrono::{NaiveDate, Weekday};

mod arithmetic;
pub use arithmetic::{add_days, add_weeks, sub_days, sub_weeks};

mod epoch_days;
pub use epoch_days::{from_epoch_day, to_epoch_day};

mod interval_midpoint;
pub use interval_midpoint::interval_midpoint;

mod months;
pub use months::add_months_preserving_end_of_month;
pub use months::{add_months, sub_months};

mod half_year_arithmetic;
pub use half_year_arithmetic::{add_half_years, sub_half_years};

mod years;
pub use years::add_years_preserving_end_of_month;
pub use years::{add_years, sub_years};

mod boundaries;
pub use boundaries::{end_of_month, end_of_year, is_leap_year, start_of_month, start_of_year};

mod calendar_month_grid;
pub use calendar_month_grid::{calendar_month_grid, get_calendar_weeks_in_month};

mod decades;
pub use decades::add_decades;
pub use decades::sub_decades;
pub use decades::{end_of_decade, start_of_decade};

mod difference;
pub use difference::difference_in_calendar_decades;
pub use difference::{
    difference_in_calendar_days, difference_in_calendar_months, difference_in_calendar_years,
};

mod calendar_week_difference;
pub use calendar_week_difference::difference_in_calendar_iso_week_years;
pub use calendar_week_difference::difference_in_calendar_weeks;

mod weeks;
pub use weeks::{end_of_iso_week, end_of_week, start_of_iso_week, start_of_week};

mod week_comparisons;
pub use week_comparisons::{is_same_iso_week, is_same_week};

mod weekday_arithmetic;
pub use weekday_arithmetic::nearest_weekday;
pub use weekday_arithmetic::{next_weekday, previous_weekday};

mod compare;
pub use compare::max_date;
pub use compare::min_date;
pub use compare::{is_after, is_before, is_same_day, is_same_month, is_same_year, is_weekend};

mod interval;
pub use interval::{
    are_intervals_overlapping, each_day_of_interval, is_within_interval, DaysInInterval,
};

mod interval_adjacency;
pub use interval_adjacency::are_intervals_adjacent;

mod stepped_days;
pub use stepped_days::{each_nth_day_of_interval, SteppedDaysInInterval};

mod iso;
pub use iso::{format_iso_date, parse_iso_date};

mod iso_year;
pub use iso_year::{format_iso_year, parse_iso_year};

mod iso_week_date;
pub use iso_week_date::{format_iso_week_date, parse_iso_week_date};

mod iso_date_interval;
pub use iso_date_interval::{format_iso_date_interval, parse_iso_date_interval};

mod iso_basic_date;
pub use iso_basic_date::{format_iso_basic_date, parse_iso_basic_date};

mod metadata;
pub use metadata::{get_day_of_year, get_days_in_month, get_days_in_year};

mod half_year_day_ordinals;
pub use half_year_day_ordinals::{get_day_of_half_year, get_days_in_half_year};

mod iso_ordinal_date;
pub use iso_ordinal_date::{format_iso_ordinal_date, parse_iso_ordinal_date};

mod iso_month_day;
pub use iso_month_day::{format_iso_month_day, parse_iso_month_day};

mod calendar_quarter_difference;
pub use calendar_quarter_difference::difference_in_calendar_quarters;

mod calendar_half_year_difference;
pub use calendar_half_year_difference::difference_in_calendar_half_years;

mod quarters;
pub use quarters::get_day_of_quarter;
pub use quarters::get_days_in_quarter;
pub use quarters::{end_of_quarter, get_quarter, start_of_quarter};

mod half_years;
pub use half_years::{end_of_half_year, get_half_year, start_of_half_year};

mod fiscal_quarters;
pub use fiscal_quarters::difference_in_calendar_fiscal_quarters;
pub use fiscal_quarters::{end_of_fiscal_quarter, get_fiscal_quarter, start_of_fiscal_quarter};

mod fiscal_years;
pub use fiscal_years::{end_of_fiscal_year, get_fiscal_year, start_of_fiscal_year};

mod calendar_predicates;
pub use calendar_predicates::is_first_day_of_year;
pub use calendar_predicates::is_last_day_of_year;
pub use calendar_predicates::{
    is_first_day_of_month, is_last_day_of_month, is_leap_day, is_same_quarter,
};

mod half_year_predicates;
pub use half_year_predicates::{
    is_first_day_of_half_year, is_last_day_of_half_year, is_same_half_year,
};

mod quarter_arithmetic;
pub use quarter_arithmetic::{add_quarters, sub_quarters};

mod iso_weeks;
pub use iso_weeks::get_iso_day_of_year;
pub use iso_weeks::{get_iso_week, get_iso_week_year, get_iso_weeks_in_year};

mod iso_week_year_boundaries;
pub use iso_week_year_boundaries::{end_of_iso_week_year, start_of_iso_week_year};

mod business_days;
pub use business_days::nearest_business_day;
pub use business_days::{
    add_business_days, is_business_day, next_business_day, previous_business_day, sub_business_days,
};

mod business_holidays;
pub use business_holidays::{add_business_days_with_holidays, sub_business_days_with_holidays};

mod business_difference;
pub use business_difference::difference_in_business_days;

mod business_year_metadata;
pub use business_year_metadata::{get_business_day_of_year, get_business_days_in_year};

mod closest_date;
pub use closest_date::{closest_date_index_to, closest_date_to};

mod interval_ops;
pub use interval_ops::is_interval_within_interval;
pub use interval_ops::{clamp_to_interval, intersection_of_intervals, interval_length_in_days};

mod interval_gap;
pub use interval_gap::gap_between_intervals;

mod interval_subtract;
pub use interval_subtract::subtract_intervals;

mod interval_merge;
pub use interval_merge::merge_intervals;

mod interval_weeks;
pub use interval_weeks::{each_week_of_interval, WeeksInInterval};

mod interval_week_segments;
pub use interval_week_segments::{each_week_segment_of_interval, WeekSegmentsInInterval};

mod interval_months;
pub use interval_months::{each_month_of_interval, MonthsInInterval};

mod interval_month_segments;
pub use interval_month_segments::{each_month_segment_of_interval, MonthSegmentsInInterval};

mod interval_years;
pub use interval_years::{each_year_of_interval, YearsInInterval};

mod interval_year_segments;
pub use interval_year_segments::{each_year_segment_of_interval, YearSegmentsInInterval};

mod interval_quarters;
pub use interval_quarters::{each_quarter_of_interval, QuartersInInterval};

mod interval_quarter_segments;
pub use interval_quarter_segments::{each_quarter_segment_of_interval, QuarterSegmentsInInterval};

mod interval_weekdays;
pub use interval_weekdays::{each_weekday_of_interval, WeekdaysInInterval};

mod interval_business_days;
pub use interval_business_days::{each_business_day_of_interval, BusinessDaysInInterval};

mod business_month_boundaries;
pub use business_month_boundaries::{first_business_day_of_month, last_business_day_of_month};

mod interval_weekends;
pub use interval_weekends::{each_weekend_of_interval, WeekendsInInterval};

mod nth_weekday;
pub use nth_weekday::nth_weekday_of_month;

mod weekday_month_ordinals;
pub use weekday_month_ordinals::{
    get_weekday_occurrence_from_end_of_month, get_weekday_occurrence_in_month,
};

mod week_of_month;
pub use week_of_month::get_week_of_month;

mod week_of_quarter;
pub use week_of_quarter::get_week_of_quarter;

mod calendar_week_metadata;
pub use calendar_week_metadata::{get_week, get_week_year};

mod calendar_week_year_bounds;
pub use calendar_week_year_bounds::{end_of_week_year, get_weeks_in_week_year, start_of_week_year};

mod full_week_difference;
pub use full_week_difference::difference_in_weeks;

mod full_month_difference;
pub use full_month_difference::difference_in_months;

mod full_quarter_difference;
pub use full_quarter_difference::difference_in_quarters;

mod iso_week_setters;
pub use iso_week_setters::set_iso_day_of_week;
pub use iso_week_setters::{set_iso_week, set_iso_week_year};

mod full_year_difference;
pub use full_year_difference::difference_in_years;

mod set_month;
pub use set_month::set_month;

mod set_year;
pub use set_year::set_year;

mod set_day_of_year;
pub use set_day_of_year::set_day_of_year;

mod iso_basic_ordinal_date;
pub use iso_basic_ordinal_date::{format_iso_basic_ordinal_date, parse_iso_basic_ordinal_date};

mod iso_basic_week_date;
pub use iso_basic_week_date::{format_iso_basic_week_date, parse_iso_basic_week_date};

mod iso_year_month;
pub use iso_year_month::{format_iso_year_month, parse_iso_year_month};

mod date_distance_en;
pub use date_distance_en::{format_date_distance_en, format_date_distance_strict_days_en};

mod centuries;
pub use centuries::{end_of_century, start_of_century};

mod set_quarter;
pub use set_quarter::set_quarter;

mod weekday_occurrence_count;
pub use weekday_occurrence_count::count_weekday_occurrences;

mod interval_day_counts;
pub use interval_day_counts::{count_business_days_in_interval, count_weekend_days_in_interval};

mod calendar_week_setters;
pub use calendar_week_setters::{set_week, set_week_year};

mod monthly_weekday_occurrences;
pub use monthly_weekday_occurrences::{
    each_nth_weekday_of_month_in_interval, MonthlyWeekdayOccurrences,
};

mod holiday_business_difference;
pub use holiday_business_difference::difference_in_business_days_excluding_holidays;

mod half_year_intervals;
pub use half_year_intervals::{each_half_year_of_interval, HalfYearsInInterval};

mod fiscal_month_number;
pub use fiscal_month_number::get_fiscal_month;

mod fiscal_half_years;
pub use fiscal_half_years::{
    end_of_fiscal_half_year, get_fiscal_half_year, start_of_fiscal_half_year,
};

mod business_month_metadata;
pub use business_month_metadata::{get_business_day_of_month, get_business_days_in_month};

mod interval_split;
pub use interval_split::split_interval_after;
