//! Contains functionalities to query metadata about the main disk.

use std::mem;

use anyhow::{Result, bail};
use libc::{c_char, c_ulong};

/// Represents the disk usage percentage.
pub(crate) type UsagePercentage = u8;

/// Contains the possible statuses for the disk usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UsageStatus {
    /// The usage is low, allowing you to work fine.
    Low,
    /// The usage is moderate, and clearing up some stuff is recommended.
    Moderate,
    /// The usage is high, and the computer will have issues allocating files.
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

/// Represents the disk usage metadata, currently just a wrapper for its usage percentage.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Usage(pub(crate) UsagePercentage);

impl Usage {
    /// Gets the disk status that describes its current usage.
    pub(crate) fn status(&self) -> UsageStatus {
        UsageStatus::from(self.0)
    }
}

/// Gets the current disk usage metadata of the main disk.
///
/// # Returns
/// On success, the disk usage metadata.
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
