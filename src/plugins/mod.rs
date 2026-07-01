mod hangar;
mod modrinth;
mod registry;

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use bytes::Bytes;

pub use hangar::Hangar;
pub use modrinth::Modrinth;
pub use registry::Registry;

#[async_trait]
pub trait PluginRepository {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, RepositoryError>;

    async fn resolve(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<PluginArtifact, RepositoryError>;

    async fn download(&self, artifact: &PluginArtifact) -> Result<Bytes, RepositoryError>;
}

pub struct PluginRepositoryFactory;

impl PluginRepositoryFactory {
    pub fn create(registry: Registry) -> Box<dyn PluginRepository> {
        match registry {
            Registry::Modrinth => Box::new(Modrinth::new()),
            Registry::Hangar => Box::new(Hangar::new()),
        }
    }
}

#[derive(Debug)]
pub struct SearchResult {
    /// Unique identifier for this plugin
    pub id: String,
    /// Title or name of the project
    pub name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PluginArtifact {
    /// Registry the plugin was downloaded from
    pub registry: Registry,
    /// Unique identifier of the plugin
    pub id: String,
    /// Version downloaded
    pub version: String,
    /// Link to the plugin JAR
    pub download_url: String,
}

#[derive(Debug)]
pub enum RepositoryError {
    /// Failed to send request
    Request { source: reqwest::Error },
    /// Failed to parse response
    Parse { source: reqwest::Error },
    /// Resource not found
    NotFound,
    /// Custom error message
    Custom(String),
}

impl Error for RepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Request { ref source } | Self::Parse { ref source } => Some(source),
            Self::NotFound | Self::Custom(..) => None,
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Request { source: _ } => "failed to send request to plugin repository".fmt(f),
            Self::Parse { source: _ } => "failed to parse response from plugin repository".fmt(f),
            Self::NotFound => "resource not found".fmt(f),
            Self::Custom(ref message) => message.fmt(f),
        }
    }
}
