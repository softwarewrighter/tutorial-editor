//! SQLite scene repository

use crate::mapping::scene_from_row;
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_db_core::{format_timestamp, Connection};
use orchestrator_domain::Scene;
use orchestrator_ports_repo::SceneRepository;
use rusqlite::params;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct SqliteSceneRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteSceneRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl SceneRepository for SqliteSceneRepository {
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, title, description, sort_order, script_text,
                        created_at, updated_at
                 FROM scenes WHERE project_id = ?1 ORDER BY sort_order ASC",
            )?;
            let rows = stmt.query_map(params![project_id], scene_from_row)?;
            rows.collect::<Result<Vec<_>, _>>().context("failed to list scenes")
        }).await?
    }

    async fn get_scene(&self, id: i64) -> Result<Option<Scene>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, title, description, sort_order, script_text,
                        created_at, updated_at
                 FROM scenes WHERE id = ?1",
            )?;
            let mut rows = stmt.query(params![id])?;
            match rows.next()? {
                Some(row) => Ok(Some(scene_from_row(row)?)),
                None => Ok(None),
            }
        }).await?
    }

    async fn create_scene(&self, mut scene: Scene) -> Result<Scene> {
        let conn = self.conn.clone();
        let s = scene.clone();
        let id = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created = format_timestamp(s.created_at)?;
            let updated = format_timestamp(s.updated_at)?;
            conn.execute(
                "INSERT INTO scenes (project_id, title, description, sort_order,
                                     script_text, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![s.project_id, s.title, s.description, s.sort_order, s.script_text, created, updated],
            )?;
            Ok::<i64, anyhow::Error>(conn.last_insert_rowid())
        }).await??;
        scene.id = Some(id);
        Ok(scene)
    }

    async fn update_scene(&self, scene: Scene) -> Result<Scene> {
        let conn = self.conn.clone();
        let s = scene.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let updated = format_timestamp(OffsetDateTime::now_utc())?;
            conn.execute(
                "UPDATE scenes SET title = ?1, description = ?2, sort_order = ?3,
                                   script_text = ?4, updated_at = ?5
                 WHERE id = ?6",
                params![s.title, s.description, s.sort_order, s.script_text, updated, s.id],
            )?;
            Ok::<(), anyhow::Error>(())
        }).await??;
        Ok(scene)
    }

    async fn delete_scene(&self, id: i64) -> Result<()> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute("DELETE FROM scenes WHERE id = ?1", params![id])?;
            Ok::<(), anyhow::Error>(())
        }).await??;
        Ok(())
    }
}
