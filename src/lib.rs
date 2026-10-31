//! Date-only helpers for Rust using the proleptic Gregorian calendar.
//!
//! The crate operates on [`NaiveDate`] and never silently chooses a time zone.
//! Arithmetic that could exceed Chrono's supported date range returns `None`.

#![forbid(unsafe_code)]

pub use chrono::{NaiveDate, Weekday};
