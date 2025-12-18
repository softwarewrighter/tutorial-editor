//! SQLite asset repository struct and mapping

use orchestrator_db_core::{parse_timestamp, Connection};
use orchestrator_domain::Asset;
use rusqlite::Row;
use std::sync::{Arc, Mutex};

/// SQLite-backed asset repository
#[derive(Clone)]
pub struct SqliteAssetRepository {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl SqliteAssetRepository {
    /// Create a new repository with the given connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

/// Map a database row to an Asset struct
pub fn asset_from_row(row: &Row) -> rusqlite::Result<Asset> {
    let created_at = parse_timestamp(&row.get::<_, String>(8)?);
    let updated_at = parse_timestamp(&row.get::<_, String>(9)?);
    Ok(Asset {
        id: row.get(0)?,
        project_id: row.get(1)?,
        scene_id: row.get(2)?,
        name: row.get(3)?,
        asset_type: row.get(4)?,
        file_path: row.get(5)?,
        url: row.get(6)?,
        metadata: row.get(7)?,
        created_at,
        updated_at,
    })
}
