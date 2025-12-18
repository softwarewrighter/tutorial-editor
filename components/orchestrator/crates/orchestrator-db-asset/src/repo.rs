//! SQLite asset repository

use crate::mapping::asset_from_row;
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_db_core::{format_timestamp, Connection};
use orchestrator_domain::Asset;
use orchestrator_ports_repo::AssetRepository;
use rusqlite::params;
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
                "SELECT id, project_id, scene_id, name, asset_type, file_path, url, metadata,
                        created_at, updated_at
                 FROM assets WHERE project_id = ?1 ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map(params![project_id], asset_from_row)?;
            rows.collect::<Result<Vec<_>, _>>().context("failed to list assets")
        }).await?
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
        }).await?
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
        }).await?
    }

    async fn create_asset(&self, mut asset: Asset) -> Result<Asset> {
        let conn = self.conn.clone();
        let a = asset.clone();
        let id = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created = format_timestamp(a.created_at)?;
            let updated = format_timestamp(a.updated_at)?;
            conn.execute(
                "INSERT INTO assets (project_id, scene_id, name, asset_type, file_path, url,
                                     metadata, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![a.project_id, a.scene_id, a.name, a.asset_type, a.file_path, a.url,
                        a.metadata, created, updated],
            )?;
            Ok::<i64, anyhow::Error>(conn.last_insert_rowid())
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
        }).await??;
        Ok(asset)
    }

    async fn delete_asset(&self, id: i64) -> Result<()> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute("DELETE FROM assets WHERE id = ?1", params![id])?;
            Ok::<(), anyhow::Error>(())
        }).await??;
        Ok(())
    }
}
