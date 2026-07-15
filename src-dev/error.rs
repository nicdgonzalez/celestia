use std::error::Error;
use std::io;
use std::io::Write as _;

use crate::reporter::Reporter;

/// Describes the result of the program after it has terminated.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Reports an application error and determines the appropriate process exit code.
///
/// Most errors are formatted and written to standard error, and return [`ExitCode::Failure`].
/// Errors caused by a broken pipe return [`ExitCode::Success`] to match conventional Unix CLI
/// behavior.
pub fn report(error: &(dyn Error + 'static)) -> ExitCode {
    if is_broken_pipe(error) {
        return ExitCode::Success;
    }

    let mut stderr = io::stderr().lock();
    writeln!(stderr, "{}", Reporter::new(error)).ok();

    ExitCode::Failure
}

fn is_broken_pipe(error: &(dyn Error + 'static)) -> bool {
    error
        .downcast_ref::<io::Error>()
        .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe)
}
