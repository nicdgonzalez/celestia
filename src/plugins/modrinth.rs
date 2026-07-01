use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{Client, Response};
use tracing::info;

use crate::plugins::{PluginArtifact, PluginRepository, Registry, RepositoryError, SearchResult};

#[derive(Debug, Clone, Default)]
pub struct Modrinth {
    http: Client,
}

impl Modrinth {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl PluginRepository for Modrinth {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, RepositoryError> {
        #[derive(serde::Deserialize)]
        struct SearchResponse {
            hits: Vec<Hit>,
        }

        #[derive(serde::Deserialize)]
        struct Hit {
            title: String,
            project_id: String,
        }

        let url = format!("https://api.modrinth.com/v2/search?query={query}");

        info!("sending request to Modrinth: {url}");
        let response = self
            .http
            .get(&url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|source| RepositoryError::Request { source })?;

        let body = response
            .json::<SearchResponse>()
            .await
            .map_err(|source| RepositoryError::Parse { source })?;

        let results = body
            .hits
            .into_iter()
            .map(|result| SearchResult {
                id: result.project_id,
                name: result.title,
            })
            .collect();

        Ok(results)
    }

    async fn resolve(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<PluginArtifact, RepositoryError> {
        #[derive(serde::Deserialize)]
        struct ListVersionsResponse {
            version_number: String,
            files: Vec<File>,
        }

        #[derive(serde::Deserialize)]
        struct File {
            url: String,
        }

        let url = format!("https://api.modrinth.com/v2/project/{id}/version");

        let response = self
            .http
            .get(&url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|source| RepositoryError::Request { source })?;

        let body = response
            .json::<Vec<ListVersionsResponse>>()
            .await
            .map_err(|source| RepositoryError::Parse { source })?;

        let plugin = match version {
            Some(version) => body
                .into_iter()
                .find(|result| version == result.version_number)
                .ok_or(RepositoryError::NotFound)?,
            None => body.into_iter().next().ok_or(RepositoryError::NotFound)?,
        };

        let artifact = PluginArtifact {
            registry: Registry::Modrinth,
            id: id.to_owned(),
            version: plugin.version_number,
            download_url: plugin
                .files
                .into_iter()
                .next()
                .ok_or_else(|| {
                    RepositoryError::Custom("this version does not provide any files".to_owned())
                })?
                .url,
        };

        Ok(artifact)
    }

    async fn download(&self, artifact: &PluginArtifact) -> Result<Bytes, RepositoryError> {
        self.http
            .get(&artifact.download_url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(|source| RepositoryError::Request { source })?
            .bytes()
            .await
            .map_err(|source| RepositoryError::Parse { source })
    }
}
