#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

use sqlx::SqlitePool;

pub mod server;

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    /// Create a new `SQLite` repository.
    #[must_use]
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
