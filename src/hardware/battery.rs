//! Provides features to get the battery metadata.

use anyhow::{Result, anyhow};
use battery::units::ratio::ratio;

/// Represents the battery charge percentage.
pub(crate) type ChargePercentage = u8;

/// Contains the possible statuses for the battery charge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChargeStatus {
    /// The charge is at a critical level (from 0% to 5%).
    Critical,
    /// The charge is at a low level (from 5% to 30%).
    Low,
    /// The charge is at a moderate level (from 30% to 60%).
    Moderate,
    /// The charge is at a high level (from 60% to 100%).
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

/// Represents the battery charge metadata.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Charge {
    /// The current charge percentage.
    pub(crate) percentage: ChargePercentage,
    /// A boolean that states the battery is being charged.
    pub(crate) is_charging: bool,
}

impl Charge {
    /// Gets the battery status that better describes its current charge.
    pub(crate) fn status(&self) -> ChargeStatus {
        ChargeStatus::from(self.percentage)
    }
}

/// Gets the charge metadata of the battery.
///
/// # Returns
/// The metadata if successful and a battery is a available. Otherwise, `None` or an error.
///
/// # Errors
/// It returns a generic displayable error if it fails.
pub(crate) fn charge() -> Result<Option<Charge>> {
    const SUPPLY_ERROR: &str = "can not retrieve info about the energy supply of the computer.";
    for battery in battery::Manager::new()
        .map_err(|_| anyhow!(SUPPLY_ERROR))?
        .batteries()
        .map_err(|_| anyhow!(SUPPLY_ERROR))?
    {
        // HACK: Currently it returns metadata for the first battery found in the loop. In the
        // future, consider adding a more reliable check to ensure it selects the correct one.
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
