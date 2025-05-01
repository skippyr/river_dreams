use crate::{
    command_line::stdout_write,
    file_system::directory,
    prompt::{self, Color},
};
use anyhow::Result;
use num_format::{Locale, ToFormattedString as _};
use std::io::{self, StdoutLock};

fn write_entry_type_count<T>(
    stdout: &mut StdoutLock,
    symbol: T,
    color: Option<Color>,
    count: directory::entry::TypeCount,
) -> Result<()>
where
    T: AsRef<str>,
{
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

pub(crate) fn write() -> Result<()> {
    let mut stdout = io::stdout().lock();
    write_entry_type_counts_section(&mut stdout, &directory::current_entry_type_counts())?;
    write_jobs_section(&mut stdout)?;
    stdout_write!(&mut stdout, "\n")
}
