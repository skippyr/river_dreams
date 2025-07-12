//! Provides features to perform error handling.

use std::io::{self, Write as _};

use anyhow::Error;
use crossterm::style::Stylize as _;

use crate::metadata;

/// Writes the message of a displayable error to the terminal error stream.
///
/// # Parameters
/// - `error`: the error to be considered.
///
/// # Remarks
/// - If a write error gets thrown, it simply gets ignored.
/// - Ideally, the application execution should be terminated from the `main` fn right after its
/// call if the error cannot be handled, allowing proper resources deallocation.
pub(crate) fn write(error: Error) {
    let mut stderr = io::stderr().lock();
    _ = writeln!(
        stderr,
        "{}{}{} {} {}{} {}
{} use {} or {} for help instructions.",
        ":".dark_yellow().bold(),
        "<>".dark_red().bold(),
        "::".dark_yellow().bold(),
        metadata::APP_METADATA.name.dark_magenta().bold(),
        "(exit 1)".dark_yellow().bold(),
        ":".dark_magenta().bold(),
        error,
        " INFO:".dark_cyan().bold(),
        "-h".dark_cyan(),
        "--help".dark_cyan()
    );
}
