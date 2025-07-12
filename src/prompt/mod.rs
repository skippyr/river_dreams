//! Provides features related to prompt rendering.

use std::io;

use anyhow::Result;

use crate::command_line::stdout_write;

pub(crate) mod left;
pub(crate) mod right;

/// Represents the terminal size unit.
type Size = u16;

/// The symbol used in ZSH to escape the percentage sign.
const ZSH_PERCENTAGE_SYMBOL: &str = "%%";
/// The symbol used in ZSH that gets replaced by the exit code of the last
/// command.
const ZSH_EXIT_CODE: &str = "%?";
/// The symbol used in ZSH that gets replaced by the total of background jobs.
const ZSH_JOBS_COUNT: &str = "%j";

/// Contains a subset of the available ANSI colors that can be used in ZSH.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    /// Refers to the dark red color (ANSI 1).
    Red,
    /// Refers to the dark green color (ANSI 2).
    Green,
    /// Refers to the dark yellow color (ANSI 3).
    Yellow,
    /// Refers to the dark blue color (ANSI 4).
    Blue,
    /// Refers to the dark magenta color (ANSI 5).
    Magenta,
    /// Refers to the dark cyan color (ANSI 6).
    Cyan,
}

impl Color {
    /// Gets the ANSI value correspondent to the color.
    ///
    /// # Returns
    /// The value.
    const fn ansi(&self) -> u8 {
        match self {
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
        }
    }
}

/// Writes the ZSH script that initiates the prompt to the terminal output stream.
///
/// This output should be redirected and appended to the `~/.zshrc`.
///
/// # Returns
/// An error on failure.
///
/// # Errors
/// It returns an error if it fails to write to the stream.
pub(crate) fn init() -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout_write!(
        &mut stdout,
        "setopt promptsubst;
export VIRTUAL_ENV_DISABLE_PROMPT=1;
PROMPT='$(river_dreams prompt left)';
RPROMPT='$(river_dreams prompt right)';"
    )
}

/// Allocates a string on the heap that wraps a symbol using the specified foreground color using
/// the ZSH syntax.
///
/// # Parameters
/// - `symbol`: the symbol to be wrapped.
/// - `color`: the foreground color to be used.
///
/// # Returns
/// The string allocated.
///
/// # Panics
/// It panics with a "memory allocation failed" message if the allocation fails.
fn color_symbol(symbol: impl AsRef<str>, color: Color) -> String {
    format!("%F{{{}}}{}%f", color.ansi(), symbol.as_ref())
}

/// Allocates a string on the heap that wraps a symbol to be shown only when the user is `root`
/// using the ZSH syntax.
///
/// # Parameters
/// - `symbol`: the symbol to be wrapped.
///
/// # Returns
/// The string allocated.
///
/// # Panics
/// It panics with a "memory allocation failed" message if the allocation fails.
fn show_symbol_when_root(symbol: impl AsRef<str>) -> String {
    format!("%(#.{}.)", symbol.as_ref())
}

/// Allocates a string on the heap that wraps two symbols to be shown for success and error exit
/// codes using the ZSH syntax.
///
/// # Parameters
/// - `on_success_symbol`: the wrapped symbol to be shown on success codes.
/// - `on_error_symbol`: the wrapped symbol to be show on error codes.
///
/// # Returns
/// The string allocated.
///
/// # Panics
/// It panics with a "memory allocation failed" message if the allocation fails.
fn show_symbols_for_exit_codes(
    on_success_symbol: impl AsRef<str>,
    on_error_symbol: impl AsRef<str>,
) -> String {
    format!(
        "%(?.{}.{})",
        on_success_symbol.as_ref(),
        on_error_symbol.as_ref()
    )
}

/// Allocates a string on the heap that wraps a symbol to be shown only when there are background
/// jobs running using the ZSH syntax.
///
/// # Parameters
/// - `symbol`: the symbol to be wrapped.
///
/// # Returns
/// The string allocated.
///
/// # Panics
/// It panics with a "memory allocation failed" message if the allocation fails.
fn show_symbol_when_job(symbol: impl AsRef<str>) -> String {
    format!("%(1j.{}.)", symbol.as_ref())
}
