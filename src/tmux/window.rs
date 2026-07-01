use std::process::{Command, Stdio};

use anyhow::{Context as _, bail};

use crate::tmux::{Server, Session};

pub struct Window(String);

impl Window {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    #[expect(dead_code)]
    pub fn id(&self) -> &str {
        &self.0
    }

    pub fn try_exists(&self, server: &Server, session: &Session) -> anyhow::Result<bool> {
        let target = format!("={}:{}", session.id(), self.0);
        let status = Command::new("tmux")
            .args(["-L", server.id(), "has-session", "-t", &target])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("failed to execute tmux")?;

        Ok(status.success())
    }

    pub fn send_keys(
        &self,
        server: &Server,
        session: &Session,
        command: &str,
    ) -> anyhow::Result<()> {
        let target = format!("={}:{}", session.id(), self.0);
        let status = Command::new("tmux")
            .args([
                "-L",
                server.id(),
                "send-keys",
                "-t",
                &target,
                command,
                "Enter",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("failed to execute tmux")?;

        match status.code() {
            Some(0) => Ok(()),
            Some(code) => bail!("process returned a non-zero exit code: {code}"),
            None => bail!("process terminated via signal"),
        }
    }
}
