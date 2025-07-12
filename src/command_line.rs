//! Provides features to handle argument parsing and output.

/// Writes to the terminal output stream, propagating a possible write error.
///
/// # Parameters
/// - `$stdout`: the mutex lock of the stream.
/// - `$format`: the format to be used. It uses the same rules as the `println!` macro.
/// - `$arguments`: the list of arguments to be formatted.
///
/// # Errors
/// It returns a displayable error if it cannot write to the stream.
macro_rules! stdout_write {
    ($stdout:expr, $format:literal $(, $arguments:expr)* $(,)?) => {
        {
            use anyhow::anyhow;
            use std::io::{StdoutLock, Write as _};

            fn __assert_stdout_lock<'a, 'b>(stdout: &'a mut StdoutLock<'b>) -> &'a mut StdoutLock<'b> {
                stdout
            }

            write!(__assert_stdout_lock($stdout), $format $(, $arguments)*).map_err(|_| anyhow!("cannot write to the standard output stream."))
        }
    };
}

pub(crate) use stdout_write;

/// Provides members to parse command-line arguments.
pub(crate) trait ArgumentParsing {
    /// Checks if the object refers to a command-line option, e.g: `-h` (short version) or `--help`
    /// (long version).
    ///
    /// # Returns
    /// A boolean that states that.
    fn is_option(&self) -> bool;
}

impl<T> ArgumentParsing for T
where
    T: AsRef<str>,
{
    fn is_option(&self) -> bool {
        self.as_ref().starts_with("-")
    }
}

/// Contains the command-line commands the application can parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Command {
    /// Writes a prompt side to the terminal output stream. It requires an argument that specifies
    /// the side to write, either left (specified with values "left" or "l") or right (specified
    /// with values "right" or "r").
    Prompt,
    /// Writes the ZSH script that initiates the prompt to the terminal output stream.
    Init,
}

impl Command {
    /// Gets the name of the command.
    ///
    /// # Returns
    /// The name.
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Prompt => "prompt",
        }
    }
}
