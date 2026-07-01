use std::env;
use std::path::PathBuf;

use clap::Parser as _;

use crate::commands::{Parser, Run as _};
use crate::context::Context;

/// The main entry point to the command-line application.
///
/// This function is responsible for setting up and invoking the user's selected subcommand.
///
/// Errors are propagated to the caller for reporting.
pub async fn run() -> anyhow::Result<()> {
    let args = Parser::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();

    let root = args
        .directory
        .unwrap_or_else(|| env::current_dir().unwrap_or(PathBuf::from("/")));

    let mut ctx = Context::new(root);

    args.subcommand.run(&mut ctx).await
}
