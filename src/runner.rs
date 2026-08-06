use core::error::Error;

use tokio::runtime::Runtime;

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

/// Constructs the async runtime.
fn build_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
}

/// Executes the application, propagating errors to the caller.
async fn try_run() -> Result<(), Box<dyn Error>> {
    // TODO: Search for plugins on Modrinth and return the latest stable release.
    unimplemented!()
}
