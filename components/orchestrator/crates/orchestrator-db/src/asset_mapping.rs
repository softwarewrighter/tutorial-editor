use crate::timestamps;
use orchestrator_core::domain::Asset;

pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Asset> {
    let created_at = timestamps::parse(row.get::<_, String>(8)?);
    let updated_at = timestamps::parse(row.get::<_, String>(9)?);
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
