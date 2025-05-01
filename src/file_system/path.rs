use std::path::Path;

pub(crate) trait PathResolutions {
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
