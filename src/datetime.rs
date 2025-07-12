//! Provides features related to date time resolutions.

use chrono::{DateTime, Datelike as _, TimeZone, Timelike as _};

/// Represents a date ordinal.
pub(crate) type Ordinal = u8;

/// Provides members to resolve date time attributes.
pub(crate) trait DateTimeResolutions {
    /// Gets the ordinal that corresponds to the object: "st", "nd", "rd" or "th".
    ///
    /// # Returns
    /// The ordinal.
    fn day_ordinal(&self) -> &'static str;
    /// Gets the day fraction that corresponds to the object.
    ///
    /// # Returns
    /// The fraction.
    fn day_fraction(&self) -> DayFraction;
}

impl<T> DateTimeResolutions for DateTime<T>
where
    T: TimeZone,
{
    fn day_ordinal(&self) -> &'static str {
        if is_ordinal(self, 1) {
            "st"
        } else if is_ordinal(self, 2) {
            "nd"
        } else if is_ordinal(self, 3) {
            "rd"
        } else {
            "th"
        }
    }

    fn day_fraction(&self) -> DayFraction {
        DayFraction::from(self)
    }
}

/// Contains the possible fractions of the day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DayFraction {
    /// The dawn fraction (from 00h00m to 05h59m).
    Dawn,
    /// The morning fraction (from 06h00 to 11h59m).
    Morning,
    /// The afternoon fraction (from 12h00m to 17h59m).
    Afternoon,
    /// The night fraction (from 18h00m to 23h59m).
    Night,
}

impl<T> From<&DateTime<T>> for DayFraction
where
    T: TimeZone,
{
    fn from(time: &DateTime<T>) -> Self {
        match time.hour() {
            0..6 => Self::Dawn,
            6..12 => Self::Morning,
            12..18 => Self::Afternoon,
            _ => Self::Night,
        }
    }
}

/// Checks if a date time corresponds to a certain ordinal.
///
/// # Parameters
/// - `datetime`: the date time to be checked.
/// - `ordinal`: the ordinal to be compared to. It can be: 1 (refering to
/// first, "st"), 2 (refering to second, "nd") or 3 (refering to third, "rd").
///
/// # Returns
/// A boolean that states that.
fn is_ordinal(datetime: &DateTime<impl TimeZone>, ordinal: Ordinal) -> bool {
    (datetime.day() - ordinal as u32) % 10 == 0
}
