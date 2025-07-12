//! Provides features related to the left prompt rendering.

use std::env;
use std::io::{self, StdoutLock};
use std::net::IpAddr;
use std::path::Path;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Local};
use crossterm::terminal;

use crate::command_line::stdout_write;
use crate::file_system::directory;
use crate::file_system::path::PathResolutions as _;
use crate::hardware::{battery, disk};
use crate::prompt::{self, Color, ZSH_EXIT_CODE, ZSH_PERCENTAGE_SYMBOL};
use crate::datetime::{DateTimeResolutions as _, DayFraction};
use crate::{format, git};

/// The length of the prompt that is composed by constant section areas.
const SECTIONS_CONSTANT_LENGTH: prompt::Size = 42;

/// Writes the prompt separator composed by the first tribal symbol set seen at its top to the
/// terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `terminal_width`: the terminal width to be considered.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns a displayable error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_top_separator(stdout: &mut StdoutLock, terminal_width: prompt::Size) -> Result<()> {
    for column in 0..terminal_width {
        stdout_write!(
            stdout,
            "{}",
            if column % 2 == 0 {
                prompt::color_symbol("≥", Color::Yellow)
            } else {
                prompt::color_symbol("v", Color::Red)
            }
        )?;
    }
    stdout_write!(stdout, "{}", prompt::color_symbol(":«(", Color::Yellow))
}

/// Writes the prompt section that shows the primary local IP address of the machine to the terminal
/// output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `ip`: the possible IP to be considered. If `None`, the function will not do anything.
/// - `sections_length`: a reference to the current prompt length. It gets incremented by this
/// section length upon a complete execution.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns a displayable error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_local_ip_section(
    stdout: &mut StdoutLock,
    ip: Option<IpAddr>,
    sections_length: &mut prompt::Size,
) -> Result<()> {
    let ip = ip
        .map(|ip| ip.to_string())
        .unwrap_or(String::from("No Address Found"));
    stdout_write!(stdout, "{} {}", prompt::color_symbol(" ", Color::Blue), ip)?;
    *sections_length += ip.len() as prompt::Size;
    Ok(())
}

/// Writes the prompt section that shows the disk usage and its status to the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `usage`: the usage to be considered.
/// - `sections_length`: a reference to the current prompt length. It gets incremented by this
/// section length upon a complete execution.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_disk_section(
    stdout: &mut StdoutLock,
    usage: disk::Usage,
    sections_length: &mut prompt::Size,
) -> Result<()> {
    stdout_write!(
        stdout,
        "  {}{}{}",
        prompt::color_symbol(
            "󰋊 ",
            match usage.status() {
                disk::UsageStatus::Low => Color::Green,
                disk::UsageStatus::Moderate => Color::Yellow,
                disk::UsageStatus::High => Color::Red,
            }
        ),
        usage.0,
        ZSH_PERCENTAGE_SYMBOL
    )?;
    *sections_length += format::number_length(usage.0)? as prompt::Size;
    Ok(())
}

/// Writes the prompt section that shows the battery charge and its status to the terminal output
/// stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `charge`: the possible charge to be considered. If `None`, the function will not do anything.
/// - `sections_length`: a reference to the current prompt length. It gets incremented by this
/// section length upon a complete execution.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_battery_section(
    stdout: &mut StdoutLock,
    charge: Option<battery::Charge>,
    sections_length: &mut prompt::Size,
) -> Result<()> {
    let charge = match charge {
        Some(charge) => charge,
        None => return Ok(()),
    };
    stdout_write!(
        stdout,
        "  {} {}{}",
        match charge.status() {
            battery::ChargeStatus::Critical =>
                prompt::color_symbol(if charge.is_charging { "󰢟" } else { "󰂎" }, Color::Red),
            battery::ChargeStatus::Low =>
                prompt::color_symbol(if charge.is_charging { "󱊤" } else { "󱊡" }, Color::Red),
            battery::ChargeStatus::Moderate =>
                prompt::color_symbol(if charge.is_charging { "󱊥" } else { "󱊢" }, Color::Yellow),
            battery::ChargeStatus::High =>
                prompt::color_symbol(if charge.is_charging { "󱊦" } else { "󱊣" }, Color::Green),
        },
        charge.percentage,
        ZSH_PERCENTAGE_SYMBOL
    )?;
    *sections_length += format::number_length(charge.percentage)? as prompt::Size + 5;
    Ok(())
}

/// Writes the prompt section that shows a calendar with the weekday, month and day of month to the
/// terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `current_date_time`: the current time to be considered.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_calendar_section(stdout: &mut StdoutLock, current_date_time: DateTime<Local>) -> Result<()> {
    stdout_write!(
        stdout,
        "  {}{}{}",
        prompt::color_symbol("󰃭 ", Color::Red),
        current_date_time.format("(%a) %b %d"),
        current_date_time.day_ordinal(),
    )
}

/// Writes the prompt section that shows a 24-hours clock with the hours and minutes to the terminal
/// output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `current_date_time`: the current time to be considered.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_clock_section(stdout: &mut StdoutLock, current_date_time: DateTime<Local>) -> Result<()> {
    stdout_write!(
        stdout,
        "  {}{}",
        match current_date_time.day_fraction() {
            DayFraction::Dawn => prompt::color_symbol("󰭎 ", Color::Cyan),
            DayFraction::Morning => prompt::color_symbol("󰖨 ", Color::Red),
            DayFraction::Afternoon => prompt::color_symbol(" ", Color::Blue),
            DayFraction::Night => prompt::color_symbol("󰽥 ", Color::Yellow),
        },
        current_date_time.format("%Hh%Mm")
    )
}

/// Writes the prompt separator used to fill the space between the second and third section lines
/// composed by the second tribal symbol set seen at its top to the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `terminal_width`: the terminal width to be considered.
/// - `sections_length`: a reference to the current prompt length.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_middle_separator(
    stdout: &mut StdoutLock,
    terminal_width: prompt::Size,
    sections_length: prompt::Size,
) -> Result<()> {
    stdout_write!(stdout, "{}", prompt::color_symbol(")»:", Color::Yellow))?;
    for column in 0..terminal_width.saturating_sub(sections_length) {
        stdout_write!(
            stdout,
            "{}",
            if column % 2 == 0 {
                prompt::color_symbol("-", Color::Red)
            } else {
                prompt::color_symbol("=", Color::Yellow)
            }
        )?;
    }
    Ok(())
}

/// Writes the prompt section that shows a decorator when user is `root` to the terminal output
/// stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_user_permissions_section(stdout: &mut StdoutLock) -> Result<()> {
    stdout_write!(
        stdout,
        "{}",
        prompt::show_symbol_when_root(format!(
            "{}{}{}",
            prompt::color_symbol("{", Color::Yellow),
            prompt::color_symbol("#", Color::Red),
            prompt::color_symbol("}", Color::Yellow)
        ))
    )
}

/// Writes the prompt section that shows different decorators for success and error exit codes to
/// the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_exit_code_section(stdout: &mut StdoutLock) -> Result<()> {
    stdout_write!(
        stdout,
        "{}{}{}",
        prompt::color_symbol("{", Color::Yellow),
        prompt::show_symbols_for_exit_codes(
            prompt::color_symbol(ZSH_EXIT_CODE, Color::Yellow),
            prompt::color_symbol(ZSH_EXIT_CODE, Color::Red)
        ),
        prompt::color_symbol("}⤐ ", Color::Yellow)
    )
}

/// Writes the prompt section that shows the active Python virtual environment name to the terminal
/// output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_virtual_env_section(stdout: &mut StdoutLock) -> Result<()> {
    let virtual_env = match env::var("VIRTUAL_ENV") {
        Ok(variable) => variable,
        Err(_) => return Ok(()),
    };
    stdout_write!(
        stdout,
        " ({})",
        match Path::new(&virtual_env).file_name() {
            Some(file_name) => file_name.to_string_lossy(),
            None => return Ok(()),
        }
    )
}

/// Writes the prompt section that shows the current directory path to the terminal output stream.
/// It gets abbreviated inside of Git repositories and uses the `~` (for the home directory) and `@`
/// (for Git repository directories) prefixes.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `current_directory`: the current directory path to be considered.
/// - `repository`: the possible Git repository to be considered.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_path_section(
    stdout: &mut StdoutLock,
    current_directory: impl AsRef<Path>,
    repository: Option<&git::Repository>,
) -> Result<()> {
    stdout_write!(stdout, " ")?;
    let repository = match repository {
        Some(repository) if !repository.path.is_root() => repository,
        _ => return stdout_write!(stdout, "{}", prompt::color_symbol("%~", Color::Red)),
    };
    stdout_write!(
        stdout,
        "{}",
        prompt::color_symbol(
            format!(
                "@/{}",
                current_directory
                    .as_ref()
                    .strip_prefix(repository.path.parent().ok_or_else(|| anyhow!(
                        r#"bad use of malformed repository path "{}"."#,
                        repository.path.display()
                    ))?)
                    .map_err(|_| anyhow!(
                        r#"can not strip prefix of repository path "{}"."#,
                        repository.path.display()
                    ))?
                    .display()
            ),
            Color::Red
        )
    )
}

/// Writes the prompt section that shows the active branch name and a decorator when it is dirty to
/// the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
/// - `repository`: the Git repository to be considered.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_git_section(stdout: &mut StdoutLock, repository: Option<&git::Repository>) -> Result<()> {
    let repository = match repository {
        Some(repository) => repository,
        None => return Ok(()),
    };
    stdout_write!(stdout, "{}", prompt::color_symbol(":«(", Color::Yellow))?;
    if let git::Reference::RebaseHash(_) = repository.reference {
        stdout_write!(
            stdout,
            "{}:",
            prompt::color_symbol("@rebase", Color::Magenta)
        )?;
    }
    stdout_write!(
        stdout,
        "{}{}",
        match &repository.reference {
            git::Reference::Branch(branch) => branch,
            git::Reference::RebaseHash(hash) => hash,
        },
        prompt::color_symbol(")»", Color::Yellow)
    )?;
    if repository.is_dirty {
        stdout_write!(stdout, " {}", prompt::color_symbol("✗", Color::Cyan))?;
    }
    Ok(())
}

/// Writes the prompt section that shows a decorator when the user does not owns the current
/// repository, that is, it does not have write permissions, to the terminal output stream.
///
/// # Parameters
/// - `stdout`: the mutex lock of the stream.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to write to the stream.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
fn write_directory_ownership_section(stdout: &mut StdoutLock) -> Result<()> {
    if !directory::owns_current() {
        stdout_write!(stdout, " {}", prompt::color_symbol("", Color::Cyan))?;
    }
    Ok(())
}

/// Writes the left prompt to the terminal output stream.
///
/// # Returns
/// A possible error.
///
/// # Errors
/// It returns an empty error if it fails to:
/// - Write to the stream.
/// - Retrieve the terminal window dimensions.
/// - Retrieve the battery charge metadata.
/// - Retrieve the current directory path.
///
/// # Panics
/// It panics with a "memory allocation failed" message if any string allocation fails.
pub(crate) fn write() -> Result<()> {
    let terminal_width = terminal::size()
        .map(|(width, _)| width)
        .map_err(|_| anyhow!("can not retrieve the terminal dimensions."))?;
    let disk_usage = disk::usage()?;
    let battery_charge = battery::charge()?;
    let current_date_time = Local::now();
    let git_repository = git::find_repository();
    let current_directory = directory::current()?;
    let mut sections_length = SECTIONS_CONSTANT_LENGTH;
    let mut stdout = io::stdout().lock();
    write_top_separator(&mut stdout, terminal_width)?;
    write_local_ip_section(
        &mut stdout,
        local_ip_address::local_ip().ok(),
        &mut sections_length,
    )?;
    write_disk_section(&mut stdout, disk_usage, &mut sections_length)?;
    write_battery_section(&mut stdout, battery_charge, &mut sections_length)?;
    write_calendar_section(&mut stdout, current_date_time)?;
    write_clock_section(&mut stdout, current_date_time)?;
    write_middle_separator(&mut stdout, terminal_width, sections_length)?;
    write_user_permissions_section(&mut stdout)?;
    write_exit_code_section(&mut stdout)?;
    write_virtual_env_section(&mut stdout)?;
    write_path_section(&mut stdout, current_directory, git_repository.as_ref())?;
    write_git_section(&mut stdout, git_repository.as_ref())?;
    write_directory_ownership_section(&mut stdout)?;
    stdout_write!(&mut stdout, " \n")
}
