use std::fmt;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use crate::manifest::model::TomlPluginSource;
use crate::registry::PluginRegistry;

#[derive(Debug, Clone)]
pub struct Plugin {
    name: PluginName,
    source: PluginSource,
    /// Custom file name to use when saving the plugin.
    file: PathBuf,
}

impl Plugin {
    /// Construct a new plugin.
    #[must_use]
    pub const fn new(name: PluginName, source: PluginSource, file: PathBuf) -> Self {
        Self { name, source, file }
    }

    /// Case-insensitive name of the plugin.
    #[must_use]
    pub const fn name(&self) -> &PluginName {
        &self.name
    }

    /// Describes where to install the plugin from.
    #[must_use]
    pub const fn source(&self) -> &PluginSource {
        &self.source
    }

    /// Name of file to save the plugin to.
    #[must_use]
    pub fn file(&self) -> &Path {
        &self.file
    }
}

/// Describes where to install the plugin from.
#[derive(Debug, Clone)]
pub enum PluginSource {
    /// Install from an online plugin repository.
    Registry {
        /// Where the plugin will be downloaded from.
        registry: PluginRegistry,
        /// Unique identifier for the plugin.
        id: String,
        /// Target plugin version to install.
        version: String,
    },
    /// Install from an external server.
    Url {
        /// URL to the target plugin JAR.
        url: String,
    },
    /// Install from a path on the system.
    Local {
        /// Path to the target plugin JAR.
        path: PathBuf,
    },
}

impl From<TomlPluginSource> for PluginSource {
    fn from(value: TomlPluginSource) -> Self {
        match value {
            TomlPluginSource::Registry {
                registry,
                id,
                version,
            } => Self::Registry {
                registry: registry.into(),
                id,
                version,
            },
            TomlPluginSource::Url { url } => Self::Url { url },
            TomlPluginSource::Local { path } => Self::Local { path },
        }
    }
}

/// Case-insensitive plugin name.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PluginName(String);

impl PluginName {
    /// Reference to the raw original inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Hash for PluginName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}

impl PartialEq for PluginName {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for PluginName {}

impl From<String> for PluginName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for PluginName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
