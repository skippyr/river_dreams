use chrono::{DateTime, Datelike as _, TimeZone, Timelike as _};

pub(crate) type Ordinal = u8;

pub(crate) trait DateTimeResolutions {
    fn day_ordinal(&self) -> &'static str;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DayFraction {
    Dawn,
    Morning,
    Afternoon,
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

fn is_ordinal(time: &DateTime<impl TimeZone>, ordinal: Ordinal) -> bool {
    (time.day() - ordinal as u32) % 10 == 0
}
