//! Scene row mapping

use orchestrator_db_core::parse_timestamp;
use orchestrator_domain::Scene;
use rusqlite::Row;

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
