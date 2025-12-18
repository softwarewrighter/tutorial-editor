//! Asset write operations

use crate::repo::SqliteAssetRepository;
use anyhow::Result;
use async_trait::async_trait;
use orchestrator_db_core::format_timestamp;
use orchestrator_domain::Asset;
use orchestrator_ports_asset::AssetWriteOps;
use rusqlite::params;
use time::OffsetDateTime;

const INSERT_SQL: &str = "INSERT INTO assets (project_id, scene_id, name, asset_type, file_path, url, metadata, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

#[async_trait]
impl AssetWriteOps for SqliteAssetRepository {
    async fn create_asset(&self, mut asset: Asset) -> Result<Asset> {
        let conn = self.conn.clone();
        let a = asset.clone();
        let id = tokio::task::spawn_blocking(move || {
            let c = conn.lock().unwrap();
            let (cr, up) = (format_timestamp(a.created_at)?, format_timestamp(a.updated_at)?);
            c.execute(INSERT_SQL, params![a.project_id, a.scene_id, a.name, a.asset_type, a.file_path, a.url, a.metadata, cr, up])?;
            Ok::<i64, anyhow::Error>(c.last_insert_rowid())
        }).await??;
        asset.id = Some(id);
        Ok(asset)
    }

    async fn update_asset(&self, asset: Asset) -> Result<Asset> {
        let conn = self.conn.clone();
        let a = asset.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let updated = format_timestamp(OffsetDateTime::now_utc())?;
            conn.execute(
                "UPDATE assets SET name = ?1, asset_type = ?2, file_path = ?3, url = ?4,
                                   metadata = ?5, updated_at = ?6
                 WHERE id = ?7",
                params![a.name, a.asset_type, a.file_path, a.url, a.metadata, updated, a.id],
            )?;
            Ok::<(), anyhow::Error>(())
        })
        .await??;
        Ok(asset)
    }

    async fn delete_asset(&self, id: i64) -> Result<()> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute("DELETE FROM assets WHERE id = ?1", params![id])?;
            Ok::<(), anyhow::Error>(())
        })
        .await??;
        Ok(())
    }
}
