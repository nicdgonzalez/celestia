use std::error::Error;
use std::fmt;

use colored::Colorize as _;

/// Error reporter that formats an error and its source chain into a human-readable report.
pub struct Reporter<'a> {
    error: &'a (dyn Error + 'static),
}

impl<'a> Reporter<'a> {
    /// Constructs a new error reporter.
    pub const fn new(error: &'a (dyn Error + 'static)) -> Self {
        Self { error }
    }
}

impl Reporter<'_> {
    /// Format the report as multiple lines, with each cause on its own line.
    pub fn fmt_multiline(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut source: Option<&dyn Error> = self.error.source();

        write!(f, "{}: {}", "error".bold().red(), self.error)?;

        while let Some(cause) = source {
            write!(f, "  {}: {cause}", "Cause".bold())?;
            source = cause.source();
        }

        Ok(())
    }
}

impl fmt::Display for Reporter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_multiline(f)
    }
}
