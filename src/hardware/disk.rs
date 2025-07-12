//! Provides features to get the disk metadata.

use std::mem;

use anyhow::{Result, bail};
use libc::{c_char, c_ulong};

/// Represents the disk usage percentage.
pub(crate) type UsagePercentage = u8;

/// Contains the possible statuses for the disk usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UsageStatus {
    /// The usage is low (0% to 60%).
    Low,
    /// The usage is moderate (60% to 80%).
    Moderate,
    /// The usage is high (80% to 100%).
    High,
}

impl From<UsagePercentage> for UsageStatus {
    fn from(percentage: UsagePercentage) -> Self {
        match percentage {
            0..60 => Self::Low,
            60..80 => Self::Moderate,
            _ => Self::High,
        }
    }
}

/// Represents the disk usage metadata.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Usage(
    /// The disk usage percentage.
    pub(crate) UsagePercentage
);

impl Usage {
    /// Gets the disk status that better describes its current usage.
    pub(crate) fn status(&self) -> UsageStatus {
        UsageStatus::from(self.0)
    }
}

/// Gets the usage metadata of the disk.
///
/// # Returns
/// The metadata or an error.
///
/// # Errors
/// It returns a generic displayable error on failure.
pub(crate) fn usage() -> Result<Usage> {
    let mut metadata = unsafe { mem::zeroed() };
    if unsafe { libc::statvfs([b'/' as c_char, 0].as_ptr(), &mut metadata) } < 0 {
        bail!("can not retrieve the disk information.");
    }
    let total_bytes = metadata.f_frsize * metadata.f_blocks as c_ulong;
    let available_bytes = metadata.f_frsize * metadata.f_bavail as c_ulong;
    let used_bytes = total_bytes - available_bytes;
    Ok(Usage(
        (used_bytes as f64 / total_bytes as f64 * 100.0) as UsagePercentage,
    ))
}
