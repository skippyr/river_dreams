//! Provides features to query directory entries metadata.

#[cfg(target_os = "macos")]
use std::mem;

use libc::c_char;

/// Represents the count of a single directory entry type.
pub(crate) type TypeCount = usize;

/// Provides members to query directory entry metadata.
pub(super) trait UnixMetadata {
    /// Gets the entry type of the object.
    ///
    /// # Returns
    /// The entry type.
    fn r#type(&self) -> Type;
    /// Checks whether the object is a special dot entry, either `.` (refering to the current
    /// directory) or `..` (refering to the parent directory).
    ///
    /// # Returns
    /// A boolean that states that.
    ///
    /// # Panics
    /// It tries to reads the first bytes of the entry name conditionally to determinate that. If
    /// the name contains 0 bytes, on debug builds, it will panic with an "out of bounds" message,
    /// however this should never happen if the information came from the system.
    fn is_dot_special(&self) -> bool;
    /// Checks whether the object is a temporary entry, that is, it ends with `~`.
    ///
    /// # Returns
    /// A boolean that states that.
    ///
    /// # Safety
    /// It is considered unsafe, because it needs to iterate the bytes of the entry name until it
    /// finds a null termination character to determinate its length, which can cause a segmentation
    /// fault if it does not contains one.
    unsafe fn is_temporary(&self) -> bool;
    #[cfg(target_os = "macos")]
    /// Gets the POSIX entry metadata of the object.
    ///
    /// # Returns
    /// The metadata or an error.
    ///
    /// # Errors
    /// It returns an empty error if it fails.
    ///
    /// # Unsafe
    /// It is considered unsafe, because it calls low-level system APIs code that needs to iterate
    /// over the entry name bytes until a null termination character is read, which can cause a
    /// segmentation fault if it does not have one. However, this should not be possible if the
    /// information came from the system.
    ///
    /// # Availability
    /// It is only available on macOS.
    unsafe fn metadata(&self) -> Result<libc::stat, ()>;
    /// Checks whether the object is a hidden entry, that is it starts with `.` or, exclusively on
    /// macOS, it is marked with the `UF_HIDDEN` attribute.
    ///
    /// # Returns
    /// A boolean that states that or an error.
    ///
    /// # Errors
    /// It returns an empty error on macOS if querying for the entry metadata is required and fails
    /// for any reason.
    ///
    /// # Unsafe
    /// It is considered unsafe, because it calls low-level system APIs code that needs to iterate
    /// over the entry name bytes until a null termination character is read, which can cause a
    /// segmentation fault if it does not have one. However, this should not be possible if the
    /// information came from the system.
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
    /// Refers to a domain socket.
    Socket,
    /// Refers to a named pipe (FIFO — first-in, first-out).
    Fifo,
    /// Refers to a block device.
    Block,
    /// Refers to a character device.
    Character,
    /// Refers to a symbolic link.
    Symlink,
}

/// Represents the count of all entry types inside of a directory.
#[derive(Debug, Clone, Default)]
pub(crate) struct TypeCounts {
    /// The total of regular files.
    pub(crate) total_files: TypeCount,
    /// The total of directories.
    pub(crate) total_directories: TypeCount,
    /// The total of domain sockets.
    pub(crate) total_sockets: TypeCount,
    /// The total of named pipes (FIFO — first-in, first-out).
    pub(crate) total_fifos: TypeCount,
    /// The total of block devices.
    pub(crate) total_blocks: TypeCount,
    /// The total of character devices.
    pub(crate) total_characters: TypeCount,
    /// The total of symbolic links.
    pub(crate) total_symlinks: TypeCount,
    /// The total of hidden entries.
    pub(crate) total_hiddens: TypeCount,
    /// The total of temporary entries.
    pub(crate) total_temporaries: TypeCount,
}
