use std::path::PathBuf;
use std::sync::LazyLock;

use bytes::Bytes;
use reqwest::Client;

const BASE_URL: &str = "https://api.modrinth.com/v2";

static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "nicdgonzalez/celestia/{version} (ndgonzalez.work@gmail.com)",
        version = env!("CARGO_PKG_VERSION")
    )
});

/// HTTP client for the Modrinth plugin repository.
#[derive(Debug, Clone)]
pub struct ModrinthClient {
    http: Client,
}

/// Response to a [`Search Projects`] request.
///
/// [`Search Projects`]: https://docs.modrinth.com/api/operations/searchprojects/
#[derive(Debug, Clone)]
pub struct Project {
    /// Unique identifier for the project.
    pub id: String,
    /// Name of the project.
    pub name: String,
}

#[derive(Debug)]
pub struct SearchRequest<'a> {
    /// Name of the plugin to search for.
    pub name: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct ListVersionsRequest<'a> {
    /// Unique identifier for the plugin.
    pub id: &'a str,
    /// Target Minecraft version.
    pub version: &'a str,
    /// Whether to include non-release builds.
    pub allow_experimental: bool,
}

/// Response to a [`List Versions`] request.
///
/// [`List Versions`]: https://docs.modrinth.com/api/operations/getprojectversions/
#[derive(Debug, Clone)]
pub struct Version {
    #[expect(dead_code)]
    pub kind: VersionKind,
    pub file: Option<File>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename = "version_type")]
#[serde(rename_all = "lowercase")]
pub enum VersionKind {
    Release,
    Beta,
    Alpha,
}

#[derive(Debug, Clone)]
pub struct File {
    pub file_name: PathBuf,
    pub url: String,
}

impl ModrinthClient {
    /// Constructs a new HTTP client for Modrinth.
    ///
    /// # Panics
    ///
    /// This method panics if we cannot initialize the Transport Layer Security (TLS) backend.
    #[must_use]
    pub fn new() -> Self {
        Self {
            http: Client::builder()
                .user_agent(&*USER_AGENT)
                .build()
                .expect("failed to initialize Transport Layer Security (TLS) backend"),
        }
    }

    /// Get projects whose names match [`SearchRequest::name`].
    ///
    /// # Errors
    ///
    /// Returns an error if we failed to send our request to Modrinth (e.g., connection timed out).
    pub async fn search_projects(
        &self,
        request: SearchRequest<'_>,
    ) -> Result<impl Iterator<Item = Project>, reqwest::Error> {
        static URL: LazyLock<String> = LazyLock::new(|| format!("{BASE_URL}/search"));

        #[derive(serde::Deserialize)]
        struct SearchProjectsResponse {
            hits: Vec<Hit>,
        }

        #[derive(serde::Deserialize)]
        struct Hit {
            title: String,
            project_id: String,
        }

        let facets = r#"[["categories:paper"],["project_type:plugin"]]"#;
        let response = self
            .http
            .get(&*URL)
            .query(&[("query", request.name), ("facets", facets)])
            .send()
            .await?;

        debug_assert!(!response.status().is_client_error());
        response.error_for_status_ref()?;

        let body = response
            .json::<SearchProjectsResponse>()
            .await
            .expect("failed to parse `Search Projects` response");

        let results = body.hits.into_iter().map(|hit| Project {
            id: hit.project_id,
            name: hit.title,
        });

        Ok(results)
    }

    /// Returns a list of plugin versions.
    ///
    /// Versions are returned in descending order (newest first).
    pub async fn list_versions(
        &self,
        request: ListVersionsRequest<'_>,
    ) -> Result<impl Iterator<Item = Version>, reqwest::Error> {
        #[derive(serde::Deserialize)]
        struct VersionResponse {
            version_type: VersionKind,
            files: Vec<_File>,
        }

        #[derive(Debug, Clone, serde::Deserialize)]
        struct _File {
            filename: PathBuf,
            url: String,
            primary: bool,
        }

        let url = format!("{BASE_URL}/project/{id}/version", id = request.id);
        let game_version = format!("[\"{}\"]", request.version);
        let query = [
            ("loaders", "paper"),
            ("game_versions", game_version.as_str()),
            ("include_changelog", "false"),
        ];
        let response = self.http.get(&url).query(&query).send().await?;

        debug_assert!(!response.status().is_client_error());
        response.error_for_status_ref()?;

        let body = response.json::<Vec<VersionResponse>>().await?;

        let versions = body
            .into_iter()
            .filter(move |v| request.allow_experimental || v.version_type == VersionKind::Release)
            .map(|v| {
                let file = v
                    .files
                    .iter()
                    .find(|file| file.primary)
                    .or_else(|| v.files.first())
                    .cloned()
                    .map(|f| File {
                        file_name: f.filename,
                        url: f.url,
                    });

                Version {
                    kind: v.version_type,
                    file,
                }
            });

        Ok(versions)
    }

    pub async fn download(&self, file: &File) -> Result<Bytes, reqwest::Error> {
        self.http.get(&file.url).send().await?.bytes().await
    }
}
