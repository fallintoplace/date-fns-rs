# date-fns-rs

A small Rust library for working with **calendar dates**, inspired by date-fns.

The API uses `chrono::NaiveDate` and the proleptic Gregorian calendar. It does not guess a time zone, silently wrap overflowing dates, or implement JavaScript `Date` semantics.

## Getting started

Add the library as a Git dependency while the crate is under development:

```toml
[dependencies]
date-fns-rs = { git = "https://github.com/fallintoplace/date-fns-rs" }
```

```rust
use date_fns_rs::NaiveDate;

let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
assert_eq!(date.to_string(), "2024-02-29");
```

## Scope

This project starts with date-only helpers for arithmetic, boundaries, comparisons, intervals, and parsing. Operations that can leave Chrono's supported date range return `Option` instead of panicking.

It is not yet a drop-in replacement for the JavaScript date-fns library. Time zones, locale-aware formatting, and JavaScript-style tokens are out of scope for the initial implementation.
