use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::manifest::model::TomlPluginRegistry;

/// Identifies the remote registry used to resolve and download a plugin.
///
/// This type displays user-friendly names while serializing and parsing
/// canonical lowercase identifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum PluginRegistry {
    /// The Modrinth plugin registry.
    #[default]
    Modrinth,
}

impl From<TomlPluginRegistry> for PluginRegistry {
    fn from(value: TomlPluginRegistry) -> Self {
        match value {
            TomlPluginRegistry::Modrinth => Self::Modrinth,
        }
    }
}

impl fmt::Display for PluginRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Modrinth => "Modrinth".fmt(f),
        }
    }
}

impl serde::Serialize for PluginRegistry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Modrinth => serializer.serialize_str("modrinth"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for PluginRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for PluginRegistry {
    type Err = InvalidRegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variant = match s.to_ascii_lowercase().as_str() {
            "modrinth" => Self::Modrinth,
            _ => return Err(InvalidRegistryError),
        };

        Ok(variant)
    }
}

/// Error returned when parsing an unregistered plugin registry.
#[derive(Debug)]
pub struct InvalidRegistryError;

impl Error for InvalidRegistryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for InvalidRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "invalid registry name".fmt(f)
    }
}
