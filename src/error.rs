use std::error::Error;
use std::io::Write as _;
use std::{fmt, io, iter};

use colored::Colorize as _;

/// Describes the result of the program after it has terminated.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ExitCode {
    /// Program terminated without any errors.
    Success = 0,
    /// Program terminated due to an unrecoverable error.
    Failure = 1,
}

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self as u8)
    }
}

/// Reports an application error and determines the appropriate exit code.
///
/// Most errors are formatted, output to standard error, and return [`ExitCode::Failure`].
/// Errors caused by a broken pipe return [`ExitCode::Success`] to match conventional Unix
/// CLI behavior.
pub fn report(error: &(dyn Error + 'static)) -> ExitCode {
    if is_broken_pipe(error) {
        return ExitCode::Success;
    }

    let mut stderr = io::stderr().lock();
    write!(stderr, "{}", Reporter::new(error)).ok();

    ExitCode::Failure
}

fn is_broken_pipe(error: &(dyn Error + 'static)) -> bool {
    error
        .downcast_ref::<io::Error>()
        .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe)
}

/// Error reporter that formats an error and its source chain into a human-readable report.
struct Reporter<'a> {
    error: &'a (dyn Error + 'static),
}

impl<'a> Reporter<'a> {
    /// Constructs a new error reporter.
    #[must_use]
    pub const fn new(error: &'a (dyn Error + 'static)) -> Self {
        Self { error }
    }
}

impl Reporter<'_> {
    /// Formats the report as multiple lines, with each cause on its own line.
    fn fmt_multiline(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", "error".bold().red(), self.error).ok();

        let causes =
            iter::successors(self.error.source(), |e| Error::source(*e)).collect::<Vec<_>>();

        if !causes.is_empty() {
            let width = causes.len().max(1).to_string().len();

            writeln!(f, "\n\nCaused by:")?;

            for (i, cause) in causes.iter().enumerate() {
                writeln!(f, "  {i:>width$}: {cause}")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Reporter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_multiline(f)
    }
}
