use std::path::PathBuf;

use anyhow::bail;

use crate::commands::Run;
use crate::context::Context;

#[derive(Debug, Clone, Default, clap::Args)]
pub struct Init {
    /// Name to use when the server is displayed in the web dashboard.
    #[arg(long)]
    name: Option<String>,

    /// Path to the server's `server.jar` file.
    ///
    /// If the JAR file is not named `server.jar`, a symbolic link will be created to ensure it
    /// exists. This is done to simplify the `start.sh` script and is subject to change since this
    /// is not a *real* limitation, but rather just me being lazy. :)
    #[arg(long)]
    server_jar: Option<PathBuf>,

    /// Target build number.
    ///
    /// Must be a valid build number for the target Minecraft version.
    #[arg(long)]
    build: Option<u32>,
}

impl Run for Init {
    async fn run(&self, _ctx: &mut Context) -> anyhow::Result<()> {
        // Register the server into the database.

        // Create a symbolic link from the target directory to `$XDG_DATA_HOME/celestia/servers`.

        // Store the server ID somewhere in the server directory.

        // Install the Celestia plugin.

        bail!("not implemented yet")
    }
}
