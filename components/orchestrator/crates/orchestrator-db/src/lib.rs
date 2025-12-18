mod asset_repo;
mod project_repo;
mod scene_repo;
mod schema;

pub use asset_repo::SqliteAssetRepository;
pub use project_repo::SqliteProjectRepository;
pub use scene_repo::SqliteSceneRepository;
pub use schema::init_db;

use anyhow::{Context, Result};
use orchestrator_core::domain::{Asset, Scene};
use time::OffsetDateTime;

pub(crate) fn parse_timestamp(s: String) -> OffsetDateTime {
    OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
}

pub(crate) fn format_timestamp(dt: OffsetDateTime) -> Result<String> {
    dt.format(&time::format_description::well_known::Rfc3339)
        .context("failed to format timestamp")
}

pub(crate) fn scene_from_row(row: &rusqlite::Row) -> rusqlite::Result<Scene> {
    let created_at = parse_timestamp(row.get::<_, String>(6)?);
    let updated_at = parse_timestamp(row.get::<_, String>(7)?);
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

pub(crate) fn asset_from_row(row: &rusqlite::Row) -> rusqlite::Result<Asset> {
    let created_at = parse_timestamp(row.get::<_, String>(8)?);
    let updated_at = parse_timestamp(row.get::<_, String>(9)?);
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
