use crate::timestamps;
use orchestrator_core::domain::Scene;

pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Scene> {
    let created_at = timestamps::parse(row.get::<_, String>(6)?);
    let updated_at = timestamps::parse(row.get::<_, String>(7)?);
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
