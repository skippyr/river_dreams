macro_rules! stdout_write {
    ($stream:expr, $format:literal $(, $arguments:expr)* $(,)?) => {
        {
            use anyhow::anyhow;
            use std::io::{StdoutLock, Write as _};

            fn __assert_stdout_lock<'a, 'b>(stdout: &'a mut StdoutLock<'b>) -> &'a mut StdoutLock<'b> {
                stdout
            }

            write!(__assert_stdout_lock($stream), $format $(, $arguments)*).map_err(|_| anyhow!("can not write to the standard output stream."))
        }
    };
}

pub(crate) use stdout_write;

pub(crate) trait ArgumentParsing {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Command {
    Prompt,
    Init,
}

impl Command {
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Prompt => "prompt",
        }
    }
}
