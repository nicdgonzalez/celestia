#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod commands;
mod context;

use std::io;
use std::io::Write as _;
use std::process::ExitCode;

use clap::Parser as _;
use clap_verbosity_flag::Verbosity;
use colored::Colorize as _;

use crate::context::Context;

#[derive(Debug, clap::Parser)]
struct Parser {
    #[clap(subcommand)]
    subcommand: commands::Subcommand,

    #[clap(flatten)]
    verbosity: Verbosity,
}

#[tokio::main]
async fn main() -> ExitCode {
    try_main().await.unwrap_or_else(|err| {
        writeln!(io::stderr(), "{}", "celestia failed".bold().red()).ok();

        for cause in err.chain() {
            writeln!(io::stderr(), "  {}: {}", "Cause".bold(), cause).ok();
        }

        ExitCode::FAILURE
    })
}

async fn try_main() -> anyhow::Result<ExitCode> {
    let args = Parser::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();

    let mut ctx = Context::new();
    args.subcommand
        .run(&mut ctx)
        .await
        .map(|()| ExitCode::SUCCESS)
}
