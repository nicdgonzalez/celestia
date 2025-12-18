use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use celestia_core::server::{Server, Repository};
use snowflaked::Generator;

use crate::SqliteRepository;

/// Represents a raw server entry in the database.
#[derive(sqlx::FromRow)]
struct ServerRow {
    id: String,
    name: String,
    path: String,
}

impl From<ServerRow> for Server {
    fn from(value: ServerRow) -> Self {
        Self::new(value.id, value.name, PathBuf::from(value.path))
    }
}

impl Repository for SqliteRepository {
    type InsertError = InsertServerError;
    type GetByIdError = GetByIdError;

    async fn insert(&self, name: String, path: PathBuf) -> Result<Server, Self::InsertError> {
        let mut generator = Generator::new(0);
        let id = generator.generate::<u64>().to_string();

        sqlx::query("INSERT INTO server (id, name, path) VALUES ($1, $2, $3)")
            .bind(&id)
            .bind(&name)
            .bind(path.to_str().expect("expected path to be valid unicode"))
            .execute(&self.pool)
            .await
            .map_err(|err| InsertServerError { source: err })?;

        Ok(Server::new(id, name, path))
    }

    async fn get_by_id(&self, id: String) -> Result<Server, Self::GetByIdError> {
        match sqlx::query_as::<_, ServerRow>("SELECT * FROM server WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => Ok(row.into()),
            Err(sqlx::Error::RowNotFound) => Err(GetByIdError::NotFound { id }),
            Err(source) => Err(GetByIdError::FailedToExecute { id, source }),
        }
    }
}

/// Describes an error that occurred while adding a server to the repository.
#[derive(Debug)]
pub struct InsertServerError {
    /// The underlying error that caused the failure.
    source: sqlx::Error,
}

impl Error for InsertServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for InsertServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "failed to execute query".fmt(f)
    }
}

/// Describes an error that occurred while getting a server from the repository.
#[derive(Debug)]
pub enum GetByIdError {
    /// Failed to execute query in the database.
    FailedToExecute {
        /// The passed-in ID.
        id: String,
        /// The underlying error that caused the failure.
        source: sqlx::Error,
    },
    /// No entries in the database matched the given `id`.
    NotFound {
        /// The passed-in ID.
        id: String,
    },
}

impl Error for GetByIdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::FailedToExecute { ref source, .. } => Some(source),
            Self::NotFound { .. } => None,
        }
    }
}

impl fmt::Display for GetByIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FailedToExecute { .. } => "failed to execute query".fmt(f),
            Self::NotFound { ref id } => write!(f, "server with id {id} not found"),
        }
    }
}
