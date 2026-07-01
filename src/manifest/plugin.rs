use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;

use crate::plugins::Registry;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Plugin {
    #[serde(flatten)]
    pub source: Source,

    /// Save-as filename.
    pub file: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Source {
    Registry {
        /// Where to download the plugin from
        #[serde(default)]
        registry: Registry,
        /// Unique identifier for the plugin
        id: String,
        /// Target plugin version to install
        version: String,
    },

    Url {
        /// URL to the target plugin JAR
        url: String,
    },

    Local {
        /// Path to the local plugin JAR
        path: PathBuf,
    },
}

/// Case-insensitive plugin name.
#[derive(Debug, Clone, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Name(String);

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Hash for Name {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
