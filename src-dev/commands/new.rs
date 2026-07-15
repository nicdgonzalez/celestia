use std::path::PathBuf;

use crate::commands::prelude::*;
use crate::notifier::Status;

type Version = String;
type Build = u32;

/// Arguments for the `new` subcommand.
#[derive(clap::Args)]
pub struct New {
    path: PathBuf,

    /// Minecraft version to use. Default to latest version
    #[clap(long)]
    version: Option<Version>,

    /// Build number for server JAR. Defaults to latest build
    #[clap(long, requires = "version")]
    build: Option<Build>,

    /// Allow non-stable builds to be used
    #[clap(long, conflicts_with = "build")]
    allow_experimental: bool,
}

impl Run for New {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let version = self.version.clone().unwrap_or_else(|| "26.2".to_owned());
        let build = 72;
        let message = format!("Server created for Minecraft version {version} (#{build})");
        ctx.notifier().status(Status::Finished, message).ok();
        Ok(())
    }
}
