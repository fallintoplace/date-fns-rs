//! Finite ISO 8601 repeated START/PERIOD calendar-date intervals.

use std::iter::FusedIterator;

use chrono::NaiveDate;

use crate::IsoCalendarDuration;

/// Lazy dates defined by a bounded Rn/START/PERIOD expression.
///
/// Rn means n repetitions after the first occurrence. Calendar months are
/// applied successively, with day clamping at each occurrence.
#[derive(Clone, Debug)]
pub struct IsoRepeatedDateOccurrences {
    next: Option<NaiveDate>,
    period: IsoCalendarDuration,
    remaining: usize,
}

impl Iterator for IsoRepeatedDateOccurrences {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let day = self.next?;
        self.remaining -= 1;
        self.next = if self.remaining == 0 {
            None
        } else {
            crate::apply_iso_calendar_duration(day, self.period)
        };
        Some(day)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}

impl FusedIterator for IsoRepeatedDateOccurrences {}

/// Parse a finite ISO repeated interval such as R2/2024-01-31/P1M.
///
/// Unlimited repetitions, signed counts, zero periods, and negative periods
/// are rejected. Iteration stops safely if Chrono's date limit is reached.
pub fn parse_iso_repeating_date_interval(
    input: &str,
) -> Option<IsoRepeatedDateOccurrences> {
    let mut fields = input.split('/');
    let repetitions = fields.next()?.strip_prefix('R')?;
    let start = fields.next()?;
    let period = fields.next()?;
    if fields.next().is_some()
        || repetitions.is_empty()
        || !repetitions.as_bytes().iter().all(u8::is_ascii_digit)
        || (repetitions.len() > 1 && repetitions.starts_with('0'))
    {
        return None;
    }
    let repetitions = repetitions.parse::<usize>().ok()?;
    let remaining = repetitions.checked_add(1)?;
    let start = crate::parse_iso_date(start)?;
    let period = crate::parse_iso_calendar_duration(period)?;
    if period.negative
        || (period.years == 0
            && period.months == 0
            && period.weeks == 0
            && period.days == 0)
    {
        return None;
    }
    Some(IsoRepeatedDateOccurrences {
        next: Some(start),
        period,
        remaining,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn repeats_months_with_successive_clamping() {
        let dates: Vec<_> = parse_iso_repeating_date_interval("R2/2024-01-31/P1M")
            .unwrap()
            .collect();
        assert_eq!(
            dates,
            vec![date(2024, 1, 31), date(2024, 2, 29), date(2024, 3, 29)]
        );
        assert_eq!(
            parse_iso_repeating_date_interval("R0/2024-02-29/P1Y")
                .unwrap()
                .collect::<Vec<_>>(),
            vec![date(2024, 2, 29)]
        );
    }

    #[test]
    fn rejects_unbounded_recurrence_and_invalid_periods() {
        for invalid in [
            "R/2024-01-01/P1D",
            "R-1/2024-01-01/P1D",
            "R01/2024-01-01/P1D",
            "R1/2024-01-01/P0D",
            "R1/2024-01-01/-P1D",
            "R1/2024-02-30/P1D",
            "R1/2024-01-01/P1D/extra",
        ] {
            assert!(parse_iso_repeating_date_interval(invalid).is_none(), "{invalid}");
        }
    }
}
