#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

use crate::error::ExitCode;

mod commands;
mod error;
mod runner;

fn main() -> ExitCode {
    runner::run()
}
