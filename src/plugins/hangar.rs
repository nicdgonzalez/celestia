use async_trait::async_trait;
use reqwest::Client;

use crate::plugins::{PluginArtifact, PluginRepository, RepositoryError, SearchResult};

#[derive(Debug, Clone, Default)]
pub struct Hangar {
    #[expect(dead_code)]
    http: Client,
}

impl Hangar {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl PluginRepository for Hangar {
    async fn search(&self, _query: &str) -> Result<Vec<SearchResult>, RepositoryError> {
        todo!()
    }

    async fn resolve(
        &self,
        _id: &str,
        _version: Option<&str>,
    ) -> Result<PluginArtifact, RepositoryError> {
        todo!()
    }

    async fn download(&self, _artifact: &PluginArtifact) -> Result<bytes::Bytes, RepositoryError> {
        todo!()
    }
}
