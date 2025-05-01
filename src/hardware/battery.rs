use anyhow::{Result, anyhow};
use battery::units::ratio::ratio;

pub(crate) type ChargePercentage = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChargeStatus {
    Critical,
    Low,
    Moderate,
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Charge {
    pub(crate) percentage: ChargePercentage,
    pub(crate) is_charging: bool,
}

impl Charge {
    pub(crate) fn status(&self) -> ChargeStatus {
        ChargeStatus::from(self.percentage)
    }
}

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
