use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PaperError {
    Http { source: reqwest::Error },
    Status { code: reqwest::StatusCode },
    Parse { source: reqwest::Error },
    NotFound,
}

impl Error for PaperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Http { ref source } | Self::Parse { ref source } => Some(source),
            Self::Status { .. } | Self::NotFound => None,
        }
    }
}

impl fmt::Display for PaperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Http { source: _ } => "failed to send request".fmt(f),
            Self::Status { code } => write!(f, "unexpected status code: {code}"),
            Self::Parse { source: _ } => "failed to parse response".fmt(f),
            Self::NotFound => "resource not found".fmt(f),
        }
    }
}
