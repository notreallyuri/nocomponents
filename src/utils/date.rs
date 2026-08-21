//! A civil date, and the arithmetic a calendar needs.
//!
//! Deliberately not a date-time library: no clock, no zones, no parsing beyond ISO 8601, and a
//! whole 12 bytes. A UI library should not hand its consumers a `chrono` version to agree with,
//! and a calendar grid needs exactly four things — what day of the week a date falls on, how many
//! days a month has, and how to step by days and by months.
//!
//! The conversions to and from a day count are Howard Hinnant's `days_from_civil` /
//! `civil_from_days`, which are exact for any year in `i32` and have no branches worth worrying
//! about. Everything else is built on them, so stepping over a month end or a leap day is never a
//! special case.

/// Month names, and the abbreviations for a weekday header. English only: rendering these through
/// `Intl` would drag in a locale API for two arrays.
pub const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Indexed by [`Date::weekday`], so Sunday first.
pub const WEEKDAY_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// What a weekday column is labelled, indexed like [`WEEKDAY_NAMES`].
pub const WEEKDAY_ABBREVIATIONS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// A date with no time and no zone: the thing a calendar grid is made of.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Date {
    pub year: i32,
    /// 1–12.
    pub month: u32,
    /// 1–31.
    pub day: u32,
}

impl Date {
    pub const fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Today, as the browser's local clock has it. `1970-01-01` where there is no clock at all,
    /// which only happens outside a browser.
    pub fn today() -> Self {
        let now = js_sys::Date::new_0();
        Self {
            year: now.get_full_year() as i32,
            // `getMonth` is zero-based; nothing else here is.
            month: now.get_month() + 1,
            day: now.get_date(),
        }
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 0,
        }
    }

    /// Days since 1970-01-01, negative before it.
    pub fn to_days(self) -> i64 {
        let year = if self.month <= 2 {
            self.year - 1
        } else {
            self.year
        };
        let era = if year >= 0 { year } else { year - 399 } as i64 / 400;
        let year_of_era = year as i64 - era * 400;
        let month = if self.month > 2 {
            self.month - 3
        } else {
            self.month + 9
        } as i64;
        let day_of_year = (153 * month + 2) / 5 + self.day as i64 - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

        era * 146097 + day_of_era - 719468
    }

    pub fn from_days(days: i64) -> Self {
        let days = days + 719468;
        let era = if days >= 0 { days } else { days - 146096 } / 146097;
        let day_of_era = days - era * 146097;
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146096) / 365;
        let year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_prime = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
        let month = if month_prime < 10 {
            month_prime + 3
        } else {
            month_prime - 9
        };

        Self {
            year: (if month <= 2 { year + 1 } else { year }) as i32,
            month: month as u32,
            day: day as u32,
        }
    }

    /// 0 is Sunday, matching [`WEEKDAY_NAMES`].
    pub fn weekday(self) -> u32 {
        (self.to_days() + 4).rem_euclid(7) as u32
    }

    pub fn add_days(self, days: i64) -> Self {
        Self::from_days(self.to_days() + days)
    }

    /// Steps whole months, clamping the day to the end of the month it lands in — 31 January plus
    /// a month is 28 February, not 3 March.
    pub fn add_months(self, months: i32) -> Self {
        let total = self.year as i64 * 12 + (self.month as i64 - 1) + months as i64;
        let year = total.div_euclid(12) as i32;
        let month = total.rem_euclid(12) as u32 + 1;

        Self {
            year,
            month,
            day: self.day.min(Self::days_in_month(year, month)),
        }
    }

    /// The first of this date's month, which is what a calendar grid is anchored on.
    pub fn first_of_month(self) -> Self {
        Self { day: 1, ..self }
    }

    pub fn is_same_month(self, other: Self) -> bool {
        self.year == other.year && self.month == other.month
    }

    /// `2026-08-21`. What a form field or a URL should carry.
    pub fn to_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Parses `YYYY-MM-DD`, rejecting anything that is not a real date.
    pub fn parse_iso(text: &str) -> Option<Self> {
        let mut parts = text.split('-');
        let year: i32 = parts.next()?.parse().ok()?;
        let month: u32 = parts.next()?.parse().ok()?;
        let day: u32 = parts.next()?.parse().ok()?;
        if parts.next().is_some() || !(1..=12).contains(&month) {
            return None;
        }
        if day < 1 || day > Self::days_in_month(year, month) {
            return None;
        }

        Some(Self { year, month, day })
    }

    /// "August 2026", for a month caption.
    pub fn month_caption(self) -> String {
        format!(
            "{} {}",
            MONTH_NAMES[(self.month as usize - 1).min(11)],
            self.year
        )
    }

    /// "21 August 2026", for a button that has to say which date it is holding.
    pub fn day_month_year(self) -> String {
        format!(
            "{} {} {}",
            self.day,
            MONTH_NAMES[(self.month as usize - 1).min(11)],
            self.year
        )
    }

    /// "Friday, 21 August 2026", for a day cell's accessible name.
    pub fn long_form(self) -> String {
        format!(
            "{}, {} {} {}",
            WEEKDAY_NAMES[self.weekday() as usize],
            self.day,
            MONTH_NAMES[(self.month as usize - 1).min(11)],
            self.year
        )
    }
}

/// The 42 days a six-week grid shows for `month`, starting on `week_starts_on` (0 = Sunday).
///
/// Always six weeks, never four or five: a grid that changes height as you page through the year
/// makes everything below it jump.
pub fn month_grid(month: Date, week_starts_on: u32) -> Vec<Date> {
    let first = month.first_of_month();
    let offset = (first.weekday() + 7 - week_starts_on % 7) % 7;
    let start = first.add_days(-(offset as i64));

    (0..42).map(|day| start.add_days(day)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_the_day_count() {
        for date in [
            Date::new(1970, 1, 1),
            Date::new(2000, 2, 29),
            Date::new(2026, 8, 21),
            Date::new(1899, 12, 31),
            Date::new(2400, 6, 15),
        ] {
            assert_eq!(Date::from_days(date.to_days()), date);
        }
    }

    #[test]
    fn knows_the_epoch_was_a_thursday() {
        assert_eq!(Date::new(1970, 1, 1).weekday(), 4);
        assert_eq!(Date::new(2026, 8, 21).weekday(), 5);
    }

    #[test]
    fn steps_over_month_and_year_ends() {
        assert_eq!(Date::new(2026, 12, 31).add_days(1), Date::new(2027, 1, 1));
        assert_eq!(Date::new(2024, 2, 28).add_days(1), Date::new(2024, 2, 29));
        assert_eq!(Date::new(2023, 2, 28).add_days(1), Date::new(2023, 3, 1));
    }

    #[test]
    fn clamps_a_month_step_to_the_month_it_lands_in() {
        assert_eq!(Date::new(2026, 1, 31).add_months(1), Date::new(2026, 2, 28));
        assert_eq!(Date::new(2024, 1, 31).add_months(1), Date::new(2024, 2, 29));
        assert_eq!(
            Date::new(2026, 1, 15).add_months(-1),
            Date::new(2025, 12, 15)
        );
        assert_eq!(
            Date::new(2026, 3, 15).add_months(-14),
            Date::new(2025, 1, 15)
        );
    }

    #[test]
    fn parses_only_real_dates() {
        assert_eq!(Date::parse_iso("2026-08-21"), Some(Date::new(2026, 8, 21)));
        assert_eq!(Date::parse_iso("2026-02-29"), None);
        assert_eq!(Date::parse_iso("2024-02-29"), Some(Date::new(2024, 2, 29)));
        assert_eq!(Date::parse_iso("2026-13-01"), None);
        assert_eq!(Date::parse_iso("2026-08"), None);
        assert_eq!(Date::parse_iso("2026-08-21-01"), None);
    }

    #[test]
    fn lays_out_six_weeks_from_the_right_weekday() {
        let grid = month_grid(Date::new(2026, 8, 1), 0);
        assert_eq!(grid.len(), 42);
        // 1 August 2026 is a Saturday, so a Sunday-first grid opens on 26 July.
        assert_eq!(grid[0], Date::new(2026, 7, 26));
        assert_eq!(grid[0].weekday(), 0);
        assert_eq!(grid[41], Date::new(2026, 9, 5));

        // Monday-first moves the same month on by a day.
        let grid = month_grid(Date::new(2026, 8, 1), 1);
        assert_eq!(grid[0], Date::new(2026, 7, 27));
        assert_eq!(grid[0].weekday(), 1);
    }
}
