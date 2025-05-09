use std::io;

use anyhow::Result;

use crate::command_line::stdout_write;

pub(crate) mod left;
pub(crate) mod right;

type Size = u16;

const ZSH_PERCENTAGE_SYMBOL: &str = "%%";
const ZSH_EXIT_CODE: &str = "%?";
const ZSH_JOBS_COUNT: &str = "%j";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Color {
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

fn color_symbol(symbol: impl AsRef<str>, color: Color) -> String {
    format!("%F{{{}}}{}%f", color.ansi(), symbol.as_ref())
}

fn show_symbol_when_root(symbol: impl AsRef<str>) -> String {
    format!("%(#.{}.)", symbol.as_ref())
}

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

fn show_symbol_when_job(symbol: impl AsRef<str>) -> String {
    format!("%(1j.{}.)", symbol.as_ref())
}
