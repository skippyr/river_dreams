#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

pub(crate) mod command_line;
pub(crate) mod error;
pub(crate) mod file_system;
pub(crate) mod format;
pub(crate) mod git;
pub(crate) mod hardware;
pub(crate) mod metadata;
pub(crate) mod prompt;
pub(crate) mod time;

use crate::command_line::{ArgumentParsing as _, Command};
use anyhow::anyhow;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let mut command = None;
    if let Some(command_argument) = arguments.first() {
        if command_argument == Command::Init.name() {
            command = Some(Command::Init);
        } else if command_argument == Command::Prompt.name() {
            command = Some(Command::Prompt);
        } else if !command_argument.is_option() {
            error::write(anyhow!(r#"invalid command "{command_argument}" provided."#));
            return ExitCode::FAILURE;
        }
    }
    for argument in &arguments {
        if argument == "-h" || argument == "--help" {
            return match match command {
                None => metadata::write_software_help(),
                Some(Command::Prompt) => metadata::write_prompt_command_help(),
                Some(Command::Init) => metadata::write_init_command_help(),
            } {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    error::write(error);
                    ExitCode::FAILURE
                }
            };
        }
        if command.is_none() {
            if let Some(action) = match argument.as_str() {
                "-v" | "--version" => Some(metadata::write_version()),
                "-g" | "--repository" => Some(metadata::open_repository()),
                "-m" | "--email" => Some(metadata::draft_email_to_developer()),
                "-l" | "--license" => Some(metadata::write_license()),
                _ => None,
            } {
                return match action {
                    Ok(_) => ExitCode::SUCCESS,
                    Err(error) => {
                        error::write(error);
                        ExitCode::FAILURE
                    }
                };
            }
        }
        if argument.is_option() {
            error::write(if let Some(command) = command {
                anyhow!(
                    r#"invalid option "{argument}" provided for "{}" command."#,
                    command.name()
                )
            } else {
                anyhow!(r#"invalid option "{argument}" provided."#)
            });
            return ExitCode::FAILURE;
        }
    }
    if let Err(error) = match command {
        None => Err(anyhow!("no command provided.")),
        Some(Command::Prompt) => {
            if arguments.len() == 1 {
                error::write(anyhow!("no prompt side provided."));
                return ExitCode::FAILURE;
            }
            match arguments[1].as_str() {
                "l" | "left" => prompt::left::write(),
                "r" | "right" => prompt::right::write(),
                _ => Err(anyhow!(
                    r#"invalid prompt side "{}" provided."#,
                    arguments[1]
                )),
            }
        }
        Some(Command::Init) => prompt::init(),
    } {
        error::write(error);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
