use std::path::PathBuf;
use std::{env, fs};

use anyhow::Context as _;
use clap::Parser as _;
use tokio::runtime::Runtime;

use crate::commands::Cli;
use crate::error::{ExitCode, report};
use crate::modrinth::{ListVersionsRequest, ModrinthClient, SearchRequest};

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
    let modrinth = ModrinthClient::new();

    let search_request = SearchRequest { name: &args.name };
    let plugin = modrinth
        .search_projects(search_request)
        .await
        .context("failed to search for projects")?
        .find(|project| args.name.eq_ignore_ascii_case(&project.name))
        .context("plugin not found")?;

    let list_request = ListVersionsRequest {
        id: &plugin.id,
        version: "26.2",
        allow_experimental: args.allow_experimental,
    };
    let version = modrinth
        .list_versions(list_request)
        .await
        .context("failed to get versions")?
        .find(|v| v.file.is_some())
        .context("version not found")?;

    let file = version.file.unwrap();

    let bytes = modrinth
        .download(&file)
        .await
        .context("failed to download plugin")?;

    let mut plugins_dir = env::current_dir().unwrap_or(PathBuf::from("/"));
    plugins_dir.push("plugins");
    fs::create_dir_all(&plugins_dir).context("failed to create plugins directory")?;

    let output = plugins_dir.join(&file.file_name);
    fs::write(&output, bytes).context("failed to save plugin")?;
    println!("file created: {}", output.display());

    Ok(())
}
