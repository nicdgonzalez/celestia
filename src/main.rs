//! Implementation for the Celestia command-line interface.

#![warn(
    missing_docs,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod app;
mod commands;
mod context;
mod error;
mod launcher;
mod manifest;
mod notifier;
mod package;
mod paper;
mod plugins;
mod tmux;
mod varint;
mod watcher;

use std::io;
use std::io::Write as _;

use crate::error::Reporter;

/// Describes the result of the program after it has terminated.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExitCode {
    /// Program terminated without any errors
    Success = 0,
    /// Program terminated due to an unrecoverable error
    Failure = 1,
}

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self as u8)
    }
}

fn main() -> ExitCode {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async {
            match app::run().await {
                Ok(()) => ExitCode::Success,
                Err(error) => {
                    // Traditional Unix programs terminate on `SIGPIPE` when writing to a closed pipe.
                    // Rust ignores `SIGPIPE` by default[1], so the same condition is reported as
                    // `io::ErrorKind::BrokenPipe` instead. We handle this case explicitly to match
                    // conventional Unix CLI behavior.
                    //
                    // [1]: https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/on-broken-pipe.html
                    if error
                        .downcast_ref::<io::Error>()
                        .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe)
                    {
                        return ExitCode::Success;
                    }

                    let mut stderr = io::stderr().lock();
                    writeln!(stderr, "{}", Reporter::new(error.as_ref())).ok();

                    ExitCode::Failure
                }
            }
        })
}
