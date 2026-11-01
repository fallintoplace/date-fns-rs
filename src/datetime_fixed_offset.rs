//! Explicit fixed UTC offsets for naive and offset-aware date-times.

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};

/// Interpret a local naive date-time in the given UTC offset in seconds.
/// This constructs an instant and does not consult system timezone or DST rules.
pub fn assume_fixed_offset(
    local: NaiveDateTime,
    offset_seconds: i32,
) -> Option<DateTime<FixedOffset>> {
    FixedOffset::east_opt(offset_seconds)?
        .from_local_datetime(&local)
        .single()
}

/// Convert an existing instant to another fixed offset without changing the instant.
pub fn convert_fixed_offset(
    value: DateTime<FixedOffset>,
    offset_seconds: i32,
) -> Option<DateTime<FixedOffset>> {
    let offset = FixedOffset::east_opt(offset_seconds)?;
    Some(value.with_timezone(&offset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn keeps_the_same_instant_when_changing_offsets() {
        let local = NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap();
        let plus_two = assume_fixed_offset(local, 7_200).unwrap();
        let utc = convert_fixed_offset(plus_two, 0).unwrap();
        assert_eq!(
            utc.naive_local(),
            local
                .checked_sub_signed(chrono::TimeDelta::hours(2))
                .unwrap()
        );
        assert_eq!(utc.timestamp(), plus_two.timestamp());
        assert_eq!(plus_two.offset().local_minus_utc(), 7_200);
    }

    #[test]
    fn rejects_out_of_range_offsets_and_datetimes() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(assume_fixed_offset(local, 86_400), None);
        assert_eq!(assume_fixed_offset(local, -86_400), None);
        assert_eq!(
            convert_fixed_offset(assume_fixed_offset(local, 0).unwrap(), 86_400),
            None
        );
        assert_eq!(assume_fixed_offset(NaiveDateTime::MIN, 3_600), None);
    }
}