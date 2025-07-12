//! Contains funcionalities to retrieve directory entries metadata.

#[cfg(target_os = "macos")]
use std::mem;

use libc::c_char;

/// Represents the count of a single directory entry type.
pub(crate) type TypeCount = usize;

/// A trait applied to types to add functionalities to query directory entry metadata.
pub(super) trait UnixMetadata {
    /// Gets the entry type of the object.
    ///
    /// # Returns
    /// The entry type of the object.
    fn r#type(&self) -> Type;
    /// Checks whether the object is a special dot entry, aka. `.` (current directory) or `..`
    /// (parent directory).
    ///
    /// # Returns
    /// A boolean that states the object is a special dot entry.
    fn is_dot_special(&self) -> bool;
    /// Checks whether the object is a temporary entry.
    ///
    /// # Returns
    /// A boolean that states the object is a temporary entry.
    unsafe fn is_temporary(&self) -> bool;
    #[cfg(target_os = "macos")]
    /// Gets the POSIX entry metadata of the object.
    ///
    /// # Returns
    /// On success, the POSIX entry metadata.
    ///
    /// # Errors
    /// It returns an empty error on failure.
    ///
    /// # Availability
    /// It is only available on macOS.
    unsafe fn metadata(&self) -> Result<libc::stat, ()>;
    /// Checks whether the object is a hidden entry.
    ///
    /// # Remarks
    /// On Linux, it only checks if the file name starts with `.`.
    ///
    /// On macOS, it performs the previous check but also looks for the `UF_HIDDEN` attribute.
    unsafe fn is_hidden(&self) -> Result<bool, ()>;
}

impl UnixMetadata for libc::dirent {
    fn r#type(&self) -> Type {
        match self.d_type {
            libc::DT_DIR => Type::Directory,
            libc::DT_SOCK => Type::Socket,
            libc::DT_FIFO => Type::Fifo,
            libc::DT_BLK => Type::Block,
            libc::DT_CHR => Type::Character,
            libc::DT_LNK => Type::Symlink,
            _ => Type::File,
        }
    }

    fn is_dot_special(&self) -> bool {
        (self.d_name[0] == b'.' as c_char && self.d_name[1] == 0)
            || (self.d_name[0] == b'.' as c_char
                && self.d_name[1] == b'.' as c_char
                && self.d_name[2] == 0)
    }

    unsafe fn is_temporary(&self) -> bool {
        self.d_name[unsafe { libc::strlen(self.d_name.as_ptr()) - 1 }] == b'~' as c_char
    }

    #[cfg(target_os = "macos")]
    unsafe fn metadata(&self) -> Result<libc::stat, ()> {
        let mut metadata = unsafe { mem::zeroed() };
        unsafe { libc::lstat(self.d_name.as_ptr(), &mut metadata) == 0 }
            .then_some(metadata)
            .ok_or(())
    }

    unsafe fn is_hidden(&self) -> Result<bool, ()> {
        if self.d_name[0] == b'.' as c_char {
            return Ok(true);
        }
        #[cfg(target_os = "macos")]
        {
            unsafe { self.metadata() }.map(|stat| stat.st_flags & libc::UF_HIDDEN != 0)
        }
        #[cfg(target_os = "linux")]
        {
            Ok(false)
        }
    }
}

/// Contains the possible entry types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Type {
    /// Refers to a regular file.
    File,
    /// Refers to a directory.
    Directory,
    /// Refers to a socket.
    Socket,
    /// Refers to a fifo.
    Fifo,
    /// Refers to a block device.
    Block,
    /// Refers to a character device.
    Character,
    /// Refers to a symbolic link.
    Symlink,
}

/// Represents a count of each entry type inside of a directory.
#[derive(Debug, Clone, Default)]
pub(crate) struct TypeCounts {
    /// The total of regular files.
    pub(crate) total_files: TypeCount,
    /// The total of directories.
    pub(crate) total_directories: TypeCount,
    /// The total of sockets.
    pub(crate) total_sockets: TypeCount,
    /// The total of fifos.
    pub(crate) total_fifos: TypeCount,
    /// The total of blocks.
    pub(crate) total_blocks: TypeCount,
    /// The total of characters.
    pub(crate) total_characters: TypeCount,
    /// The total of symlinks.
    pub(crate) total_symlinks: TypeCount,
    /// The total of hidden entries.
    pub(crate) total_hiddens: TypeCount,
    /// The total of temporary entries.
    pub(crate) total_temporaries: TypeCount,
}
