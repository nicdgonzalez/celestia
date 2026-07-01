pub mod server;

use std::path::{Path, PathBuf};

use server::Server;

#[derive(Debug, Clone)]
pub struct Package {
    path: PathBuf,
}

impl Package {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Package { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn backups(&self) -> PathBuf {
        self.path.join("backups")
    }

    pub fn server(&self) -> Server {
        Server::new(self.path.join("server"))
    }

    pub fn manifest(&self) -> PathBuf {
        self.path.join("Celestia.toml")
    }
}
