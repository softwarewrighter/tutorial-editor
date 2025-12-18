//! SQLite project repository struct

use anyhow::{Context, Result};
use orchestrator_db_core::{init_db, Connection};
use std::sync::{Arc, Mutex};

/// SQLite-backed project repository
#[derive(Clone)]
pub struct SqliteProjectRepository {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl SqliteProjectRepository {
    /// Create a new repository with the given database path
    pub fn new(path: &str) -> Result<Self> {
        let conn =
            Connection::open(path).with_context(|| format!("failed to open sqlite db at {path}"))?;
        init_db(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get a clone of the database connection
    pub fn connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}
