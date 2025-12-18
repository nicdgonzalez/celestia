use std::path::PathBuf;

use anyhow::bail;

use crate::commands::Run;
use crate::context::Context;

#[derive(Debug, Clone, Default, clap::Args)]
pub struct New {
    /// Path to set up the Minecraft server directory.
    ///
    /// Defaults to `$XDG_DATA_HOME/celestia/servers/:id`.
    path: Option<PathBuf>,

    /// Name to use when the server is displayed in the web dashboard.
    #[arg(long)]
    name: Option<String>,

    /// Target Minecraft version to install.
    ///
    /// Must be a valid Minecraft version supported by Paper.
    #[arg(long)]
    version: Option<String>,

    /// Target build number.
    ///
    /// Must be a valid build number for the target Minecraft version.
    #[arg(long)]
    build: Option<u32>,
}

impl Run for New {
    async fn run(&self, _ctx: &mut Context) -> anyhow::Result<()> {
        // Create a new directory at `path`, with Paper version `version`, build number `build`.

        // Initialize the server (run the `init` subcommand).

        bail!("not implemented yet")
    }
}
