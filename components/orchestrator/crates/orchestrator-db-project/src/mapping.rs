//! Project row mapping

use orchestrator_db_core::parse_timestamp;
use orchestrator_domain::Project;
use rusqlite::Row;

/// Map a database row to a Project struct
pub fn project_from_row(row: &Row) -> rusqlite::Result<Project> {
    let created_at = parse_timestamp(&row.get::<_, String>(5)?);
    let updated_at = parse_timestamp(&row.get::<_, String>(6)?);
    Ok(Project {
        id: row.get(0)?,
        slug: row.get(1)?,
        title: row.get(2)?,
        subtitle: row.get(3)?,
        description: row.get(4)?,
        created_at,
        updated_at,
    })
}
