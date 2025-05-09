use std::io;

use anyhow::{Result, anyhow};
use crossterm::style::Stylize as _;

use crate::command_line::stdout_write;

pub(crate) static APP_METADATA: AppMetadata = AppMetadata {
    name: env!("CARGO_PKG_NAME"),
    version: concat!("v", env!("CARGO_PKG_VERSION")),
    creation_year: 2023,
    repository_url: env!("CARGO_PKG_HOMEPAGE"),
    license: License {
        name: env!("CARGO_PKG_LICENSE"),
        text: include_str!("../LICENSE"),
    },
    developer: Developer {
        name: "Sherman Rofeman",
        email: Email("skippyr.developer@icloud.com"),
    },
};
#[cfg(target_os = "macos")]
pub(crate) static OS: &str = "macOS";
#[cfg(target_os = "linux")]
pub(crate) static OS: &str = "Linux";

pub(crate) type Year = u16;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AppMetadata {
    pub(crate) name: &'static str,
    pub(crate) version: &'static str,
    pub(crate) creation_year: Year,
    pub(crate) repository_url: &'static str,
    pub(crate) license: License,
    pub(crate) developer: Developer,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct License {
    pub(crate) name: &'static str,
    pub(crate) text: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Developer {
    pub(crate) name: &'static str,
    pub(crate) email: Email,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Email(pub(crate) &'static str);

impl Email {
    pub(crate) fn as_url(&self) -> String {
        format!("mailto:{}", self.0)
    }
}

pub(crate) fn write_software_help() -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout_write!(
        &mut stdout,
        "{}{}{} {} {} <{}> [{}]...
Performs actions related to the River Dreams theme.

{}
    {}  writes a prompt side using ZSH syntax.
    {}    dumps the ZSH script that initiates the prompt.

{} use {} or {} with each for their help instructions.

{}
    {}, {}        shows the software help instructions.
    {}, {}     shows the software version.
    {}, {}     shows the software license.
    {}, {}  opens the software repository.
    {}, {}       drafts an e-mail to the developer.
",
        ":".dark_yellow().bold(),
        "<>".dark_red().bold(),
        "::".dark_yellow().bold(),
        "Usage:".dark_magenta().bold(),
        APP_METADATA.name,
        "COMMAND".dark_yellow().underlined(),
        "OPTIONS".dark_cyan().underlined(),
        "❡ AVAILABLE COMMANDS".dark_magenta().bold(),
        "prompt".dark_yellow(),
        "init".dark_yellow(),
        " INFO:".dark_cyan().bold(),
        "-h".dark_cyan(),
        "--help".dark_cyan(),
        "❡ AVAILABLE OPTIONS".dark_magenta().bold(),
        "-h".dark_cyan(),
        "--help".dark_cyan(),
        "-v".dark_cyan(),
        "--version".dark_cyan(),
        "-l".dark_cyan(),
        "--license".dark_cyan(),
        "-g".dark_cyan(),
        "--repository".dark_cyan(),
        "-m".dark_cyan(),
        "--email".dark_cyan()
    )
}

pub(crate) fn write_prompt_command_help() -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout_write!(
        &mut stdout,
        "{}{}{} {} {} prompt <{}> [{}]...
Writes a prompt side using ZSH syntax.

Its outputs are used during initialization to write the prompt.

For more information, use:

    {} init -h;

{}
    {}, {}   writes the left prompt.
    {}, {}  writes the right prompt.

{}
    {}, {}  shows the command help instructions.
",
        ":".dark_yellow().bold(),
        "<>".dark_red().bold(),
        "::".dark_yellow().bold(),
        "Usage:".dark_magenta().bold(),
        APP_METADATA.name,
        "SIDE".dark_yellow().underlined(),
        "OPTIONS".dark_cyan().underlined(),
        APP_METADATA.name,
        "❡ AVAILABLE SIDES".dark_magenta().bold(),
        "l".dark_yellow(),
        "left".dark_yellow(),
        "r".dark_yellow(),
        "right".dark_yellow(),
        "❡ AVAILABLE OPTIONS".dark_magenta().bold(),
        "-h".dark_cyan(),
        "--help".dark_cyan()
    )
}

pub(crate) fn write_init_command_help() -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout_write!(
        &mut stdout,
        "{}{}{} {} {} init [{}]...
Dumps the ZSH script that initiates the prompt.

Its output is meant to be executed during the ZSH startup, by adding:

    eval $({} init);

to your ~/.zshrc configuration file.

{}
    {}, {}  shows the command help instructions.
",
        ":".dark_yellow().bold(),
        "<>".dark_red().bold(),
        "::".dark_yellow().bold(),
        "Usage:".dark_magenta().bold(),
        APP_METADATA.name,
        "OPTIONS".dark_cyan().underlined(),
        APP_METADATA.name,
        "❡ AVAILABLE OPTIONS".dark_magenta().bold(),
        "-h".dark_cyan(),
        "--help".dark_cyan()
    )
}

pub(crate) fn write_version() -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout_write!(
        &mut stdout,
        "{} {} ({})
Available at: {}.

Licensed under the {} license.
Copyright © {} {} <{}>.
",
        APP_METADATA.name.dark_magenta().bold(),
        APP_METADATA.version,
        OS,
        APP_METADATA.repository_url.dark_cyan().underlined(),
        APP_METADATA.license.name,
        APP_METADATA.creation_year,
        APP_METADATA.developer.name,
        APP_METADATA.developer.email.0.dark_cyan().underlined()
    )
}

pub(crate) fn write_license() -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout_write!(&mut stdout, "{}\n", APP_METADATA.license.text)
}

pub(crate) fn open_repository() -> Result<()> {
    let mut stdout = io::stdout().lock();
    open::that(APP_METADATA.repository_url)
        .map_err(|_| anyhow!("can not open the repository in the default web browser."))?;
    stdout_write!(
        &mut stdout,
        "Opening the repository in the default web browser.\n"
    )
}

pub(crate) fn draft_email_to_developer() -> Result<()> {
    let mut stdout = io::stdout().lock();
    open::that(APP_METADATA.developer.email.as_url())
        .map_err(|_| anyhow!("can not draft e-mail to developer in the default e-mail client."))?;
    stdout_write!(
        &mut stdout,
        "Drafting e-mail to developer in the default e-mail client.\n"
    )
}
