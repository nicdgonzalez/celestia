use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, bail};
use tracing::info;

use crate::commands::prelude::*;
use crate::commands::update::resolve_server_info;
use crate::manifest::Manifest;
use crate::notifier::Action;
use crate::package::Package;
use crate::package::server::ServerInfo;
use crate::paper::{Build, Version};

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
        let package_path = ctx.package().path().join(&self.path);
        let package = Package::new(package_path);
        let manifest_path = package.manifest();
        let mut manifest = Manifest::default();

        // TODO: Cache responses with an expiry timestamp to reduce load on external servers.
        let ServerInfo { version, build } = resolve_server_info(
            ctx.paper_client(),
            self.version.as_ref(),
            self.build,
            self.allow_experimental,
        )
        .await?;

        if package
            .path()
            .try_exists()
            .context("failed to check if package already exists")?
        {
            bail!("Package already exists at: {}", package.path().display());
        }

        fs::create_dir_all(package.path()).context("failed to create package directory")?;
        info!("directory created: {}", package.path().display());
        *ctx.package_mut() = package;

        manifest.set_version(version.clone());
        manifest.set_build(build);

        manifest
            .save(&manifest_path)
            .context("failed to save initial manifest")?;

        ctx.notify(
            Action::Created,
            format!("Manifest: {}", manifest_path.display()),
        );

        ctx.notify(
            Action::Finished,
            format!("Server created for Minecraft version {version} (#{build})"),
        );

        Ok(())
    }
}
