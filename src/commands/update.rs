use anyhow::{Context as _, bail};
use colored::Colorize as _;

use crate::commands::prelude::*;
use crate::manifest::Manifest;
use crate::notifier::Action;
use crate::package::server::ServerInfo;
use crate::paper::model::BuildResponse;
use crate::paper::{self, Build, Channel, Project, Version};

/// Arguments for the `update` subcommand.
#[derive(clap::Args)]
pub struct Update {
    /// Minecraft version to use. Default to latest version
    #[clap(long)]
    version: Option<Version>,

    /// Build number for server JAR. Defaults to latest build
    #[clap(long, requires = "version")]
    build: Option<Build>,

    /// Allow non-stable builds to be used
    #[clap(long, conflicts_with = "build")]
    allow_experimental: bool,

    /// Run without committing changes
    #[clap(long)]
    dry: bool,
}

impl Run for Update {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let manifest_path = ctx.package().manifest();
        let mut manifest = Manifest::default();

        let ServerInfo { version, build } = resolve_server_info(
            ctx.paper_client(),
            self.version.as_ref(),
            self.build,
            self.allow_experimental,
        )
        .await?;

        if &version < manifest.version() {
            ctx.notify(
                Action::Warning,
                format!(
                    "Downgrading from {before} to {after}! \
                    Using an older version may corrupt your world.",
                    before = manifest.version().to_string().bold(),
                    after = version.to_string().bold()
                ),
            );
        }

        manifest.set_version(version.clone());
        manifest.set_build(build);

        if !self.dry {
            manifest
                .save(&manifest_path)
                .context("failed to save manifest")?;
        }

        ctx.notify(
            Action::Finished,
            format!("Server updated to Minecraft version {version} (#{build})"),
        );

        Ok(())
    }
}

pub async fn resolve_server_info(
    paper: &paper::Client,
    version: Option<&Version>,
    build: Option<Build>,
    allow_experimental: bool,
) -> anyhow::Result<ServerInfo> {
    let versions = paper
        .versions(Project::Paper)
        .await
        .context("failed to get supported Minecraft versions from Paper")?
        .versions;

    if let Some(target_version) = version {
        let version_response = versions
            .into_iter()
            .find(|response| target_version == &response.version.id)
            .context("invalid Minecraft version provided")?;

        let version = version_response.version.id;
        let builds = get_builds(paper, &version, allow_experimental).await?;

        let build_response = match build {
            Some(target_build) => builds
                .into_iter()
                .find(|response| target_build == response.id)
                .context("invalid build provided")?,
            None => builds
                .into_iter()
                .next()
                .with_context(|| format!("no builds found for version {version}"))?,
        };
        let build = build_response.id;

        return Ok(ServerInfo { version, build });
    }

    for response in versions {
        let version = response.version.id;
        let builds = get_builds(paper, &version, allow_experimental).await?;

        if let Some(build_response) = builds.into_iter().next() {
            return Ok(ServerInfo {
                version,
                build: build_response.id,
            });
        }
    }

    bail!("exhausted all potential Minecraft versions");
}

async fn get_builds(
    paper: &paper::Client,
    version: &paper::Version,
    allow_experimental: bool,
) -> anyhow::Result<Vec<BuildResponse>> {
    let channels = if allow_experimental {
        [].as_slice()
    } else {
        [Channel::Stable].as_slice()
    };

    paper
        .builds(Project::Paper, version, channels)
        .await
        .with_context(|| format!("failed to get supported builds for version {version}"))
}
