use std::thread;
use std::time::Duration;

use anyhow::{Context as _, bail};
use tracing::debug;

use crate::commands::prelude::*;
use crate::context::TmuxContext;
use crate::notifier::Action;
use crate::tmux;
use crate::watcher::{Status, Watcher};

/// Arguments for the `stop` subcommand.
#[derive(clap::Args)]
pub struct Stop;

impl Run for Stop {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let TmuxContext {
            server: tmux_server,
            session,
        } = ctx.tmux_ctx();

        let package = ctx.package();
        let server = package.server();
        let latest_log = server.latest_log();

        let window_id = package
            .path()
            .file_name()
            .and_then(|f| f.to_str())
            .context("path contains invalid unicode")?
            .to_owned();

        if !server
            .try_exists()
            .context("failed to check if server directory exists")?
        {
            bail!("server directory not found; try running the `build` command first");
        }

        let window = tmux::Window::new(window_id);

        if !window.try_exists(&tmux_server, &session)? {
            ctx.notify(Action::Warning, "Server is not running");
            return Ok(());
        }

        ctx.notify(Action::Waiting, "Closing server");

        // Assumes the user is not currently typing a command in the console...
        // Alternatively, we could send Ctrl+C, but I'm not sure how reliable that is.
        window
            .send_keys(&tmux_server, &session, "stop")
            .context("failed to send `stop` to the server")?;

        let duration = Duration::from_secs(1);
        debug!("sleeping for {duration:?} to give the server a chance to update latest.log");
        thread::sleep(duration);

        ctx.notify(Action::Waiting, "Checking `latest.log` for updates");
        let mut watcher = Watcher::open(&latest_log).context("failed to open latest.log")?;

        match watcher.poll(handler)? {
            Status::Success => ctx.notify(Action::Finished, "Server is listening for connections"),
            // Server may or may not have started -- we simply ran out of time checking.
            // Try running `status` to see if maybe we missed the target message.
            Status::TimeOut => ctx.notify(
                Action::Warning,
                "Timed out while waiting for server to start",
            ),
            _ => unreachable!(),
        }

        Ok(())
    }
}

fn handler(line: &str) -> Option<Status> {
    if line.contains("Stopping server") {
        return Some(Status::Success);
    }

    None
}
