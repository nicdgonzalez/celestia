use std::error::Error;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, fs, io};

/// Minecraft server configuration options.
///
/// Maps to a Minecraft server's `server.properties` file.
pub struct Properties {
    path: PathBuf,
}

impl Properties {
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>, GetValueError>
    where
        T: FromStr,
    {
        let prefix = format!("{key}=");
        let contents = fs::read_to_string(&self.path)?;

        if let Some(value_str) = contents
            .lines()
            .find(|line| line.starts_with(&prefix))
            .and_then(|line| line.split_once('='))
            .map(|(_, v)| v)
        {
            value_str
                .parse()
                .map(Option::Some)
                .map_err(|_| GetValueError::Parse)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum GetValueError {
    Io { source: io::Error },
    Parse,
}

impl Error for GetValueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Io { ref source } => Some(source),
            Self::Parse => None,
        }
    }
}

impl fmt::Display for GetValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io { source: _ } => "I/O error occurred".fmt(f),
            Self::Parse => "failed to parse to target type".fmt(f),
        }
    }
}

impl From<io::Error> for GetValueError {
    fn from(value: io::Error) -> Self {
        Self::Io { source: value }
    }
}
