use crate::command_line::stdout_write;
use anyhow::Result;
use std::io;

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

fn color_symbol<T>(symbol: T, color: Color) -> String
where
    T: AsRef<str>,
{
    format!("%F{{{}}}{}%f", color.ansi(), symbol.as_ref())
}

fn show_symbol_when_root<T>(symbol: T) -> String
where
    T: AsRef<str>,
{
    format!("%(#.{}.)", symbol.as_ref())
}

fn show_symbols_for_exit_codes<T, W>(on_success_symbol: T, on_error_symbol: W) -> String
where
    T: AsRef<str>,
    W: AsRef<str>,
{
    format!(
        "%(?.{}.{})",
        on_success_symbol.as_ref(),
        on_error_symbol.as_ref()
    )
}

fn show_symbol_when_job<T>(symbol: T) -> String
where
    T: AsRef<str>,
{
    format!("%(1j.{}.)", symbol.as_ref())
}
