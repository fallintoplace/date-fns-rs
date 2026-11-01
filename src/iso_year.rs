//! Strict four-digit ISO calendar years.

/// Parse a zero-padded four-digit year (0000 through 9999).
///
/// Rejects signs, whitespace, Unicode digits, and expanded years.
pub fn parse_iso_year(input: &str) -> Option<i32> {
    let bytes = input.as_bytes();
    if bytes.len() != 4 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    input.parse::<i32>().ok()
}

/// Format a calendar year using exactly four digits.
pub fn format_iso_year(year: i32) -> Option<String> {
    (0..=9999).contains(&year).then(|| format!("{year:04}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_zero_padded_years() {
        for year in [0, 1, 42, 2024, 9999] {
            let formatted = format_iso_year(year).unwrap();
            assert_eq!(parse_iso_year(&formatted), Some(year));
            assert_eq!(formatted.len(), 4);
        }
        assert_eq!(format_iso_year(3).as_deref(), Some("0003"));
    }

    #[test]
    fn rejects_noncanonical_and_out_of_range_years() {
        for bad in [
            "",
            "20",
            "20240",
            "+2024",
            "-001",
            "20a4",
            "２０２４",
            "2024 ",
        ] {
            assert_eq!(parse_iso_year(bad), None);
        }
        assert_eq!(format_iso_year(-1), None);
        assert_eq!(format_iso_year(10000), None);
        assert_eq!(format_iso_year(i32::MAX), None);
    }
}
