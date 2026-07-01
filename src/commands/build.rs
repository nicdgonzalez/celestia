use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::Duration;

use anyhow::Context as _;
use bytes::Bytes;
use reqwest::{Client, Response};
use tracing::info;

use crate::commands::prelude::*;
use crate::launcher::Preset;
use crate::manifest::{Manifest, Name, Source};
use crate::notifier::Action;
use crate::package::server::{GetServerInfoError, Jar};
use crate::paper::{self, Project};
use crate::plugins::PluginRepositoryFactory;

/// Arguments for the `build` subcommand.
#[derive(clap::Args)]
pub struct Build {
    /// Run without committing changes
    #[clap(long)]
    dry: bool,
}

impl Run for Build {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let manifest_path = ctx.package().manifest();
        let manifest = Manifest::open(&manifest_path).context("failed to get manifest")?;

        let server = ctx.package().server();
        let jar = server.jar();
        let start_sh = server.start_sh();
        let plugins_dir = server.plugins();

        // Server subdirectory

        if !server
            .try_exists()
            .context("failed to check if server exists")?
        {
            fs::create_dir_all(server.path()).context("failed to create server directory")?;
            info!("directory created: {}", server.path().display());
        }

        // Server JAR

        if !is_up_to_date(&jar, &manifest)? {
            let version = manifest.version();
            let build = manifest.build();

            ctx.notify(
                Action::Downloading,
                format!("Server JAR for Minecraft {version} (#{build})"),
            );

            let server_jar = download_server_jar(ctx.paper_client(), version, build)
                .await
                .context("failed to download server JAR")?;

            if !self.dry {
                fs::write(&jar, server_jar).context("failed to save server JAR")?;
            }
        }

        // Generated server files

        if !server
            .has_initial_files()
            .context("failed to check if server has initial setting files")?
        {
            ctx.notify(Action::Generating, "Initial server files");

            if !self.dry {
                server
                    .generate_initial_files()
                    .context("failed to generate initial server files")?;

                server.accept_eula().context("failed to accept EULA")?;
            }
        }

        // Start script

        if !start_sh
            .try_exists()
            .context("failed to check if start.sh exists")?
        {
            let jvm_flags = Preset::Aikars.flags().join(" ");
            let contents = format!("/usr/bin/env java\n\njava {jvm_flags} -jar server.jar --nogui");

            fs::write(&start_sh, contents).context("failed to save start.sh")?;
            set_executable_permissions(&start_sh).context("failed to make start.sh executable")?;
            ctx.notify(Action::Created, format!("start.sh: {}", start_sh.display()));
        }

        // Plugins

        if !plugins_dir.try_exists().unwrap_or(false) && !self.dry {
            fs::create_dir_all(&plugins_dir).context("failed to create plugins directory")?;
        }

        for (name, plugin) in manifest.plugins() {
            let output = plugin.file.clone().map_or_else(
                || plugins_dir.join(format!("{name}.jar")),
                |file_name| plugins_dir.join(file_name),
            );

            // TODO: Check plugin's target and actual version before `continue`-ing.
            // TODO: Create `Celestia.lock` to track plugins.
            if output
                .try_exists()
                .context("failed to check if plugin exists")?
            {
                continue;
            }

            match &plugin.source {
                Source::Registry { version, .. } => {
                    ctx.notify(Action::Installing, format!("{name} ({version})"));
                }
                Source::Url { .. } => {
                    ctx.notify(Action::Installing, format!("{name} from URL"));
                }
                Source::Local { path } => {
                    ctx.notify(
                        Action::Installing,
                        format!("{name} from {}", path.display()),
                    );
                }
            }

            if !self.dry {
                install_plugin(name, &plugin.source, &output)
                    .await
                    .context("failed to install plugin")?;
            }
        }

        ctx.notify(Action::Finished, "Run `start` to open the server");
        Ok(())
    }
}

fn is_up_to_date(jar: &Jar, manifest: &Manifest) -> anyhow::Result<bool> {
    match jar.get_server_info() {
        Ok(target) => Ok(&target.version == manifest.version() && target.build == manifest.build()),
        Err(GetServerInfoError::NotExists) => Ok(false),
        Err(err) => Err(err).context("failed to get server JAR information"),
    }
}

async fn download_server_jar(
    paper: &paper::Client,
    version: &paper::Version,
    build: paper::Build,
) -> anyhow::Result<Bytes> {
    let build_response = paper
        .build(Project::Paper, version, build)
        .await
        .context("failed to get build information from Paper")?;

    let server_jar = paper
        .download_jar(&build_response)
        .await
        .context("failed to download server JAR")?;

    Ok(server_jar)
}

fn set_executable_permissions(start_sh: &Path) -> anyhow::Result<()> {
    let metadata = start_sh
        .metadata()
        .context("failed to get metadata for start.sh")?;
    let permissions = metadata.permissions();
    // Leave existing permissions as-is, but include execute permission for "User".
    let mode = permissions.mode() | 0o700; // User | Group | Others

    fs::set_permissions(start_sh, Permissions::from_mode(mode))
        .context("failed to set permissions for start.sh")?;

    Ok(())
}

async fn install_plugin(id: &Name, source: &Source, output: &Path) -> anyhow::Result<()> {
    let bytes = match source {
        Source::Registry {
            registry,
            id,
            version,
        } => {
            let repo = PluginRepositoryFactory::create(*registry);

            let artifact = repo
                .resolve(id, Some(version))
                .await
                .context("failed to resolve plugin")?;

            repo.download(&artifact)
                .await
                .context("failed to download plugin from repository")?
        }

        Source::Url { url } => Client::new()
            .get(url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .and_then(Response::error_for_status)
            .context("failed to send request to custom URL")?
            .bytes()
            .await
            .context("failed to get plugin JAR from custom URL")?,

        Source::Local { path } => fs::read(path)
            .context("failed to read file from local path")?
            .into(),
    };

    fs::write(output, bytes).with_context(|| format!("failed to save plugin: {id}"))?;
    info!("file created: {}", output.display());

    Ok(())
}
