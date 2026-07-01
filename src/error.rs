use std::error::Error;
use std::{fmt, iter};

use colored::Colorize as _;

/// Error reporter that prints an error and its sources.
#[derive(Debug, Clone, Copy)]
pub struct Reporter<'a> {
    error: &'a (dyn Error + 'static),
}

impl<'a> Reporter<'a> {
    pub fn new(error: &'a (dyn Error + 'static)) -> Self {
        Self { error }
    }

    /// Format the report as multiple lines, with each cause on its own line.
    pub fn fmt_multiline(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let program = env!("CARGO_BIN_NAME");
        write!(f, "{}", format!("{program} failed").bold().red())?;

        for cause in chain(self.error) {
            write!(f, "\n  {}: {cause}", "Cause".bold())?;
        }

        Ok(())
    }
}

impl fmt::Display for Reporter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_multiline(f)
    }
}

fn chain(error: &dyn Error) -> impl Iterator<Item = &dyn Error> {
    iter::successors(Some(error), |e| Error::source(*e))
}
