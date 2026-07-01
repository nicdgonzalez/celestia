use std::fs::File;
use std::path::Path;
use std::{env, fs, thread};

use anyhow::Context as _;
use chrono::Local;
use flate2::Compression;
use flate2::write::GzEncoder;

use crate::commands::prelude::*;
use crate::context::TmuxContext;
use crate::notifier::Action;
use crate::tmux;

/// Arguments for the `backup` subcommand.
#[derive(clap::Args)]
pub struct Backup {
    /// Block the current process until the backup is complete.
    #[arg(long)]
    wait: bool,
}

impl Run for Backup {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let TmuxContext {
            server: tmux_server,
            session,
        } = ctx.tmux_ctx();

        let package = ctx.package();
        let server_path = package.server().path().to_owned();
        let backups = package.backups();
        let window_id = package
            .path()
            .file_name()
            .and_then(|f| f.to_str())
            .context("path contains invalid unicode")?
            .to_owned();

        let file_name = Local::now().to_rfc3339();
        let output = backups.join(&file_name);

        fs::create_dir_all(&backups).context("failed to create backups directory")?;
        env::set_current_dir(&backups).context("failed to change into backup directory")?;

        let handle = thread::Builder::new()
            .spawn(move || handler(&tmux_server, &session, window_id, &server_path, &output))
            .context("failed to start backup in separate thread")?;

        if self.wait {
            ctx.notify(Action::Waiting, "Backup started. Please wait a few minutes");

            handle
                .join()
                // The spawned thread most likely panicked.
                .expect("failed to join on associated thread")?; // Propagate errors
        } else {
            ctx.notify(
                Action::Tip,
                "Use `--wait` to block the current process until the backup is complete",
            );
        }

        ctx.notify(Action::Finished, "Backup complete");
        Ok(())
    }
}

fn handler(
    server: &tmux::Server,
    session: &tmux::Session,
    window_id: String,
    server_path: &Path,
    output: &Path,
) -> anyhow::Result<()> {
    let window = tmux::Window::new(window_id);

    // Temporarily disable auto-save if the server is currently running.
    if window
        .try_exists(server, session)
        .inspect_err(|err| tracing::error!("failed to check if window exists: {err}"))
        .unwrap_or(false)
    {
        window
            .send_keys(server, session, "save-off")
            .inspect_err(|err| tracing::error!("failed to send save-off command: {err}"))
            .ok();

        window
            .send_keys(
                server,
                session,
                "say Server backup in progess. Auto-save has been disabled",
            )
            .ok();
    }

    // Create the backup file
    let file = File::create(output).context("failed to create backup file")?;
    let encoder = GzEncoder::new(file, Compression::best());

    // Compress the directory into a tarball
    let mut tar = tar::Builder::new(encoder);
    tar.follow_symlinks(false);

    tar.append_dir_all("", server_path)
        .context("failed to compress server directory")?;

    // Backup complete; re-enable auto-save.
    if window
        .try_exists(server, session)
        .inspect_err(|err| tracing::error!("failed to check if window exists: {err}"))
        .unwrap_or(false)
    {
        window
            .send_keys(server, session, "save-on")
            .inspect_err(|err| tracing::error!("failed to send save-on command: {err}"))
            .ok();

        window
            .send_keys(
                server,
                session,
                "say Server backup complete! Auto-save has been re-enabled",
            )
            .ok();
    }

    Ok(())
}
