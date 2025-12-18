//! SQLite scene repository struct and mapping

use orchestrator_db_core::{parse_timestamp, Connection};
use orchestrator_domain::Scene;
use rusqlite::Row;
use std::sync::{Arc, Mutex};

/// SQLite-backed scene repository
#[derive(Clone)]
pub struct SqliteSceneRepository {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl SqliteSceneRepository {
    /// Create a new repository with the given connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

/// Map a database row to a Scene struct
pub fn scene_from_row(row: &Row) -> rusqlite::Result<Scene> {
    let created_at = parse_timestamp(&row.get::<_, String>(6)?);
    let updated_at = parse_timestamp(&row.get::<_, String>(7)?);
    Ok(Scene {
        id: row.get(0)?,
        project_id: row.get(1)?,
        title: row.get(2)?,
        description: row.get(3)?,
        sort_order: row.get(4)?,
        script_text: row.get(5)?,
        created_at,
        updated_at,
    })
}
