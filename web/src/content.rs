//! Pure, std-only content validation shared by `build.rs` and the unit tests.
//! Included from the build script via `#[path = "src/content.rs"]` and from the
//! app crate only under `cfg(test)`. Keep it dependency-free.

/// Slug = filename stem. Only ASCII lowercase, digits, hyphen. Non-empty.
pub fn validate_slug(slug: &str, name: &str) -> Result<(), String> {
    if slug.is_empty() {
        return Err(format!("{name}: slug is empty"));
    }
    for ch in slug.chars() {
        if !(ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-') {
            return Err(format!(
                "{name}: invalid slug character {:?} in {:?}; only ASCII lowercase, digits, hyphen",
                ch, slug
            ));
        }
    }
    Ok(())
}

/// Strict `YYYY-MM-DD`, real calendar date including leap years.
/// Returns `(year, month, day)`.
pub fn validate_date(date: &str, name: &str) -> Result<(u32, u32, u32), String> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3
        || parts[0].len() != 4
        || parts[1].len() != 2
        || parts[2].len() != 2
        || parts
            .iter()
            .any(|part| !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(format!("{name}: date {date:?} must be YYYY-MM-DD"));
    }
    let parse = |s: &str, what: &str| -> Result<u32, String> {
        s.parse::<u32>()
            .map_err(|_| format!("{name}: invalid {what} in date {date:?}"))
    };
    let y = parse(parts[0], "year")?;
    let m = parse(parts[1], "month")?;
    let d = parse(parts[2], "day")?;

    if y == 0 {
        return Err(format!("{name}: year must be positive in date {date:?}"));
    }
    if !(1..=12).contains(&m) {
        return Err(format!("{name}: month {m} out of range in date {date:?}"));
    }
    let max_day = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    };
    if d < 1 || d > max_day {
        return Err(format!(
            "{name}: day {d} out of range for {y}-{m:02} (max {max_day})"
        ));
    }
    Ok((y, m, d))
}

/// Convert a validated `YYYY-MM-DD` into RFC822 / RFC2822 UTC date.
/// e.g. `2026-11-12` -> `Thu, 12 Nov 2026 00:00:00 GMT`.
pub fn iso_to_rfc822(date: &str) -> String {
    let (y, m, d) = validate_date(date, "rss").expect("date already validated");
    let year = y as i64;
    let m = m as i64;
    let d = d as i64;

    // Howard Hinnant's days_from_civil: days since 1970-01-01 (a Thursday).
    // `y` is adjusted for the Jan/Feb era dance; `year` keeps the calendar year.
    let y = year - i64::from(m <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = m + if m > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;

    const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let wd = WEEKDAYS[(days + 4).rem_euclid(7) as usize];
    format!(
        "{wd}, {d:02} {} {year:04} 00:00:00 GMT",
        MONTHS[(m - 1) as usize]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_dates_and_leap_days() {
        for date in [
            "2026-11-12",
            "2024-02-29", // leap year
            "2000-02-29", // century leap year
            "1999-12-31",
            "2026-01-01",
        ] {
            assert!(
                validate_date(date, "x.md").is_ok(),
                "{date} should be valid"
            );
        }
    }

    #[test]
    fn rejects_invalid_and_non_leap_dates() {
        for date in [
            "2026-02-29", // not a leap year
            "1900-02-29", // century non-leap
            "2026-13-01",
            "2026-00-10",
            "2026-04-31",
            "2026-11-00",
            "2026-11-32",
            "2026-1-2",
            "0000-01-01",
            "+026-01-01",
            "2026-+1-01",
            "2026/11/12",
            "not-a-date",
            "",
        ] {
            assert!(
                validate_date(date, "x.md").is_err(),
                "{date} should be rejected"
            );
        }
    }

    #[test]
    fn rfc822_matches_known_dates() {
        assert_eq!(iso_to_rfc822("2026-11-12"), "Thu, 12 Nov 2026 00:00:00 GMT");
        assert_eq!(iso_to_rfc822("1970-01-01"), "Thu, 01 Jan 1970 00:00:00 GMT");
        assert_eq!(iso_to_rfc822("2024-02-29"), "Thu, 29 Feb 2024 00:00:00 GMT");
        assert_eq!(iso_to_rfc822("2000-01-01"), "Sat, 01 Jan 2000 00:00:00 GMT");
    }

    #[test]
    fn slug_rules() {
        assert!(validate_slug("2026-11-12-how-this-site-works", "a.md").is_ok());
        assert!(validate_slug("a1-b2", "a.md").is_ok());
        for bad in [
            "",
            "Upper",
            "under_score",
            "has space",
            "uni\u{4e2d}\u{6587}",
            "dot.name",
        ] {
            assert!(
                validate_slug(bad, "a.md").is_err(),
                "{bad:?} should be rejected"
            );
        }
    }
}
