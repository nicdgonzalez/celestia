//! Represents the `server` subdirectory.

mod jar;
mod properties;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fmt, fs, io};

use tracing::error;

pub use jar::{GetServerInfoError, Jar, ServerInfo};
pub use properties::{GetValueError, Properties};

/// A Minecraft server JAR and all of its configuration files.
#[derive(Debug, Clone)]
pub struct Server {
    path: PathBuf,
}

impl Server {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn try_exists(&self) -> io::Result<bool> {
        self.path.try_exists()
    }

    pub fn jar(&self) -> Jar {
        // TODO: Manifest will have the option to specify a JAR name,
        // so this needs to be able to reference that path.
        Jar::new(self.path.join("server.jar"))
    }

    pub fn properties(&self) -> Properties {
        Properties::new(self.path.join("server.properties"))
    }

    pub fn eula(&self) -> PathBuf {
        self.path.join("eula.txt")
    }

    pub fn plugins(&self) -> PathBuf {
        self.path.join("plugins")
    }

    pub fn logs(&self) -> PathBuf {
        self.path.join("logs")
    }

    pub fn latest_log(&self) -> PathBuf {
        let mut path = self.logs();
        path.push("latest.log");
        path
    }

    pub fn start_sh(&self) -> PathBuf {
        self.path.join("start.sh")
    }

    pub fn has_initial_files(&self) -> io::Result<bool> {
        self.properties().path().try_exists()
    }

    /// Runs the server once to generate the initial configuration files.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// - Path to server JAR contains invalid unicode
    /// - Failed to determine whether server JAR exists
    /// - Server JAR confirmed to not exist
    /// - Failed to spawn child process
    /// - Child process returned a non-zero exit code
    /// - Child process terminated via signal
    pub fn generate_initial_files(&self) -> Result<(), InitialFilesError> {
        let jar = self.jar();
        let path = jar.path().to_str().ok_or(InitialFilesError::InvalidPath)?;

        if !jar
            .path()
            .try_exists()
            .map_err(|source| InitialFilesError::Io { source })?
        {
            return Err(InitialFilesError::NotExists);
        }

        let output = Command::new("java")
            .args(["-jar", path, "--initSettings"])
            .current_dir(&self.path)
            .output()
            .map_err(|source| InitialFilesError::CommandExecutionFailed { source })?;

        match output.status.code() {
            Some(0) => Ok(()),
            Some(code) => {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                error!("failed to generate initial files:\n{}", error);
                Err(InitialFilesError::Status { code })
            }
            None => Err(InitialFilesError::Terminated),
        }
    }

    /// Accepts the Minecraft EULA.
    ///
    /// # Errors
    ///
    /// This function returns an error if an I/O error occurs while trying to write to `eula.txt`.
    pub fn accept_eula(&self) -> io::Result<()> {
        let eula = self.eula();
        let contents = fs::read_to_string(&eula)?.replace("eula=false", "eula=true");
        fs::write(&eula, contents)
    }
}

/// Describes an error that occurred while generating the initial server files.
#[derive(Debug)]
pub enum InitialFilesError {
    /// I/O error occurred
    Io { source: io::Error },
    /// Path to server JAR contains invalid Unicode
    InvalidPath,
    /// Server JAR not found
    NotExists,
    /// Failed to execute a child process
    CommandExecutionFailed { source: io::Error },
    /// Child process returned a non-zero exit code
    Status { code: i32 },
    /// Child process was terminated via signal
    Terminated,
}

impl Error for InitialFilesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::InvalidPath | Self::NotExists | Self::Status { code: _ } | Self::Terminated => {
                None
            }
            Self::Io { ref source } | Self::CommandExecutionFailed { ref source } => Some(source),
        }
    }
}

impl fmt::Display for InitialFilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io { source: _ } => "I/O error occurred".fmt(f),
            Self::InvalidPath => "path to server JAR contained invalid unicode".fmt(f),
            Self::NotExists => "server JAR not found".fmt(f),
            Self::CommandExecutionFailed { source: _ } => "failed to execute command".fmt(f),
            Self::Status { code } => write!(f, "process exited with non-zero exit code: {code}"),
            Self::Terminated => "process terminated via signal".fmt(f),
        }
    }
}
