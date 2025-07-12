//! Provides features to check directory permissions and retrieve entries metadata.

pub(crate) mod entry;

use std::env;
use std::path::PathBuf;

use anyhow::{Result, anyhow};

use self::entry::UnixMetadata as _;

/// Gets the current directory path.
///
/// # Returns
/// The current directory path or an error.
///
/// # Errors
/// It returns a generic displayable error if the path cannot be resolved.
pub(crate) fn current() -> Result<PathBuf> {
    env::current_dir().or_else(|_| {
        env::var("PWD")
            .ok()
            .map(PathBuf::from)
            .filter(|path| path != &PathBuf::from(".") && path != &PathBuf::from(".."))
            .ok_or_else(|| anyhow!("can not resolve the current directory"))
    })
}

/// Checks if the user owns the current directory, this is, has write permissions.
///
/// # Returns
/// A boolean that states that.
pub(crate) fn owns_current() -> bool {
    unsafe { libc::access(c".".as_ptr(), libc::W_OK) == 0 }
}

/// Gets a count containing the total of each entry type in the current directory.
///
/// If the directory cannot be opened, it returns a struct containing all fields assigned to zero.
///
/// # Returns
/// The count.
pub(crate) fn current_entry_type_counts() -> entry::TypeCounts {
    let mut type_counts = entry::TypeCounts::default();
    let stream = match unsafe { libc::opendir(c".".as_ptr()).as_ref() } {
        Some(stream) => stream as *const libc::DIR as *mut libc::DIR,
        None => return type_counts,
    };
    loop {
        let entry = match unsafe { libc::readdir(stream).as_ref() } {
            Some(entry) if !entry.is_dot_special() => entry,
            None => break,
            _ => continue,
        };
        if unsafe { entry.is_temporary() } {
            type_counts.total_temporaries += 1;
        }
        match unsafe { entry.is_hidden() } {
            Ok(is_hidden) if is_hidden => type_counts.total_hiddens += 1,
            Err(_) => continue,
            _ => (),
        }
        match entry.r#type() {
            entry::Type::File => type_counts.total_files += 1,
            entry::Type::Directory => type_counts.total_directories += 1,
            entry::Type::Socket => type_counts.total_sockets += 1,
            entry::Type::Fifo => type_counts.total_fifos += 1,
            entry::Type::Block => type_counts.total_blocks += 1,
            entry::Type::Character => type_counts.total_characters += 1,
            entry::Type::Symlink => type_counts.total_symlinks += 1,
        }
    }
    unsafe { libc::closedir(stream) };
    type_counts
}
