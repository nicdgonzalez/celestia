use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs};

use anyhow::Context as _;
use bytes::Bytes;
use clap::Parser as _;
use reqwest::{Client, Response};
use tokio::runtime::Runtime;

use crate::commands::Cli;
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
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
}

#[derive(serde::Serialize)]
struct SearchParams {
    query: String,
    facets: Facets,
}

#[derive(serde::Serialize)]
struct Facets(String);

#[derive(serde::Deserialize)]
struct SearchResponse {
    hits: Vec<Hit>,
}

#[derive(serde::Deserialize)]
struct Hit {
    slug: String,
    project_id: String,
}

#[derive(Debug)]
struct SearchResult {
    id: String,
    name: String,
}

/// Executes the application, propagating errors to the caller.
async fn try_run() -> anyhow::Result<()> {
    let args = Cli::parse();

    let user_agent = format!(
        "nicdgonzalez/celestia/{version} (ndgonzalez.work@gmail.com)",
        version = env!("CARGO_PKG_VERSION")
    );

    let client = Client::builder()
        .user_agent(user_agent)
        .build()
        .context("failed to initialize TLS backend")?;

    let mut results = search_for_plugin(&client, args.name.clone()).await?;

    let plugin = results
        .find(|hit| args.name.eq_ignore_ascii_case(&hit.name))
        .context("plugin not found")?;

    let latest_version = list_versions(&client, &plugin.id)
        .await?
        .next() // First entry is the latest version.
        .context("versions not found")?;

    let file = latest_version
        .files
        .iter()
        .find(|file| file.primary)
        .or_else(|| latest_version.files.first())
        .context("no files found")?;

    let bytes = download_plugin(&client, file).await?;

    let mut plugins_dir = env::current_dir().unwrap_or(PathBuf::from("/"));
    plugins_dir.push("plugins");
    fs::create_dir_all(&plugins_dir).context("failed to create plugins directory")?;

    let output = plugins_dir.join(&file.filename);
    fs::write(&output, bytes).context("failed to save plugin")?;
    println!("file created: {}", output.display());

    Ok(())
}

async fn search_for_plugin(
    client: &Client,
    plugin_name: String,
) -> anyhow::Result<impl Iterator<Item = SearchResult>> {
    let params = SearchParams {
        query: plugin_name,
        facets: Facets(
            r#"[["categories:paper"],["versions:26.2"],["project_type:plugin"]]"#.to_owned(),
        ),
    };
    let response = client
        .get("https://api.modrinth.com/v2/search")
        .query(&params)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .and_then(Response::error_for_status)
        .context("failed to send request to Modrinth API")?;

    let body = response
        .json::<SearchResponse>()
        .await
        .context("failed to parse search response")?;

    Ok(body.hits.into_iter().map(|hit| SearchResult {
        id: hit.project_id,
        name: hit.slug,
    }))
}

#[derive(Debug, serde::Deserialize)]
struct Version {
    // version_number: String,
    version_type: VersionType,
    // id: String,
    files: Vec<File>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct File {
    filename: PathBuf,
    url: String,
    primary: bool,
}

async fn list_versions(client: &Client, id: &str) -> anyhow::Result<impl Iterator<Item = Version>> {
    let url = format!("https://api.modrinth.com/v2/project/{id}/version");
    let params = &[
        ("loaders", "paper"),
        ("game_versions", r#"["26.2"]"#),
        ("include_changelog", "false"),
    ];
    let response = client
        .get(&url)
        .query(&params)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .and_then(Response::error_for_status)
        .context("failed to send request to Modrinth API")?;

    let body = response
        .json::<Vec<Version>>()
        .await
        .context("failed to parse list versions response")?;

    Ok(body
        .into_iter()
        .filter(|v| v.version_type == VersionType::Release))
}

async fn download_plugin(client: &Client, file: &File) -> anyhow::Result<Bytes> {
    client
        .get(&file.url)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .and_then(Response::error_for_status)
        .context("failed to send request to Modrinth API")?
        .bytes()
        .await
        .context("failed to download plugin")
}
