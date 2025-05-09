use std::mem;

use anyhow::{Result, bail};
use libc::{c_char, c_ulong};

pub(crate) type UsagePercentage = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UsageStatus {
    Low,
    Moderate,
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Usage(pub(crate) UsagePercentage);

impl Usage {
    pub(crate) fn status(&self) -> UsageStatus {
        UsageStatus::from(self.0)
    }
}

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
