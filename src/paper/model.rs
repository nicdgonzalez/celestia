use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<ProjectResponse>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectResponse {
    pub project: Project,
    pub version: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionsResponse {
    pub versions: Vec<VersionResponse>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionResponse {
    pub builds: Vec<u32>,
    pub version: Version,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Version {
    pub id: super::Version,
    pub java: Java,
    pub support: Support,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Java {
    pub flags: JavaFlags,
    pub version: JavaVersion,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JavaFlags {
    pub recommended: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JavaVersion {
    pub minimum: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Support {
    pub end: Option<NaiveDate>,
    pub status: SupportStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SupportStatus {
    Supported,
    Deprecated,
    Unsupported,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildResponse {
    pub channel: Channel,
    pub commits: Vec<Commit>,
    pub downloads: HashMap<String, Download>,
    pub id: super::Build,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Channel {
    Alpha,
    Beta,
    Stable,
    Recommended,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Commit {
    pub message: String,
    pub sha: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Download {
    pub checksums: Checksums,
    pub name: String,
    pub size: u32,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Checksums {
    pub sha256: String,
}
