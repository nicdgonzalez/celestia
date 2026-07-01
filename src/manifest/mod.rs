mod model;
mod plugin;
mod server;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, io};

use crate::manifest::model::Model;
use crate::paper::{Build, Version};
pub use plugin::{Name, Plugin, Source};

#[derive(Debug, Clone, Default)]
pub struct Manifest {
    inner: Model,
}

impl Manifest {
    pub fn open<P>(path: P) -> Result<Self, ManifestError>
    where
        P: AsRef<Path>,
    {
        File::open(path)
            .map_err(|source| ManifestError::Io { source })
            .and_then(Self::from_reader)
    }

    pub fn from_reader<R>(mut reader: R) -> Result<Self, ManifestError>
    where
        R: Read,
    {
        let mut buffer = String::new();
        reader
            .read_to_string(&mut buffer)
            .map_err(|source| ManifestError::Io { source })?;

        Self::from_str(&buffer)
    }

    pub fn save<P>(&self, path: P) -> Result<(), ManifestError>
    where
        P: AsRef<Path>,
    {
        let contents = toml::to_string_pretty(&self.inner)
            .map_err(|source| ManifestError::Write { source })?;

        fs::write(path, contents).map_err(|source| ManifestError::Io { source })
    }

    pub fn version(&self) -> &Version {
        &self.inner.server.version
    }

    pub fn set_version(&mut self, version: Version) -> &mut Self {
        self.inner.server.version = version;
        self
    }

    pub fn build(&self) -> Build {
        self.inner.server.build
    }

    pub fn set_build(&mut self, build: Build) -> &mut Self {
        self.inner.server.build = build;
        self
    }

    pub fn plugins(&self) -> &HashMap<Name, Plugin> {
        &self.inner.plugins
    }

    pub fn add_plugin(&mut self, name: Name, plugin: Plugin) -> Option<Plugin> {
        self.inner.plugins.insert(name, plugin)
    }

    pub fn remove_plugin(&mut self, name: &Name) -> Option<(Name, Plugin)> {
        self.inner.plugins.remove_entry(name)
    }
}

impl FromStr for Manifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
            .map(|inner| Self { inner })
            .map_err(|source| ManifestError::Read { source })
    }
}

#[derive(Debug)]
pub enum ManifestError {
    /// I/O-related error occurred
    Io { source: io::Error },
    /// Failed to deserialize data
    Read { source: toml::de::Error },
    /// Failed to serialize data
    Write { source: toml::ser::Error },
}

impl Error for ManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Io { ref source } => Some(source),
            Self::Read { ref source } => Some(source),
            Self::Write { ref source } => Some(source),
        }
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io { source: _ } => "I/O error occurred".fmt(f),
            Self::Read { source: _ } => "Failed to deserialize data".fmt(f),
            Self::Write { source: _ } => "Failed to serialize data".fmt(f),
        }
    }
}
