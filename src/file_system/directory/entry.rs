#[cfg(target_os = "macos")]
use std::mem;

use libc::c_char;

pub(crate) type TypeCount = usize;

pub(super) trait UnixMetadata {
    fn r#type(&self) -> Type;
    fn is_dot_special(&self) -> bool;
    unsafe fn is_temporary(&self) -> bool;
    #[cfg(target_os = "macos")]
    unsafe fn metadata(&self) -> Result<libc::stat, ()>;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Type {
    File,
    Directory,
    Socket,
    Fifo,
    Block,
    Character,
    Symlink,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TypeCounts {
    pub(crate) total_files: TypeCount,
    pub(crate) total_directories: TypeCount,
    pub(crate) total_sockets: TypeCount,
    pub(crate) total_fifos: TypeCount,
    pub(crate) total_blocks: TypeCount,
    pub(crate) total_characters: TypeCount,
    pub(crate) total_symlinks: TypeCount,
    pub(crate) total_hiddens: TypeCount,
    pub(crate) total_temporaries: TypeCount,
}
