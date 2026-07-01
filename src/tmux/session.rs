use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{Context, bail};

use crate::tmux::{Server, Window};

pub struct Session(String);

#[derive(Default)]
pub struct CreateWindowOptions {
    pub detached: bool,
    pub current_dir: Option<PathBuf>,
    pub command: Option<String>,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn id(&self) -> &str {
        &self.0
    }

    #[expect(dead_code)]
    pub fn try_exists(&self, server: &Server) -> anyhow::Result<bool> {
        let target = format!("={}", self.0);
        let status = Command::new("tmux")
            .args(["-L", server.id(), "has-session", "-t", &target])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("failed to execute tmux")?;

        Ok(status.success())
    }

    pub fn create_window(
        &self,
        server: &Server,
        id: String,
        opts: &CreateWindowOptions,
    ) -> anyhow::Result<Window> {
        let mut args = vec!["-L", &server.id(), "new-window"];

        if let Some(ref path) = opts.current_dir {
            args.push("-c");
            args.push(path.to_str().context("path contains invalid unicode")?);
        }

        if opts.detached {
            args.push("-d");
        }

        let session_name = format!("={}", &self.0);
        args.push("-t");
        args.push(&session_name);

        args.push("-n");
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
            Some(0) => Ok(Window::new(id)),
            Some(code) => bail!("process returned a non-zero exit code: {code}"),
            None => bail!("process terminated via signal"),
        }
    }
}
