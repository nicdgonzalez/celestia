use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{Context as _, bail};

use crate::tmux::{Session, Window};

pub struct Server(String);

#[derive(Default)]
pub struct CreateSessionOptions {
    pub detached: bool,
    pub current_dir: Option<PathBuf>,
    pub command: Option<String>,
}

impl Server {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn id(&self) -> &str {
        &self.0
    }

    #[expect(dead_code)]
    pub fn create_session(
        &self,
        id: String,
        opts: &CreateSessionOptions,
    ) -> anyhow::Result<Session> {
        let mut args = vec!["-L", &self.0, "new-session"];

        if let Some(ref path) = opts.current_dir {
            args.push("-c");
            args.push(path.to_str().context("path contains invalid unicode")?);
        }

        if opts.detached {
            args.push("-d");
        }

        args.push("-s");
        args.push(&id);

        if let Some(ref command) = opts.command {
            args.push(command);
        }

        let status = Command::new("tmux")
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("failed to execute tmux")?;

        match status.code() {
            Some(0) => Ok(Session::new(id)),
            Some(code) => bail!("process returned a non-zero exit code: {code}"),
            None => bail!("process terminated via signal"),
        }
    }

    pub fn create_session_and_window(
        &self,
        session_id: String,
        window_id: String,
        opts: &CreateSessionOptions,
    ) -> anyhow::Result<(Session, Window)> {
        let mut args = vec!["-L", &self.0, "new-session"];

        if let Some(ref path) = opts.current_dir {
            args.push("-c");
            args.push(path.to_str().context("path contains invalid unicode")?);
        }

        if opts.detached {
            args.push("-d");
        }

        args.push("-s");
        args.push(&session_id);

        args.push("-n");
        args.push(&window_id);

        if let Some(ref command) = opts.command {
            args.push(command);
        }

        let status = Command::new("tmux")
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("failed to execute tmux")?;

        match status.code() {
            Some(0) => Ok((Session::new(session_id), Window::new(window_id))),
            Some(code) => bail!("process returned a non-zero exit code: {code}"),
            None => bail!("process terminated via signal"),
        }
    }
}
