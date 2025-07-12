//! Contains functionalities to query metadata about the main battery.

use anyhow::{Result, anyhow};
use battery::units::ratio::ratio;

/// Represents the battery charge percentage.
pub(crate) type ChargePercentage = u8;

/// Contains the possible statuses for the battery charge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChargeStatus {
    /// The charge is at a critical level, and the computer will sleep soon.
    Critical,
    /// The charge is at a low level, and should be charged.
    Low,
    /// The charge is at a moderate level, not requiring charging.
    Moderate,
    /// The charge is at its full capacity, and can be unplugged.
    High,
}

impl From<ChargePercentage> for ChargeStatus {
    fn from(percentage: ChargePercentage) -> Self {
        match percentage {
            0..5 => Self::Critical,
            5..30 => Self::Low,
            30..60 => Self::Moderate,
            _ => Self::High,
        }
    }
}

/// Contains the battery charge metadata.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Charge {
    /// The current charge percentage.
    pub(crate) percentage: ChargePercentage,
    /// A boolean that states the battery is being charged.
    pub(crate) is_charging: bool,
}

impl Charge {
    /// Gets the battery status that describes its current charge.
    pub(crate) fn status(&self) -> ChargeStatus {
        ChargeStatus::from(self.percentage)
    }
}

/// Gets the current charge metadata of the main battery.
///
/// # Returns
/// On success, an Option that may contain the current charge metadata. None is returned if no
/// battery is available.
///
/// # Errors
/// It returns a generic displayable error, if querying the battery metadata fails.
pub(crate) fn charge() -> Result<Option<Charge>> {
    const SUPPLY_ERROR: &str = "can not retrieve info about the energy supply of the computer.";
    for battery in battery::Manager::new()
        .map_err(|_| anyhow!(SUPPLY_ERROR))?
        .batteries()
        .map_err(|_| anyhow!(SUPPLY_ERROR))?
    {
        let battery = match battery {
            Ok(battery) => battery,
            Err(_) => continue,
        };
        return Ok(Some(Charge {
            percentage: (battery.state_of_charge().get::<ratio>() * 100.0).round()
                as ChargePercentage,
            is_charging: matches!(
                battery.state(),
                battery::State::Charging | battery::State::Full | battery::State::Unknown
            ),
        }));
    }
    Ok(None)
}
