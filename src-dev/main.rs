//! This crate provides the command-line executable.

#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod commands;
mod context;
mod error;
mod notifier;
mod reporter;
mod runner;
mod runtime;

use crate::error::ExitCode;

fn main() -> ExitCode {
    runtime::run()
}
