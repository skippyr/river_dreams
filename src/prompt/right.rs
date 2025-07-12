//! Provides features related to the right prompt rendering.

use std::io::{self, StdoutLock};

use anyhow::Result;
use num_format::{Locale, ToFormattedString as _};

use crate::command_line::stdout_write;
use crate::file_system::directory;
use crate::prompt::{self, Color};

/// Writes a single directory entry type count to the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `symbol`: the symbol to be used to represent the type.
/// - `color`: the color the symbol will have.
/// - `count`: the count of the type.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_entry_type_count(
    stdout: &mut StdoutLock,
    symbol: impl AsRef<str>,
    color: Option<Color>,
    count: directory::entry::TypeCount,
) -> Result<()> {
    if count == 0 {
        return Ok(());
    }
    stdout_write!(
        stdout,
        " {}{}",
        if let Some(color) = color {
            prompt::color_symbol(symbol.as_ref(), color)
        } else {
            String::from(symbol.as_ref())
        },
        count.to_formatted_string(&Locale::en)
    )
}

/// Writes the prompt section that shows the total of each entry type in the current directory to
/// the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `type_counts`: the count to be considered.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_entry_type_counts_section(
    stdout: &mut StdoutLock,
    type_counts: &directory::entry::TypeCounts,
) -> Result<()> {
    write_entry_type_count(
        stdout,
        " ",
        Some(Color::Yellow),
        type_counts.total_directories,
    )?;
    write_entry_type_count(stdout, " ", None, type_counts.total_files)?;
    write_entry_type_count(stdout, "󱄙 ", Some(Color::Cyan), type_counts.total_sockets)?;
    write_entry_type_count(stdout, "󰟦 ", Some(Color::Blue), type_counts.total_fifos)?;
    write_entry_type_count(stdout, "󰇖 ", Some(Color::Magenta), type_counts.total_blocks)?;
    write_entry_type_count(
        stdout,
        "󱣴 ",
        Some(Color::Green),
        type_counts.total_characters,
    )?;
    write_entry_type_count(stdout, "󰌷 ", Some(Color::Blue), type_counts.total_symlinks)?;
    write_entry_type_count(stdout, "󰈉 ", Some(Color::Red), type_counts.total_hiddens)?;
    write_entry_type_count(
        stdout,
        "󱣹 ",
        Some(Color::Magenta),
        type_counts.total_temporaries,
    )
}

/// Writes the prompt section that displays the total of jobs running in the background to the
/// terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the
fn write_jobs_section(stdout: &mut StdoutLock) -> Result<()> {
    stdout_write!(
        stdout,
        "{}",
        prompt::show_symbol_when_job(format!(
            " {} {}",
            prompt::color_symbol("", Color::Magenta),
            prompt::ZSH_JOBS_COUNT,
        ))
    )
}

/// Writes the right prompt to the terminal output stream.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
pub(crate) fn write() -> Result<()> {
    let mut stdout = io::stdout().lock();
    write_entry_type_counts_section(&mut stdout, &directory::current_entry_type_counts())?;
    write_jobs_section(&mut stdout)?;
    stdout_write!(&mut stdout, "\n")
}
