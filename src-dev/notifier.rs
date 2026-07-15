use std::fmt::Display;
use std::io::{self, Write};

use colored::Colorize as _;

/// Writes formatted messages to an output stream.
pub struct Notifier<W> {
    sink: W,
}

/// User-facing status label for messages emitted by [`Notifier`].
#[derive(Debug, Clone, Copy, strum::Display)]
pub enum Status {
    Finished,
}

impl<W> Notifier<W>
where
    W: Write,
{
    /// Constructs a new notifier.
    #[must_use]
    pub const fn new(sink: W) -> Self {
        Self { sink }
    }

    /// Prints a right-aligned green status message.
    pub fn status(&mut self, status: Status, message: impl Display) -> io::Result<()> {
        writeln!(
            self.sink,
            "{:>12} {message}",
            status.to_string().bold().green()
        )
    }

    /// Prints a yellow "warning" message.
    pub fn warn(&mut self, message: impl Display) -> io::Result<()> {
        writeln!(self.sink, "{}: {message}", "warning".bold().yellow())
    }

    /// Prints a red "error" message.
    pub fn error(&mut self, message: impl Display) -> io::Result<()> {
        writeln!(self.sink, "{}: {message}", "error".bold().red())
    }

    /// Prints a cyan "note" message.
    pub fn note(&mut self, message: impl Display) -> io::Result<()> {
        writeln!(self.sink, "{}: {message}", "note".bold().cyan())
    }

    /// Prints a green "hint" message.
    pub fn hint(&mut self, message: impl Display) -> io::Result<()> {
        writeln!(self.sink, "{}: {message}", "hint".bold().green())
    }
}
