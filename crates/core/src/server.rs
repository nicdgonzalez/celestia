//! This module defines how to operate on and use a `Server`.

use std::path::{Path, PathBuf};

/// Repository abstraction for storing and retrieving persistent `Server` entries.
#[expect(
    async_fn_in_trait,
    reason = "Only I use this trait, so I will know when I need it to be `dyn`-compatible."
)]
pub trait Repository {
    /// Describes an error that occurred while inserting an entry to the repository.
    type InsertError;

    /// Describes an error that occurred while getting an entry from the repository.
    type GetByIdError;

    /// Create a new server entry in the repository.
    async fn insert(&self, name: String, path: PathBuf) -> Result<Server, Self::InsertError>;

    /// Retrieve an existing server by searching via `id`.
    async fn get_by_id(&self, id: String) -> Result<Server, Self::GetByIdError>;
}

/// Represents a Minecraft server that has been assigned an ID.
pub struct Server {
    id: String,
    name: String,
    path: PathBuf,
}

impl Server {
    /// Create a new `Server`.
    #[must_use]
    pub fn new(id: String, name: String, path: PathBuf) -> Self {
        Self { id, name, path }
    }

    /// Unique identifier for the Minecraft server at [`Self::path`].
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Display name for the current server in the web dashboard.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Path to the Minecraft server.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}
