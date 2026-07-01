use std::error::Error;
use std::fmt;
use std::io::{self, BufRead as _};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::paper::{Build, Version};

/// Server JAR file from Paper.
#[derive(Debug, Clone)]
pub struct Jar {
    path: PathBuf,
}

pub struct ServerInfo {
    pub version: Version,
    pub build: Build,
}

impl Jar {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn get_server_info(&self) -> Result<ServerInfo, GetServerInfoError> {
        if !self
            .path
            .try_exists()
            .map_err(|source| GetServerInfoError::Io { source })?
        {
            return Err(GetServerInfoError::NotExists);
        }

        let path = self.path.to_str().ok_or(GetServerInfoError::InvalidPath)?;
        let output = Command::new("java")
            .args(["-jar", path, "--version"])
            .output()
            .map_err(|source| GetServerInfoError::CommandExecutionFailed { source })?;

        let line = output
            .stdout
            .lines()
            .last()
            .ok_or(GetServerInfoError::MissingResponse)?
            .map_err(|source| GetServerInfoError::LineReadFailed { source })?;

        let mut parts = line.split('-');
        let version = parts
            .next()
            .ok_or(GetServerInfoError::MissingPart)?
            .parse::<Version>()
            .or(Err(GetServerInfoError::UnexpectedFormat))?;
        let build = parts
            .next()
            .ok_or(GetServerInfoError::MissingPart)?
            .parse::<Build>()
            .or(Err(GetServerInfoError::UnexpectedFormat))?;

        Ok(ServerInfo { version, build })
    }
}

impl AsRef<Path> for Jar {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug)]
pub enum GetServerInfoError {
    /// I/O-related error occurred
    Io {
        source: io::Error,
    },
    /// JAR does not exist at [`Self::as_path`]
    NotExists,
    /// Path to server JAR contains invalid Unicode
    InvalidPath,
    /// Failed to execute a child process
    CommandExecutionFailed {
        source: io::Error,
    },
    /// Server produced no output
    MissingResponse,
    /// Failed to read a line of output
    LineReadFailed {
        source: io::Error,
    },
    /// Version output did not match the expected format
    UnexpectedFormat,
    MissingPart,
}

impl Error for GetServerInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::InvalidPath
            | Self::MissingResponse
            | Self::UnexpectedFormat
            | Self::NotExists
            | Self::MissingPart => None,
            Self::Io { ref source }
            | Self::CommandExecutionFailed { ref source }
            | Self::LineReadFailed { ref source } => Some(source),
        }
    }
}

impl fmt::Display for GetServerInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io { source: _ } => "I/O error occurred".fmt(f),
            Self::NotExists => "server JAR file does not exist".fmt(f),
            Self::InvalidPath => "path to server JAR contained invalid unicode".fmt(f),
            Self::CommandExecutionFailed { source: _ } => "failed to execute command".fmt(f),
            Self::MissingResponse => "command returned no output".fmt(f),
            Self::LineReadFailed { source: _ } => "failed to read line".fmt(f),
            Self::UnexpectedFormat => "unexpected format for version output".fmt(f),
            Self::MissingPart => "missing part in JAR output from `version-build-hash`".fmt(f),
        }
    }
}
