use std::path::{Path, PathBuf};

/// Directory containing a manifest and application-related files and directories.
#[derive(Debug)]
pub struct Package {
    path: PathBuf,
}

impl Package {
    /// Constructs a new package.
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Path to the root of the package.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Path to the package manifest.
    #[must_use]
    pub fn manifest(&self) -> PathBuf {
        self.path.join("Celestia.toml")
    }
}
