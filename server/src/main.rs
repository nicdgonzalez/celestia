//! This is the main entry point to the application.
//!
//! This module is responsible for preparing and starting the HTTP server.

#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod opcode;
mod routes;
mod state;

use std::io::Write as _;
use std::sync::Arc;
use std::{env, io, process};

use anyhow::Context as _;
use axum::Router;
use colored::Colorize as _;
use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

use state::AppState;

/// Represents the status of the program after it has been terminated.
enum ExitCode {
    /// Indicates the program terminated without any errors.
    Success = 0,
    /// Indicates the program terminated due to an unrecoverable error.
    Failure = 1,
}

impl process::Termination for ExitCode {
    fn report(self) -> process::ExitCode {
        process::ExitCode::from(self as u8)
    }
}

/// The main entry point to our program.
///
/// This function is responsible for displaying errors in a human-readable way.
fn main() -> ExitCode {
    try_main().unwrap_or_else(|err| {
        let mut stderr = io::stderr().lock();
        _ = writeln!(stderr, "{}", "An unrecoverable error occurred".bold().red());

        for cause in err.chain() {
            _ = writeln!(stderr, "  {}: {}", "Cause".bold(), cause);
        }

        ExitCode::Failure
    })
}

/// Initializes the async runtime and starts the HTTP server.
fn try_main() -> Result<ExitCode, anyhow::Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .context("failed to build tokio runtime")?
        .block_on(start_server())
        .map(|()| ExitCode::Success)
}

/// Prepares and runs the HTTP server.
async fn start_server() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, _rx) = broadcast::channel(25);
    let state = Arc::new(AppState::new(tx));

    let app = Router::new().merge(routes::router()).with_state(state);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 1140))
        .await
        .context("failed to bind to address")?;

    tracing::debug!(
        "Listening on: {}",
        listener
            .local_addr()
            .context("failed to get local address")?
    );

    Ok(axum::serve(listener, app).await.unwrap())
}
