use clap::Parser as _;
use tokio::runtime::Runtime;

use crate::commands::{self, Cli};
use crate::error::{ExitCode, report};

/// Executes the application.
///
/// If the application completes successfully, [`ExitCode::Success`] is returned.
/// Otherwise, the error is reported and converted into the appropriate exit code.
pub fn run() -> ExitCode {
    build_runtime().block_on(async {
        match try_run().await {
            Ok(()) => ExitCode::Success,
            Err(err) => report(err.as_ref()),
        }
    })
}

/// Constructs the asynchronous runtime.
fn build_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
}

/// Executes the application, propagating errors to the caller.
async fn try_run() -> anyhow::Result<()> {
    let args = Cli::parse();
    commands::run(args).await?;
    Ok(())
}
