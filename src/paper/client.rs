use std::time::Duration;

use bytes::Bytes;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::paper::error::PaperError;
use crate::paper::model::{
    BuildResponse, ProjectResponse, ProjectsResponse, VersionResponse, VersionsResponse,
};
use crate::paper::{Build, Channel, Project, Version};

const BASE_URL: &str = "https://fill.papermc.io/v3";

/// HTTP client for the Paper API.
#[derive(Debug, Clone, Default)]
pub struct Client {
    http: reqwest::Client,
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    async fn send_get_request(&self, url: &str) -> Result<Response, PaperError> {
        self.http
            .get(url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|source| PaperError::Http { source })
    }

    async fn get<T>(&self, url: &str) -> Result<T, PaperError>
    where
        T: DeserializeOwned,
    {
        let response = self.send_get_request(url).await?;

        match response.status() {
            code if code.is_success() => response
                .json()
                .await
                .map_err(|source| PaperError::Parse { source }),
            code => Err(PaperError::Status { code }),
        }
    }

    #[expect(dead_code)]
    pub async fn projects(&self) -> Result<ProjectsResponse, PaperError> {
        let url = format!("{BASE_URL}/projects");
        self.get(&url).await
    }

    #[expect(dead_code)]
    pub async fn project(&self, project: Project) -> Result<ProjectResponse, PaperError> {
        let url = format!("{BASE_URL}/projects/{project}");
        self.get(&url).await
    }

    pub async fn versions(&self, project: Project) -> Result<VersionsResponse, PaperError> {
        let url = format!("{BASE_URL}/projects/{project}/versions");
        self.get(&url).await
    }

    #[expect(dead_code)]
    pub async fn version(
        &self,
        project: Project,
        version: &Version,
    ) -> Result<Option<VersionResponse>, PaperError> {
        let url = format!("{BASE_URL}/projects/{project}/versions/{version}");
        let response = self.send_get_request(&url).await?;

        match response.status() {
            StatusCode::OK => response
                .json()
                .await
                .map(Option::Some)
                .map_err(|source| PaperError::Parse { source }),
            StatusCode::NOT_FOUND => Ok(None),
            code => Err(PaperError::Status { code }),
        }
    }

    pub async fn builds(
        &self,
        project: Project,
        version: &Version,
        channels: &[Channel],
    ) -> Result<Vec<BuildResponse>, PaperError> {
        let url = if channels.is_empty() {
            format!("{BASE_URL}/projects/{project}/versions/{version}/builds")
        } else {
            let channels_filter = channels
                .iter()
                .map(|channel| format!("channel={channel}"))
                .collect::<Vec<_>>()
                .join("&");

            format!("{BASE_URL}/projects/{project}/versions/{version}/builds?{channels_filter}")
        };

        let response = self.send_get_request(&url).await?;

        match response.status() {
            StatusCode::OK => response
                .json()
                .await
                .map_err(|source| PaperError::Parse { source }),
            StatusCode::NOT_FOUND => Err(PaperError::NotFound),
            code => Err(PaperError::Status { code }),
        }
    }

    #[expect(dead_code)]
    pub async fn build_latest(
        &self,
        project: Project,
        version: &Version,
    ) -> Result<BuildResponse, PaperError> {
        let url = format!("{BASE_URL}/projects/{project}/versions/{version}/builds/latest");
        let response = self.send_get_request(&url).await?;

        match response.status() {
            StatusCode::OK => response
                .json()
                .await
                .map_err(|source| PaperError::Parse { source }),
            StatusCode::NOT_FOUND => Err(PaperError::NotFound),
            code => Err(PaperError::Status { code }),
        }
    }

    pub async fn build(
        &self,
        project: Project,
        version: &Version,
        build: Build,
    ) -> Result<BuildResponse, PaperError> {
        let url = format!("{BASE_URL}/projects/{project}/versions/{version}/builds/{build}");
        let response = self.send_get_request(&url).await?;

        match response.status() {
            StatusCode::OK => response
                .json()
                .await
                .map_err(|source| PaperError::Parse { source }),
            StatusCode::NOT_FOUND => Err(PaperError::NotFound),
            code => Err(PaperError::Status { code }),
        }
    }

    /// Downloads the server JAR from a build.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// - Download URL is not in its expected location within [`BuildResponse`]
    pub async fn download_jar(&self, response: &BuildResponse) -> Result<Bytes, PaperError> {
        let url = response
            .downloads
            .get("server:default")
            .ok_or(PaperError::NotFound)?
            .url
            .as_str();

        self.send_get_request(url)
            .await?
            .bytes()
            .await
            .map_err(|source| PaperError::Parse { source })
    }
}
