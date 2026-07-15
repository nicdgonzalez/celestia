use std::env;
use std::path::PathBuf;

use clap::Parser as _;

use crate::commands::{Parser, Run as _};
use crate::context::Context;
use crate::error::{ExitCode, report};

/// Executes the application.
///
/// If the application completes successfully, [`ExitCode::Success`] is returned.
/// Otherwise, the error is reported and converted into the appropriate exit code.
pub async fn run() -> ExitCode {
    match try_run().await {
        Ok(()) => ExitCode::Success,
        Err(err) => report(err.as_ref()),
    }
}

/// Executes the application's core logic and propagates any errors to the caller.
async fn try_run() -> anyhow::Result<()> {
    let args = Parser::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();

    let root = args
        .directory
        .unwrap_or_else(|| env::current_dir().unwrap_or(PathBuf::from("/")));

    let mut ctx = Context::new();

    args.subcommand.run(&mut ctx).await?;

    Ok(())
}
