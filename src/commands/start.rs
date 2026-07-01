use std::ops::Not;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use anyhow::{Context as _, bail};
use tracing::debug;

use crate::commands::prelude::*;
use crate::context::TmuxContext;
use crate::notifier::Action;
use crate::tmux::{self, CreateSessionOptions, CreateWindowOptions};
use crate::watcher::{Status, Watcher};

/// Arguments for the `start` subcommand.
#[derive(clap::Args)]
pub struct Start;

impl Run for Start {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let TmuxContext {
            server: tmux_server,
            session,
        } = ctx.tmux_ctx();

        let package = ctx.package();
        let server = package.server();
        let server_path = server.path().to_owned();
        let start_sh = server.start_sh();
        let latest_log = server.latest_log();

        let window_id = package
            .path()
            .file_name()
            .and_then(|f| f.to_str())
            .context("path contains invalid unicode")?
            .to_owned();
        let window = tmux::Window::new(window_id);

        if !server_path
            .try_exists()
            .context("failed to check if server directory exists")?
        {
            bail!("server directory not found; try running the `build` command first");
        }

        if !start_sh
            .try_exists()
            .context("failed to check if start.sh exists")?
        {
            bail!("start.sh not found");
        }

        if window.try_exists(&tmux_server, &session)? {
            ctx.notify(Action::Warning, "Server is already running");
            return Ok(());
        }

        ctx.notify(Action::Waiting, "Launching server");

        let (session, window) =
            run_server_in_tmux_window(&tmux_server, session, window.into_inner(), server_path)?;

        let duration = Duration::from_secs(3);
        debug!("sleeping for {duration:?} to give the server a chance to update latest.log");
        thread::sleep(duration);

        ctx.notify(Action::Waiting, "Checking `latest.log` for updates");
        let mut watcher = Watcher::open(&latest_log)
            .context("failed to open latest.log")?
            .with_pre_hook(|| window.try_exists(&tmux_server, &session).map(Not::not));

        match watcher.poll(handler)? {
            Status::PreHook => bail!("tmux window closed; start.sh ended early"),
            Status::Success => ctx.notify(Action::Finished, "Server is listening for connections"),
            Status::Failure => bail!("failed to start the server; see server/logs/latest.log"),
            // Server may or may not have started -- we simply ran out of time checking.
            // Try running `status` to see if maybe we missed the target message.
            Status::TimeOut => ctx.notify(
                Action::Warning,
                "Timed out while waiting for server to start",
            ),
        }

        Ok(())
    }
}

fn run_server_in_tmux_window(
    server: &tmux::Server,
    session: tmux::Session,
    window_id: String,
    current_dir: PathBuf,
) -> anyhow::Result<(tmux::Session, tmux::Window)> {
    let command = "./start.sh".to_owned();
    let window_opts = CreateWindowOptions {
        detached: true,
        current_dir: Some(current_dir.clone()),
        command: Some(command.clone()),
    };

    // TODO: Return `window_id` in error value to avoid clone.
    if let Ok(window) = session.create_window(server, window_id.clone(), &window_opts) {
        Ok((session, window))
    } else {
        // If the session does not exist, we need to create it first.
        let session_opts = CreateSessionOptions {
            detached: true,
            current_dir: Some(current_dir),
            command: Some(command),
        };

        let (session, window) = server
            .create_session_and_window(session.into_inner(), window_id, &session_opts)
            .context("failed to create tmux window")?;

        Ok((session, window))
    }
}

fn handler(line: &str) -> Option<Status> {
    if line.contains(r#"s)! For help, type "help""#) {
        return Some(Status::Success);
    } else if line.contains("Failed to start the minecraft server") {
        return Some(Status::Failure);
    }

    None
}
