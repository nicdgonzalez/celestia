use std::fmt::Display;
use std::io::{self, Write};

use colored::Colorize as _;

/// Writes formatted messages to an output stream.
#[derive(Debug, Clone, Copy)]
pub struct Notifier<W> {
    writer: W,
}

impl<W> Notifier<W>
where
    W: Write,
{
    /// Creates a new notifier that writes formatted messages to the given writer.
    pub fn new(writer: W) -> Self {
        Notifier { writer }
    }

    /// Writes a formatted message.
    ///
    /// The output consists of a right-aligned, colored [`Action`] label followed by the provided
    /// message and a trailing newline.
    ///
    /// # Errors
    ///
    /// This function returns an error if an I/O error occurs while writing to the underlying
    /// writer.
    pub fn send<M>(&mut self, action: Action, message: M) -> io::Result<()>
    where
        M: Display,
    {
        let mut label = action.to_string().bold();

        match action {
            Action::Warning => label = label.yellow(),
            _ => label = label.green(),
        }

        writeln!(self.writer, "  {label:>12} {message}")
    }
}

/// User-facing status action.
///
/// These values are displayed as prefixes for messages emitted by [`Notifier`].
#[derive(Debug, Clone, Copy, strum::Display)]
pub enum Action {
    Finished,
    Downloading,
    Generating,
    Adding,
    Installing,
    Removed,
    Warning,
    Waiting,
    Created,
    Tip,
}
