//! Provides features to perform path resolutions.

use std::path::Path;

/// Provides members to resolve paths attributes.
pub(crate) trait PathResolutions {
    /// Checks whether the object refers to the file system root.
    ///
    /// # Returns
    /// A boolean that states that.
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
