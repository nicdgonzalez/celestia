use std::collections::HashMap;
use std::path::PathBuf;

use crate::plugin::PluginName;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TomlManifest {
    pub server: TomlServer,
    pub plugins: HashMap<PluginName, TomlPlugin>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlServer {
    pub version: String,
    pub build: u32,
}

impl Default for TomlServer {
    fn default() -> Self {
        Self {
            version: "26.2".to_owned(),
            build: 60,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlDashboard {
    pub port: u16,
}

impl Default for TomlDashboard {
    fn default() -> Self {
        Self { port: 1140 }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlPlugin {
    #[serde(flatten)]
    pub source: TomlPluginSource,
    pub file: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TomlPluginSource {
    Registry {
        #[serde(default)]
        registry: TomlPluginRegistry,
        id: String,
        version: String,
    },
    Url {
        url: String,
    },
    Local {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TomlPluginRegistry {
    #[default]
    Modrinth,
}
