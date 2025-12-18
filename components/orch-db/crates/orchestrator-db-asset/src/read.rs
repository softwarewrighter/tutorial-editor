//! Asset read operations

use crate::repo::{asset_from_row, SqliteAssetRepository};
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_domain::Asset;
use orchestrator_ports_asset::AssetReadOps;
use rusqlite::params;

#[async_trait]
impl AssetReadOps for SqliteAssetRepository {
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url, metadata,
                        created_at, updated_at
                 FROM assets WHERE project_id = ?1 ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map(params![project_id], asset_from_row)?;
            rows.collect::<Result<Vec<_>, _>>().context("failed to list assets")
        })
        .await?
    }

    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url, metadata,
                        created_at, updated_at
                 FROM assets WHERE scene_id = ?1 ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map(params![scene_id], asset_from_row)?;
            rows.collect::<Result<Vec<_>, _>>().context("failed to list scene assets")
        })
        .await?
    }

    async fn get_asset(&self, id: i64) -> Result<Option<Asset>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url, metadata,
                        created_at, updated_at
                 FROM assets WHERE id = ?1",
            )?;
            let mut rows = stmt.query(params![id])?;
            match rows.next()? {
                Some(row) => Ok(Some(asset_from_row(row)?)),
                None => Ok(None),
            }
        })
        .await?
    }
}
