use tokio::runtime::Runtime;

use crate::{ExitCode, runner};

/// Starts the application.
pub fn run() -> ExitCode {
    build_runtime().block_on(runner::run())
}

fn build_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
}
