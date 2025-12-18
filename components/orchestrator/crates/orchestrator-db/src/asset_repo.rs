use crate::{asset_from_row, format_timestamp};
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_core::domain::Asset;
use orchestrator_core::ports::AssetRepository;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct SqliteAssetRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteAssetRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl AssetRepository for SqliteAssetRepository {
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url,
                        metadata, created_at, updated_at
                 FROM assets WHERE project_id = ?1 ORDER BY name ASC",
            )?;
            let rows = stmt.query_map(params![project_id], asset_from_row)?;
            rows.collect::<Result<Vec<_>, _>>()
                .context("failed to collect assets")
        })
        .await?
    }

    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url,
                        metadata, created_at, updated_at
                 FROM assets WHERE scene_id = ?1 ORDER BY name ASC",
            )?;
            let rows = stmt.query_map(params![scene_id], asset_from_row)?;
            rows.collect::<Result<Vec<_>, _>>()
                .context("failed to collect assets")
        })
        .await?
    }

    async fn get_asset(&self, id: i64) -> Result<Option<Asset>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url,
                        metadata, created_at, updated_at
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

    async fn create_asset(&self, mut asset: Asset) -> Result<Asset> {
        let conn = self.conn.clone();
        let asset_clone = asset.clone();
        let id = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created_str = format_timestamp(asset_clone.created_at)?;
            let updated_str = format_timestamp(asset_clone.updated_at)?;
            conn.execute(
                "INSERT INTO assets (project_id, scene_id, name, asset_type, file_path,
                                     url, metadata, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    asset_clone.project_id,
                    asset_clone.scene_id,
                    asset_clone.name,
                    asset_clone.asset_type,
                    asset_clone.file_path,
                    asset_clone.url,
                    asset_clone.metadata,
                    created_str,
                    updated_str
                ],
            )?;
            Ok::<i64, anyhow::Error>(conn.last_insert_rowid())
        })
        .await??;
        asset.id = Some(id);
        Ok(asset)
    }

    async fn update_asset(&self, asset: Asset) -> Result<Asset> {
        let conn = self.conn.clone();
        let asset_clone = asset.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let updated_str = format_timestamp(OffsetDateTime::now_utc())?;
            conn.execute(
                "UPDATE assets SET name = ?1, asset_type = ?2, file_path = ?3,
                                   url = ?4, metadata = ?5, updated_at = ?6
                 WHERE id = ?7",
                params![
                    asset_clone.name,
                    asset_clone.asset_type,
                    asset_clone.file_path,
                    asset_clone.url,
                    asset_clone.metadata,
                    updated_str,
                    asset_clone.id
                ],
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
