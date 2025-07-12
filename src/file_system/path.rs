//! Contains functionalities related to path resolutions.

use std::path::Path;

/// A trait applied to types to add path funcionalities.
pub(crate) trait PathResolutions {
    /// Checks whether the type refers to the file system root.
    ///
    /// # Returns
    /// A boolean that states the type refers to the file system root.
    fn is_root(&self) -> bool;
}

impl<T> PathResolutions for T
where
    T: AsRef<Path>,
{
    fn is_root(&self) -> bool {
        self.as_ref().ancestors().count() == 1
    }
}
