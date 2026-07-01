use std::fmt::Display;
use std::io::{self, Stdout};
use std::path::PathBuf;

use crate::notifier::{Action, Notifier};
use crate::package::Package;
use crate::{paper, tmux};

/// Shared state for command execution.
#[derive(Debug)]
pub struct Context {
    package: Package,
    paper: paper::Client,
    notifier: Notifier<Stdout>,
}

pub struct TmuxContext {
    pub server: tmux::Server,
    pub session: tmux::Session,
}

impl Context {
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self {
            package: Package::new(root),
            paper: paper::Client::new(),
            notifier: Notifier::new(io::stdout()),
        }
    }

    pub fn package(&self) -> &Package {
        &self.package
    }

    pub fn package_mut(&mut self) -> &mut Package {
        &mut self.package
    }

    pub fn paper_client(&self) -> &paper::Client {
        &self.paper
    }

    #[expect(clippy::unused_self)]
    pub fn tmux_ctx(&self) -> TmuxContext {
        TmuxContext {
            server: tmux::Server::new("celestia".to_owned()),
            session: tmux::Session::new("servers".to_owned()),
        }
    }

    pub fn notify<M>(&mut self, action: Action, message: M)
    where
        M: Display,
    {
        self.notifier.send(action, message).ok();
    }
}
